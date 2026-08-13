package com.crossbow.play_games_services.signin

interface SignInListener {
    fun onSignedInSuccessfully(playerId: String)
    fun onSignInFailed(statusCode: Int)
    fun onSignOutSuccess()
    fun onSignOutFailed()
}
