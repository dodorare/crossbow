@file:Suppress("DEPRECATION")

package com.crossbow.play_games_services

import android.app.Activity
import android.content.Intent
import com.google.android.gms.auth.api.Auth
import com.google.android.gms.auth.api.signin.GoogleSignIn
import com.google.android.gms.auth.api.signin.GoogleSignInClient
import com.google.android.gms.auth.api.signin.GoogleSignInOptions
import com.google.android.gms.drive.Drive
import com.google.android.gms.games.SnapshotsClient
import com.google.android.gms.games.snapshot.SnapshotMetadata
import com.crossbow.play_games_services.accountinfo.PlayerInfoController
import com.crossbow.play_games_services.accountinfo.PlayerInfoListener
import com.crossbow.play_games_services.achievements.AchievementsController
import com.crossbow.play_games_services.achievements.AchievementsListener
import com.crossbow.play_games_services.events.EventsController
import com.crossbow.play_games_services.events.EventsListener
import com.crossbow.play_games_services.leaderboards.LeaderBoardsListener
import com.crossbow.play_games_services.leaderboards.LeaderboardsController
import com.crossbow.play_games_services.savedgames.SavedGamesController
import com.crossbow.play_games_services.savedgames.SavedGamesListener
import com.crossbow.play_games_services.signin.SignInController
import com.crossbow.play_games_services.signin.SignInListener
import com.crossbow.play_games_services.stats.PlayerStatsController
import com.crossbow.play_games_services.stats.PlayerStatsListener
import com.crossbow.library.BuildConfig
import com.crossbow.library.Crossbow
import com.crossbow.library.plugin.CrossbowPlugin
import com.crossbow.library.plugin.ExposedToCrossbow
import com.crossbow.library.plugin.SignalInfo
import java.math.BigInteger
import java.util.Random

