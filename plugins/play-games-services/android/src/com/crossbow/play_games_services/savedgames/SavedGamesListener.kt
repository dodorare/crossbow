package com.crossbow.play_games_services.savedgames

interface SavedGamesListener {
    fun onSavedGameSuccess()
    fun onSavedGameFailed()
    fun onSavedGameLoadFailed()
    fun onSavedGameLoadSuccess(data: String)
    fun onSavedGameCreateSnapshot(currentSaveName: String)
}
