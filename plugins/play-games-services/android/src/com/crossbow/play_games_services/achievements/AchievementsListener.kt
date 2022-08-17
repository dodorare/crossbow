package com.crossbow.play_games_services.achievements

interface AchievementsListener {
    fun onAchievementUnlocked(achievementName: String)
    fun onAchievementUnlockingFailed(achievementName: String)
    fun onAchievementRevealed(achievementName: String)
    fun onAchievementRevealingFailed(achievementName: String)
    fun onAchievementIncremented(achievementName: String)
    fun onAchievementIncrementingFailed(achievementName: String)
    fun onAchievementStepsSet(achievementName: String)
    fun onAchievementStepsSettingFailed(achievementName: String)
    fun onAchievementInfoLoaded(achievementsJson: String)
    fun onAchievementInfoLoadingFailed()
}
