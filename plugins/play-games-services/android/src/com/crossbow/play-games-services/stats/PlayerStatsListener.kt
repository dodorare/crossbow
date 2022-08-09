package com.crossbow.play_games_services.stats

interface PlayerStatsListener {
    fun onPlayerStatsLoaded(statsJson: String)
    fun onPlayerStatsLoadingFailed()
}
