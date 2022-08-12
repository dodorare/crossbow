use crossbow_android::{error::*, jni::JavaVM, plugin::*};
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
        "PlayGamesServices"
    }

    fn get_receiver(&self) -> &Receiver<Signal> {
        self.singleton.get_receiver()
    }
}

impl PlayGamesServicesPlugin {
    pub fn init(&self, enable_popups: bool) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton
            .call_method(&jnienv, "init", &[enable_popups.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn init_with_saved_games<S>(&self, enable_popups: bool, save_game_name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let save_game_name_str = jnienv.new_string(save_game_name)?;
        self.singleton.call_method(
            &jnienv,
            "initWithSavedGames",
            &[enable_popups.into(), save_game_name_str.into()],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn sign_in(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton.call_method(&jnienv, "signIn", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn sign_out(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton.call_method(&jnienv, "signOut", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_signed_in(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let val = self.singleton.call_method(&jnienv, "isSignedIn", &[])?;
        Ok(val.z()?)
    }

    pub fn show_achievements(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton
            .call_method(&jnienv, "showAchievements", &[])?;
        Ok(())
    }

    pub fn unlock_achievement<S>(&self, achievement_name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let achievement_name_str = jnienv.new_string(achievement_name)?;
        self.singleton
            .call_method(&jnienv, "unlockAchievement", &[achievement_name_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn reveal_achievement<S>(&self, achievement_name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let achievement_name_str = jnienv.new_string(achievement_name)?;
        self.singleton
            .call_method(&jnienv, "revealAchievement", &[achievement_name_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn increment_achievement<S>(&self, achievement_name: S, step: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let achievement_name_str = jnienv.new_string(achievement_name)?;
        self.singleton.call_method(
            &jnienv,
            "incrementAchievement",
            &[achievement_name_str.into(), step.into()],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn set_achievement_steps<S>(&self, achievement_name: S, steps: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let achievement_name_str = jnienv.new_string(achievement_name)?;
        self.singleton.call_method(
            &jnienv,
            "setAchievementSteps",
            &[achievement_name_str.into(), steps.into()],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn load_achievement_info(&self, force_reload: bool) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton
            .call_method(&jnienv, "loadAchievementInfo", &[force_reload.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn show_leader_board<S>(&self, leader_board_id: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let leader_board_id_str = jnienv.new_string(leader_board_id)?;
        self.singleton
            .call_method(&jnienv, "showLeaderBoard", &[leader_board_id_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn show_all_leader_boards(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton
            .call_method(&jnienv, "showAllLeaderBoards", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn submit_leader_board_score<S>(&self, leader_board_id: S, score: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let leader_board_id_str = jnienv.new_string(leader_board_id)?;
        self.singleton.call_method(
            &jnienv,
            "submitLeaderBoardScore",
            &[leader_board_id_str.into(), score.into()],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn submit_event<S>(&self, event_id: S, increment_by: i32) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let event_id_str = jnienv.new_string(event_id)?;
        self.singleton.call_method(
            &jnienv,
            "submitEvent",
            &[event_id_str.into(), increment_by.into()],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn load_events(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton.call_method(&jnienv, "loadEvents", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn load_events_by_id<S>(&self, ids: &[S]) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let empty_str = jnienv.new_string("")?;
        let string_array =
            jnienv.new_object_array(ids.len() as i32, "java/lang/String", empty_str)?;
        for (index, id) in ids.iter().enumerate() {
            let id_str = jnienv.new_string(id)?;
            jnienv.set_object_array_element(string_array, index as i32, id_str)?;
        }
        self.singleton
            .call_method(&jnienv, "loadEventsById", &[string_array.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn load_player_stats(&self, force_refresh: bool) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton
            .call_method(&jnienv, "loadPlayerStats", &[force_refresh.into()])?;
        jnienv.exception_check()?;
        Ok(())
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
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let title_str = jnienv.new_string(title)?;
        self.singleton.call_method(
            &jnienv,
            "showSavedGames",
            &[
                title_str.into(),
                allow_add_btn.into(),
                allow_delete_btn.into(),
                max_number_of_saved_games_to_show.into(),
            ],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn save_snapshot<S>(&self, name: S, data: S, description: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let name_str = jnienv.new_string(name)?;
        let data_str = jnienv.new_string(data)?;
        let description_str = jnienv.new_string(description)?;
        self.singleton.call_method(
            &jnienv,
            "saveSnapshot",
            &[name_str.into(), data_str.into(), description_str.into()],
        )?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn load_snapshot<S>(&self, name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        let name_str = jnienv.new_string(name)?;
        self.singleton
            .call_method(&jnienv, "loadSnapshot", &[name_str.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn load_player_info(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon()?;
        self.singleton.call_method(&jnienv, "loadPlayerInfo", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }
}
