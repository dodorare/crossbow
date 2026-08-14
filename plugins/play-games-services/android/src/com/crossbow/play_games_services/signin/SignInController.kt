package com.crossbow.play_games_services.signin

import android.app.Activity
import com.google.android.gms.common.api.ApiException
import com.google.android.gms.games.PlayGames

class SignInController(
    private val activity: Activity,
    private val signInListener: SignInListener,
) {
    @Volatile
    private var authenticated = false

    fun refreshAuthentication() {
        PlayGames.getGamesSignInClient(activity).isAuthenticated
            .addOnCompleteListener(activity) { task ->
                authenticated = task.isSuccessful && task.result.isAuthenticated
                if (authenticated) notifySignedIn()
            }
    }

    fun signIn() {
        PlayGames.getGamesSignInClient(activity).signIn()
            .addOnCompleteListener(activity) { task ->
                authenticated = task.isSuccessful && task.result.isAuthenticated
                if (authenticated) {
                    notifySignedIn()
                } else {
                    signInListener.onSignInFailed(statusCode(task.exception))
                }
            }
    }

    /** PGS v2 intentionally no longer exposes programmatic sign-out. */
    fun signOut() {
        signInListener.onSignOutFailed()
    }

    fun isSignedIn(): Boolean = authenticated

    private fun notifySignedIn() {
        PlayGames.getPlayersClient(activity).currentPlayerId
            .addOnCompleteListener(activity) { playerTask ->
                if (playerTask.isSuccessful) {
                    signInListener.onSignedInSuccessfully(playerTask.result.orEmpty())
                } else {
                    authenticated = false
                    signInListener.onSignInFailed(statusCode(playerTask.exception))
                }
            }
    }

    private fun statusCode(error: Exception?): Int =
        (error as? ApiException)?.statusCode ?: 8 // CommonStatusCodes.INTERNAL_ERROR
}
