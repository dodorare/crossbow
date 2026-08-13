package com.crossbow.play_games_services.achievements

import android.app.Activity
import com.crossbow.play_games_services.model.AchievementInfo
import com.google.android.gms.games.PlayGames
import com.google.android.gms.games.achievement.Achievement
import com.google.gson.Gson

class AchievementsController(
    private val activity: Activity,
    private val achievementsListener: AchievementsListener,
) {
    companion object {
        const val RC_ACHIEVEMENT_UI = 9003
    }

    fun unlockAchievement(achievementName: String) {
        PlayGames.getAchievementsClient(activity).unlockImmediate(achievementName)
            .addOnSuccessListener { achievementsListener.onAchievementUnlocked(achievementName) }
            .addOnFailureListener { achievementsListener.onAchievementUnlockingFailed(achievementName) }
    }

    fun revealAchievement(achievementName: String) {
        PlayGames.getAchievementsClient(activity).revealImmediate(achievementName)
            .addOnSuccessListener { achievementsListener.onAchievementRevealed(achievementName) }
            .addOnFailureListener { achievementsListener.onAchievementRevealingFailed(achievementName) }
    }

    fun incrementAchievement(achievementName: String, step: Int) {
        PlayGames.getAchievementsClient(activity).incrementImmediate(achievementName, step)
            .addOnSuccessListener { achievementsListener.onAchievementIncremented(achievementName) }
            .addOnFailureListener { achievementsListener.onAchievementIncrementingFailed(achievementName) }
    }

    fun setAchievementSteps(achievementName: String, steps: Int) {
        PlayGames.getAchievementsClient(activity).setStepsImmediate(achievementName, steps)
            .addOnSuccessListener { achievementsListener.onAchievementStepsSet(achievementName) }
            .addOnFailureListener { achievementsListener.onAchievementStepsSettingFailed(achievementName) }
    }

    fun showAchievements() {
        PlayGames.getAchievementsClient(activity).achievementsIntent
            .addOnSuccessListener { activity.startActivityForResult(it, RC_ACHIEVEMENT_UI) }
            .addOnFailureListener { achievementsListener.onAchievementInfoLoadingFailed() }
    }

    fun loadAchievementInfo(forceReload: Boolean) {
        PlayGames.getAchievementsClient(activity).load(forceReload)
            .addOnCompleteListener { task ->
                val achievements = if (task.isSuccessful) task.result?.get() else null
                if (achievements == null) {
                    achievementsListener.onAchievementInfoLoadingFailed()
                    return@addOnCompleteListener
                }
                val list = achievements.map { achievement ->
                    val incremental = achievement.type == Achievement.TYPE_INCREMENTAL
                    AchievementInfo(
                        achievement.achievementId,
                        achievement.name,
                        achievement.description,
                        achievement.state,
                        achievement.type,
                        if (incremental) achievement.currentSteps else null,
                        if (incremental) achievement.totalSteps else null,
                        achievement.xpValue,
                    )
                }
                achievementsListener.onAchievementInfoLoaded(Gson().toJson(list))
            }
    }
}
