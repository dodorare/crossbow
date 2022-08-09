use crossbow_android::{error::*, jni::JavaVM, plugin::*};
use std::sync::Arc;

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
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton
            .call_method(&jnienv, "init", &[enable_popups.into()])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn init_with_saved_games<S>(&self, enable_popups: bool, save_game_name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
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
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton.call_method(&jnienv, "signIn", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn sign_out(&self) -> Result<()> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        self.singleton.call_method(&jnienv, "signOut", &[])?;
        jnienv.exception_check()?;
        Ok(())
    }

    pub fn is_signed_in(&self) -> Result<bool> {
        let jnienv = self.vm.attach_current_thread_as_daemon().unwrap();
        let val = self.singleton.call_method(&jnienv, "isSignedIn", &[])?;
        Ok(val.z()?)
    }

    // fun showAchievements()
    // fun unlockAchievement(achievementName: String)
    // fun revealAchievement(achievementName: String)
    // fun incrementAchievement(achievementName: String, step: Int)
    // fun setAchievementSteps(achievementName: String, steps: Int)
    // fun loadAchievementInfo(forceReload: Boolean)
    // fun showLeaderBoard(leaderBoardId: String)
    // fun showAllLeaderBoards()
    // fun submitLeaderBoardScore(leaderBoardId: String, score: Int)
    // fun submitEvent(eventId: String, incrementBy: Int)
    // fun loadEvents()
    // fun loadEventsById(ids: Array<String>)
    // fun loadPlayerStats(forceRefresh: Boolean)
    // fun showSavedGames(
    //     title: String,
    //     allowAdBtn: Boolean,
    //     allowDeleteBtn: Boolean,
    //     maxNumberOfSavedGamesToShow: Int)
    // fun saveSnapshot(name: String, data: String, description: String)
    // fun loadSnapshot(name: String)
    // fun loadPlayerInfo()
}
