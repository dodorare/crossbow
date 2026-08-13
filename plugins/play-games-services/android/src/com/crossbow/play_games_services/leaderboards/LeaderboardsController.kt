package com.crossbow.play_games_services.leaderboards

import android.app.Activity
import com.google.android.gms.games.PlayGames

class LeaderboardsController(
    private val activity: Activity,
    private val leaderBoardsListener: LeaderBoardsListener,
) {
    companion object {
        const val RC_LEADERBOARD_UI = 9004
    }

    fun submitScore(leaderboardId: String, score: Int) {
        PlayGames.getLeaderboardsClient(activity)
            .submitScoreImmediate(leaderboardId, score.toLong())
            .addOnSuccessListener { leaderBoardsListener.onLeaderBoardScoreSubmitted(leaderboardId) }
            .addOnFailureListener {
                leaderBoardsListener.onLeaderBoardScoreSubmittingFailed(leaderboardId)
            }
    }

    fun showLeaderboard(leaderboardId: String) {
        PlayGames.getLeaderboardsClient(activity).getLeaderboardIntent(leaderboardId)
            .addOnSuccessListener { activity.startActivityForResult(it, RC_LEADERBOARD_UI) }
            .addOnFailureListener {
                leaderBoardsListener.onLeaderBoardScoreSubmittingFailed(leaderboardId)
            }
    }

    fun showAllLeaderboards() {
        PlayGames.getLeaderboardsClient(activity).allLeaderboardsIntent
            .addOnSuccessListener { activity.startActivityForResult(it, RC_LEADERBOARD_UI) }
    }
}
