package com.crossbow.play_games_services.stats

import android.app.Activity
import com.crossbow.play_games_services.model.PlayerStats
import com.google.android.gms.games.PlayGames
import com.google.gson.Gson

class PlayerStatsController(
    private val activity: Activity,
    private val playerStatsListener: PlayerStatsListener,
) {
    fun checkPlayerStats(forceRefresh: Boolean) {
        PlayGames.getPlayerStatsClient(activity).loadPlayerStats(forceRefresh)
            .addOnCompleteListener { task ->
                val stats = if (task.isSuccessful) task.result?.get() else null
                if (stats == null) {
                    playerStatsListener.onPlayerStatsLoadingFailed()
                    return@addOnCompleteListener
                }
                val playerStats = PlayerStats(
                    stats.averageSessionLength.toDouble(),
                    stats.daysSinceLastPlayed,
                    stats.numberOfPurchases,
                    stats.numberOfSessions,
                    stats.sessionPercentile.toDouble(),
                    stats.spendPercentile.toDouble(),
                )
                playerStatsListener.onPlayerStatsLoaded(Gson().toJson(playerStats))
            }
    }
}