class PlayGamesServices(crossbow: Crossbow) : CrossbowPlugin(crossbow), AchievementsListener, EventsListener,
    LeaderBoardsListener, SavedGamesListener, SignInListener, PlayerStatsListener, PlayerInfoListener {

    private lateinit var connectionController: ConnectionController
    private lateinit var signInController: SignInController
    private lateinit var achievementsController: AchievementsController
    private lateinit var leaderboardsController: LeaderboardsController
    private lateinit var eventsController: EventsController
    private lateinit var playerStatsController: PlayerStatsController
    private lateinit var playerInfoController: PlayerInfoController
    private lateinit var savedGamesController: SavedGamesController
    private lateinit var googleSignInClient: GoogleSignInClient

    private lateinit var saveGameName: String

    companion object {
        val SIGNAL_SIGN_IN_SUCCESSFUL = SignalInfo("_on_sign_in_success", String::class.java)
        val SIGNAL_SIGN_IN_FAILED = SignalInfo("_on_sign_in_failed", Int::class.javaObjectType)
        val SIGNAL_SIGN_OUT_SUCCESS = SignalInfo("_on_sign_out_success")
        val SIGNAL_SIGN_OUT_FAILED = SignalInfo("_on_sign_out_failed")
        val SIGNAL_ACHIEVEMENT_UNLOCKED = SignalInfo("_on_achievement_unlocked", String::class.java)
        val SIGNAL_ACHIEVEMENT_UNLOCKED_FAILED = SignalInfo("_on_achievement_unlocking_failed", String::class.java)
        val SIGNAL_ACHIEVEMENT_REVEALED = SignalInfo("_on_achievement_revealed", String::class.java)
        val SIGNAL_ACHIEVEMENT_REVEALED_FAILED = SignalInfo("_on_achievement_revealing_failed", String::class.java)
        val SIGNAL_ACHIEVEMENT_INCREMENTED = SignalInfo("_on_achievement_incremented", String::class.java)
        val SIGNAL_ACHIEVEMENT_INCREMENTED_FAILED =
            SignalInfo("_on_achievement_incrementing_failed", String::class.java)
        val SIGNAL_ACHIEVEMENT_STEPS_SET = SignalInfo("_on_achievement_steps_set", String::class.java)
        val SIGNAL_ACHIEVEMENT_STEPS_SET_FAILED =
            SignalInfo("_on_achievement_steps_setting_failed", String::class.java)
        val SIGNAL_ACHIEVEMENT_INFO_LOAD = SignalInfo("_on_achievement_info_loaded", String::class.java)
        val SIGNAL_ACHIEVEMENT_INFO_LOAD_FAILED = SignalInfo("_on_achievement_info_load_failed", String::class.java)
        val SIGNAL_LEADERBOARD_SCORE_SUBMITTED = SignalInfo("_on_leaderboard_score_submitted", String::class.java)
        val SIGNAL_LEADERBOARD_SCORE_SUBMITTED_FAILED =
            SignalInfo("_on_leaderboard_score_submitting_failed", String::class.java)
        val SIGNAL_EVENT_SUBMITTED = SignalInfo("_on_event_submitted", String::class.java)
        val SIGNAL_EVENT_SUBMITTED_FAILED = SignalInfo("_on_event_submitting_failed", String::class.java)
        val SIGNAL_EVENTS_LOADED = SignalInfo("_on_events_loaded", String::class.java)
        val SIGNAL_EVENTS_EMPTY = SignalInfo("_on_events_empty")
        val SIGNAL_EVENTS_LOADED_FAILED = SignalInfo("_on_events_loading_failed")
        val SIGNAL_PLAYER_STATS_LOADED = SignalInfo("_on_player_stats_loaded", String::class.java)
        val SIGNAL_PLAYER_STATS_LOADED_FAILED = SignalInfo("_on_player_stats_loading_failed")
        val SIGNAL_SAVED_GAME_SUCCESS = SignalInfo("_on_game_saved_success")
        val SIGNAL_SAVED_GAME_FAILED = SignalInfo("_on_game_saved_fail")
        val SIGNAL_SAVED_GAME_LOAD_SUCCESS = SignalInfo("_on_game_load_success", String::class.java)
        val SIGNAL_SAVED_GAME_LOAD_FAIL = SignalInfo("_on_game_load_fail")
        val SIGNAL_SAVED_GAME_CREATE_SNAPSHOT = SignalInfo("_on_create_new_snapshot", String::class.java)
        val SIGNAL_PLAYER_INFO_LOADED = SignalInfo("_on_player_info_loaded", String::class.java)
        val SIGNAL_PLAYER_INFO_LOADED_FAILED = SignalInfo("_on_player_info_loading_failed")
    }

    override val pluginName: String
        get() = javaClass.simpleName

    override val pluginSignals: Set<SignalInfo>
        get() {
            return mutableSetOf(
                SIGNAL_SIGN_IN_SUCCESSFUL,
                SIGNAL_SIGN_IN_FAILED,
                SIGNAL_SIGN_OUT_SUCCESS,
                SIGNAL_SIGN_OUT_FAILED,
                SIGNAL_ACHIEVEMENT_UNLOCKED,
                SIGNAL_ACHIEVEMENT_UNLOCKED_FAILED,
                SIGNAL_ACHIEVEMENT_REVEALED,
                SIGNAL_ACHIEVEMENT_REVEALED_FAILED,
                SIGNAL_ACHIEVEMENT_INCREMENTED,
                SIGNAL_ACHIEVEMENT_INCREMENTED_FAILED,
                SIGNAL_ACHIEVEMENT_STEPS_SET,
                SIGNAL_ACHIEVEMENT_STEPS_SET_FAILED,
                SIGNAL_ACHIEVEMENT_INFO_LOAD,
                SIGNAL_ACHIEVEMENT_INFO_LOAD_FAILED,
                SIGNAL_LEADERBOARD_SCORE_SUBMITTED,
                SIGNAL_LEADERBOARD_SCORE_SUBMITTED_FAILED,
                SIGNAL_EVENT_SUBMITTED,
                SIGNAL_EVENT_SUBMITTED_FAILED,
                SIGNAL_EVENTS_LOADED,
                SIGNAL_EVENTS_EMPTY,
                SIGNAL_EVENTS_LOADED_FAILED,
                SIGNAL_PLAYER_STATS_LOADED,
                SIGNAL_PLAYER_STATS_LOADED_FAILED,
                SIGNAL_SAVED_GAME_SUCCESS,
                SIGNAL_SAVED_GAME_FAILED,
                SIGNAL_SAVED_GAME_LOAD_SUCCESS,
                SIGNAL_SAVED_GAME_LOAD_FAIL,
                SIGNAL_SAVED_GAME_CREATE_SNAPSHOT,
                SIGNAL_PLAYER_INFO_LOADED,
                SIGNAL_PLAYER_INFO_LOADED_FAILED
            )
        }

    override fun onMainActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        if (requestCode == SignInController.RC_SIGN_IN) {
            val googleSignInResult = Auth.GoogleSignInApi.getSignInResultFromIntent(data)
            signInController.onSignInActivityResult(googleSignInResult)
        } else if (requestCode == SavedGamesController.RC_SAVED_GAMES) {
            if (data != null) {
                if (data.hasExtra(SnapshotsClient.EXTRA_SNAPSHOT_METADATA)) {
                    data.getParcelableExtra<SnapshotMetadata>(SnapshotsClient.EXTRA_SNAPSHOT_METADATA)?.let {
                        savedGamesController.loadSnapshot(it.uniqueName)
                    }
                } else if (data.hasExtra(SnapshotsClient.EXTRA_SNAPSHOT_NEW)) {
                    val unique = BigInteger(281, Random()).toString(13)
                    savedGamesController.createNewSnapshot("$saveGameName$unique")
                }
            }
        }
    }

    private fun initialize(enableSaveGamesFunctionality: Boolean, enablePopups: Boolean, saveGameName: String) {
        this.saveGameName = saveGameName
        val signInOptions = if (enableSaveGamesFunctionality) {
            val signInOptionsBuilder = GoogleSignInOptions.Builder(GoogleSignInOptions.DEFAULT_GAMES_SIGN_IN)
            signInOptionsBuilder.requestScopes(Drive.SCOPE_APPFOLDER).requestId()
            signInOptionsBuilder.build()
        } else {
            GoogleSignInOptions.DEFAULT_GAMES_SIGN_IN
        }

        connectionController = ConnectionController(crossbow.activity!!, signInOptions)
        signInController = SignInController(crossbow.activity!!, this, connectionController)
        achievementsController = AchievementsController(crossbow.activity!!, this, connectionController)
        leaderboardsController = LeaderboardsController(crossbow.activity!!, this, connectionController)
        eventsController = EventsController(crossbow.activity!!, this, connectionController)
        playerStatsController = PlayerStatsController(crossbow.activity!!, this, connectionController)
        playerInfoController = PlayerInfoController(crossbow.activity!!, this, connectionController)
        savedGamesController = SavedGamesController(crossbow.activity!!, this, connectionController)

        googleSignInClient = GoogleSignIn.getClient(crossbow.activity!!, signInOptions)

        runOnUiThread {
            signInController.setShowPopups(enablePopups)
        }
    }

    @ExposedToCrossbow
    fun init(enablePopups: Boolean) {
        initialize(false, enablePopups, "DefaultGame")
    }

    @ExposedToCrossbow
    fun initWithSavedGames(enablePopups: Boolean, saveGameName: String) {
        initialize(true, enablePopups, saveGameName)
    }

    @ExposedToCrossbow
    fun signIn() {
        runOnUiThread {
            signInController.signIn(googleSignInClient)
        }
    }

    @ExposedToCrossbow
    fun signOut() {
        runOnUiThread {
            signInController.signOut(googleSignInClient)
        }
    }

    @ExposedToCrossbow
    fun isSignedIn(): Boolean {
        return signInController.isSignedIn()
    }

    @ExposedToCrossbow
    fun showAchievements() {
        runOnUiThread {
            achievementsController.showAchievements()
        }
    }

    @ExposedToCrossbow
    fun unlockAchievement(achievementName: String) {
        runOnUiThread {
            achievementsController.unlockAchievement(achievementName)
        }
    }

    @ExposedToCrossbow
    fun revealAchievement(achievementName: String) {
        runOnUiThread {
            achievementsController.revealAchievement(achievementName)
        }
    }

    @ExposedToCrossbow
    fun incrementAchievement(achievementName: String, step: Int) {
        runOnUiThread {
            achievementsController.incrementAchievement(achievementName, step)
        }
    }

    @ExposedToCrossbow
    fun setAchievementSteps(achievementName: String, steps: Int) {
        runOnUiThread {
            achievementsController.setAchievementSteps(achievementName, steps)
        }
    }

    @ExposedToCrossbow
    fun loadAchievementInfo(forceReload: Boolean) {
        runOnUiThread {
            achievementsController.loadAchievementInfo(forceReload)
        }
    }

    @ExposedToCrossbow
    fun showLeaderBoard(leaderBoardId: String) {
        runOnUiThread {
            leaderboardsController.showLeaderboard(leaderBoardId)
        }
    }

    @ExposedToCrossbow
    fun showAllLeaderBoards() {
        runOnUiThread {
            leaderboardsController.showAllLeaderboards()
        }
    }

    @ExposedToCrossbow
    fun submitLeaderBoardScore(leaderBoardId: String, score: Int) {
        runOnUiThread {
            leaderboardsController.submitScore(leaderBoardId, score)
        }
    }

    @ExposedToCrossbow
    fun submitEvent(eventId: String, incrementBy: Int) {
        runOnUiThread {
            eventsController.submitEvent(eventId, incrementBy)
        }
    }

    @ExposedToCrossbow
    fun loadEvents() {
        runOnUiThread {
            eventsController.loadEvents()
        }
    }

    @ExposedToCrossbow
    fun loadEventsById(ids: Array<String>) {
        runOnUiThread {
            eventsController.loadEventById(ids)
        }
    }

    @ExposedToCrossbow
    fun loadPlayerStats(forceRefresh: Boolean) {
        runOnUiThread {
            playerStatsController.checkPlayerStats(forceRefresh)
        }
    }

    @ExposedToCrossbow
    fun showSavedGames(title: String, allowAddBtn: Boolean, allowDeleteBtn: Boolean, maxNumberOfSavedGamesToShow: Int) {
        runOnUiThread {
            savedGamesController.showSavedGamesUI(title, allowAddBtn, allowDeleteBtn, maxNumberOfSavedGamesToShow)
        }
    }

    @ExposedToCrossbow
    fun saveSnapshot(name: String, data: String, description: String) {
        runOnUiThread {
            savedGamesController.saveSnapshot(name, data, description)
        }
    }

    @ExposedToCrossbow
    fun loadSnapshot(name: String) {
        runOnUiThread {
            savedGamesController.loadSnapshot(name)
        }
    }

    @ExposedToCrossbow
    fun loadPlayerInfo() {
        runOnUiThread {
            playerInfoController.fetchPlayerInfo()
        }
    }

    override fun onAchievementUnlocked(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_UNLOCKED.name, achievementName)
    }

    override fun onAchievementUnlockingFailed(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_UNLOCKED_FAILED.name, achievementName)
    }

    override fun onAchievementRevealed(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_REVEALED.name, achievementName)
    }

    override fun onAchievementRevealingFailed(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_REVEALED_FAILED.name, achievementName)
    }

    override fun onAchievementIncremented(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_INCREMENTED.name, achievementName)
    }

    override fun onAchievementIncrementingFailed(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_INCREMENTED_FAILED.name, achievementName)
    }

    override fun onAchievementStepsSet(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_STEPS_SET.name, achievementName)
    }

    override fun onAchievementStepsSettingFailed(achievementName: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_STEPS_SET_FAILED.name, achievementName)
    }

    override fun onAchievementInfoLoaded(achievementsJson: String) {
        emitSignal(SIGNAL_ACHIEVEMENT_INFO_LOAD.name, achievementsJson)
    }

    override fun onAchievementInfoLoadingFailed() {
        emitSignal(SIGNAL_ACHIEVEMENT_INFO_LOAD_FAILED.name)
    }

    override fun onEventSubmitted(eventId: String) {
        emitSignal(SIGNAL_EVENT_SUBMITTED.name, eventId)
    }

    override fun onEventSubmittingFailed(eventId: String) {
        emitSignal(SIGNAL_EVENT_SUBMITTED_FAILED.name, eventId)
    }

    override fun onEventsLoaded(eventsJson: String) {
        emitSignal(SIGNAL_EVENTS_LOADED.name, eventsJson)
    }

    override fun onEventsEmpty() {
        emitSignal(SIGNAL_EVENTS_EMPTY.name)
    }

    override fun onEventsLoadingFailed() {
        emitSignal(SIGNAL_EVENTS_LOADED_FAILED.name)
    }

    override fun onLeaderBoardScoreSubmitted(leaderboardId: String) {
        emitSignal(SIGNAL_LEADERBOARD_SCORE_SUBMITTED.name, leaderboardId)
    }

    override fun onLeaderBoardScoreSubmittingFailed(leaderboardId: String) {
        emitSignal(SIGNAL_LEADERBOARD_SCORE_SUBMITTED_FAILED.name, leaderboardId)
    }

    override fun onSavedGameSuccess() {
        emitSignal(SIGNAL_SAVED_GAME_SUCCESS.name)
    }

    override fun onSavedGameFailed() {
        emitSignal(SIGNAL_SAVED_GAME_FAILED.name)
    }

    override fun onSavedGameLoadFailed() {
        emitSignal(SIGNAL_SAVED_GAME_LOAD_FAIL.name)
    }

    override fun onSavedGameLoadSuccess(data: String) {
        emitSignal(SIGNAL_SAVED_GAME_LOAD_SUCCESS.name, data)
    }

    override fun onSavedGameCreateSnapshot(currentSaveName: String) {
        emitSignal(SIGNAL_SAVED_GAME_CREATE_SNAPSHOT.name, currentSaveName)
    }

    override fun onSignedInSuccessfully(accountId: String) {
        emitSignal(SIGNAL_SIGN_IN_SUCCESSFUL.name, accountId)
    }

    override fun onSignInFailed(statusCode: Int) {
        emitSignal(SIGNAL_SIGN_IN_FAILED.name, statusCode)
    }

    override fun onSignOutSuccess() {
        emitSignal(SIGNAL_SIGN_OUT_SUCCESS.name)
    }

    override fun onSignOutFailed() {
        emitSignal(SIGNAL_SIGN_OUT_FAILED.name)
    }

    override fun onPlayerStatsLoaded(statsJson: String) {
        emitSignal(SIGNAL_PLAYER_STATS_LOADED.name, statsJson)
    }

    override fun onPlayerStatsLoadingFailed() {
        emitSignal(SIGNAL_PLAYER_STATS_LOADED_FAILED.name)
    }

    override fun onPlayerInfoLoaded(response: String) {
        emitSignal(SIGNAL_PLAYER_INFO_LOADED.name, response)
    }

    override fun onPlayerInfoLoadingFailed() {
        emitSignal(SIGNAL_PLAYER_INFO_LOADED_FAILED.name)
    }
}
