package com.crossbow.play_games_services.accountinfo

import android.app.Activity
import com.crossbow.play_games_services.model.PlayerInfo
import com.crossbow.play_games_services.model.PlayerLevel
import com.crossbow.play_games_services.model.PlayerLevelInfo
import com.google.android.gms.games.PlayGames
import com.google.gson.Gson

class PlayerInfoController(
    private val activity: Activity,
    private val playerInfoListener: PlayerInfoListener,
) {
    fun fetchPlayerInfo() {
        PlayGames.getPlayersClient(activity).currentPlayer
            .addOnCompleteListener { task ->
                val player = if (task.isSuccessful) task.result else null
                if (player == null) {
                    playerInfoListener.onPlayerInfoLoadingFailed()
                    return@addOnCompleteListener
                }
                val levelInfo = player.levelInfo
                val playerLevelInfo = levelInfo?.let {
                    PlayerLevelInfo(
                        it.currentXpTotal,
                        it.lastLevelUpTimestamp,
                        it.currentLevel?.let { level ->
                            PlayerLevel(level.levelNumber, level.minXp, level.maxXp)
                        },
                        it.nextLevel?.let { level ->
                            PlayerLevel(level.levelNumber, level.minXp, level.maxXp)
                        },
                    )
                }
                val playerInfo = PlayerInfo(
                    player.playerId,
                    player.displayName,
                    player.displayName,
                    player.iconImageUri?.toString().orEmpty(),
                    player.hiResImageUri?.toString().orEmpty(),
                    player.title,
                    player.bannerImageLandscapeUri?.toString().orEmpty(),
                    player.bannerImagePortraitUri?.toString().orEmpty(),
                    playerLevelInfo,
                )
                playerInfoListener.onPlayerInfoLoaded(Gson().toJson(playerInfo))
            }
    }
}
