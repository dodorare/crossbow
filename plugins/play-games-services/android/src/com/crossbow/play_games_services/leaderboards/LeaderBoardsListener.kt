package com.crossbow.play_games_services.leaderboards

interface LeaderBoardsListener {
    fun onLeaderBoardScoreSubmitted(leaderboardId: String)
    fun onLeaderBoardScoreSubmittingFailed(leaderboardId: String)
}
