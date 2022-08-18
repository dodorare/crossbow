package com.crossbow.play_games_services.accountinfo

interface PlayerInfoListener {
    fun onPlayerInfoLoadingFailed()
    fun onPlayerInfoLoaded(response: String)
}
