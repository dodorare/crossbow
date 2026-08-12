use crossbow_android::{
    error::*,
    jni::{objects::JObjectArray, objects::JString, JavaVM},
    plugin::*,
};
use std::sync::Arc;

// TODO: Add better handling errors:
// https://developers.google.com/android/reference/com/google/android/gms/common/api/CommonStatusCodes

pub struct PlayGamesServicesPlugin {
    singleton: Arc<JniSingleton>,
    vm: Arc<JavaVM>,
}

impl CrossbowPlugin for PlayGamesServicesPlugin {
    fn from_java_vm(vm: Arc<JavaVM>) -> Result<Self>
    where
        Self: Sized,
    {
        let singleton = get_jni_singleton(Self::get_plugin_name()).ok_or_else(|| {
            AndroidError::SingletonNotRegistered(Self::get_plugin_name().to_owned())
        })?;
        Ok(Self { singleton, vm })
    }

    fn get_plugin_name() -> &'static str {
        "CrossbowPlayGamesServices"
    }

    fn get_receiver(&self) -> &Receiver<Signal> {
        self.singleton.get_receiver()
    }
}

impl PlayGamesServicesPlugin {
    fn call_void(&self, method: &str) -> Result<()> {
        self.vm.attach_current_thread(|env| {
            self.singleton.call_method(env, method, &[])?;
            Ok(())
        })
    }

    fn call_bool(&self, method: &str) -> Result<bool> {
        self.vm.attach_current_thread(|env| {
            let value = self.singleton.call_method(env, method, &[])?;
            Ok(value.z()?)
        })
    }

    fn call_with_bool(&self, method: &str, value: bool) -> Result<()> {
        self.vm.attach_current_thread(|env| {
            self.singleton.call_method(env, method, &[value.into()])?;
            Ok(())
        })
    }

    fn call_with_string<S>(&self, method: &str, value: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let value = JString::from_str(env, value)?;
            self.singleton
                .call_method(env, method, &[(&value).into()])?;
            Ok(())
        })
    }

    fn call_with_string_int<S>(&self, method: &str, value: S, number: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let value = JString::from_str(env, value)?;
            self.singleton
                .call_method(env, method, &[(&value).into(), number.into()])?;
            Ok(())
        })
    }

    pub fn init(&self, enable_popups: bool) -> Result<()> {
        self.call_with_bool("init", enable_popups)
    }

    pub fn init_with_saved_games<S>(&self, enable_popups: bool, save_game_name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let save_game_name = JString::from_str(env, save_game_name)?;
            self.singleton.call_method(
                env,
                "initWithSavedGames",
                &[enable_popups.into(), (&save_game_name).into()],
            )?;
            Ok(())
        })
    }

    pub fn sign_in(&self) -> Result<()> {
        self.call_void("signIn")
    }

    pub fn sign_out(&self) -> Result<()> {
        self.call_void("signOut")
    }

    pub fn is_signed_in(&self) -> Result<bool> {
        self.call_bool("isSignedIn")
    }

    pub fn show_achievements(&self) -> Result<()> {
        self.call_void("showAchievements")
    }

    pub fn unlock_achievement<S>(&self, achievement_name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string("unlockAchievement", achievement_name)
    }

    pub fn reveal_achievement<S>(&self, achievement_name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string("revealAchievement", achievement_name)
    }

    pub fn increment_achievement<S>(&self, achievement_name: S, step: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string_int("incrementAchievement", achievement_name, step)
    }

    pub fn set_achievement_steps<S>(&self, achievement_name: S, steps: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string_int("setAchievementSteps", achievement_name, steps)
    }

    pub fn load_achievement_info(&self, force_reload: bool) -> Result<()> {
        self.call_with_bool("loadAchievementInfo", force_reload)
    }

    pub fn show_leader_board<S>(&self, leader_board_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string("showLeaderBoard", leader_board_id)
    }

    pub fn show_all_leader_boards(&self) -> Result<()> {
        self.call_void("showAllLeaderBoards")
    }

    pub fn submit_leader_board_score<S>(&self, leader_board_id: S, score: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string_int("submitLeaderBoardScore", leader_board_id, score)
    }

    pub fn submit_event<S>(&self, event_id: S, increment_by: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string_int("submitEvent", event_id, increment_by)
    }

    pub fn load_events(&self) -> Result<()> {
        self.call_void("loadEvents")
    }

    pub fn load_events_by_id<S>(&self, ids: &[S]) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let empty = JString::from_str(env, "")?;
            let string_array = JObjectArray::<JString>::new(env, ids.len(), &empty)?;
            for (index, id) in ids.iter().enumerate() {
                let id = JString::from_str(env, id)?;
                string_array.set_element(env, index, &id)?;
            }
            self.singleton
                .call_method(env, "loadEventsById", &[(&string_array).into()])?;
            Ok(())
        })
    }

    pub fn load_player_stats(&self, force_refresh: bool) -> Result<()> {
        self.call_with_bool("loadPlayerStats", force_refresh)
    }

    pub fn show_saved_games<S>(
        &self,
        title: S,
        allow_add_btn: bool,
        allow_delete_btn: bool,
        max_number_of_saved_games_to_show: i32,
    ) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let title = JString::from_str(env, title)?;
            self.singleton.call_method(
                env,
                "showSavedGames",
                &[
                    (&title).into(),
                    allow_add_btn.into(),
                    allow_delete_btn.into(),
                    max_number_of_saved_games_to_show.into(),
                ],
            )?;
            Ok(())
        })
    }

    pub fn save_snapshot<S>(&self, name: S, data: S, description: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.vm.attach_current_thread(|env| {
            let name = JString::from_str(env, name)?;
            let data = JString::from_str(env, data)?;
            let description = JString::from_str(env, description)?;
            self.singleton.call_method(
                env,
                "saveSnapshot",
                &[(&name).into(), (&data).into(), (&description).into()],
            )?;
            Ok(())
        })
    }

    pub fn load_snapshot<S>(&self, name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.call_with_string("loadSnapshot", name)
    }

    pub fn load_player_info(&self) -> Result<()> {
        self.call_void("loadPlayerInfo")
    }
}
