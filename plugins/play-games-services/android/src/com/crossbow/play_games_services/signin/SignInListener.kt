package com.crossbow.play_games_services.signin

interface SignInListener {
    fun onSignedInSuccessfully(accountId: String)
    fun onSignInFailed(statusCode: Int)
    fun onSignOutSuccess()
    fun onSignOutFailed()
}
