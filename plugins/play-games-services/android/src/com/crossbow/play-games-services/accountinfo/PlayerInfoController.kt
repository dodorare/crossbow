package com.crossbow.play_games_services.accountinfo

import android.app.Activity
import com.google.android.gms.auth.api.signin.GoogleSignIn
import com.google.android.gms.games.Games
import com.google.gson.Gson
import com.crossbow.play_games_services.ConnectionController
import com.crossbow.play_games_services.model.PlayerInfo
import com.crossbow.play_games_services.model.PlayerLevel
import com.crossbow.play_games_services.model.PlayerLevelInfo

class PlayerInfoController(
    private val activity: Activity,
    private val playerInfoListener: PlayerInfoListener,
    private val connectionController: ConnectionController
) {

    fun fetchPlayerInfo() {
        val googleSignInAccount = GoogleSignIn.getLastSignedInAccount(activity)
        if (connectionController.isConnected().first && googleSignInAccount != null) {
            Games.getPlayersClient(activity, googleSignInAccount).currentPlayer
                .addOnCompleteListener { task ->
                    val player = task.result
                    if (task.isSuccessful && player != null) {
                        val levelInfo = player.levelInfo
                        val playerLevelInfo = if (levelInfo != null) {
                            PlayerLevelInfo(
                                levelInfo.currentXpTotal,
                                levelInfo.lastLevelUpTimestamp,
                                if (levelInfo.currentLevel != null) PlayerLevel(
                                    levelInfo.currentLevel.levelNumber,
                                    levelInfo.currentLevel.minXp,
                                    levelInfo.currentLevel.maxXp
                                ) else null,
                                if (levelInfo.nextLevel != null) PlayerLevel(
                                    levelInfo.nextLevel.levelNumber,
                                    levelInfo.nextLevel.minXp,
                                    levelInfo.nextLevel.maxXp
                                ) else null
                            )
                        } else {
                            null
                        }

                        val playerInfo = PlayerInfo(
                            player.playerId,
                            player.displayName,
                            player.name,
                            player.iconImageUri.toString(),
                            player.hiResImageUri.toString(),
                            player.title,
                            player.bannerImageLandscapeUri.toString(),
                            player.bannerImagePortraitUri.toString(),
                            playerLevelInfo
                        )

                        playerInfoListener.onPlayerInfoLoaded(Gson().toJson(playerInfo))
                    } else {
                        playerInfoListener.onPlayerInfoLoadingFailed()
                    }
                }
        } else {
            playerInfoListener.onPlayerInfoLoadingFailed()
        }
    }
}
