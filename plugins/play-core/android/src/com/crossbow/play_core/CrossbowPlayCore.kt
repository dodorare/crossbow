@file:Suppress("DEPRECATION")

package com.crossbow.play_core

import androidx.collection.ArraySet
import com.crossbow.library.Crossbow
import com.crossbow.library.plugin.SignalInfo
import com.crossbow.library.plugin.CrossbowPlugin
import com.crossbow.library.plugin.ExposedToCrossbow

import com.google.android.play.core.appupdate.AppUpdateManager
import com.google.android.play.core.appupdate.AppUpdateManagerFactory
import com.google.android.play.core.install.model.AppUpdateType
import com.google.android.play.core.install.model.UpdateAvailability

class CrossbowPlayCore(crossbow: Crossbow) : CrossbowPlugin(crossbow) {
    private var appUpdate: AppUpdateManager
    private val REQUEST_CODE = 100

    init {
        appUpdate = AppUpdateManagerFactory.create(crossbow.activity!!)
    }

    override val pluginName: String
        get() = javaClass.simpleName

    override val pluginSignals: Set<SignalInfo>
        get() {
            val signals: MutableSet<SignalInfo> = ArraySet()
            signals.add(SignalInfo("start_update_flow"))
            signals.add(SignalInfo("continue_update_flow"))
            return signals
        }

    @ExposedToCrossbow
    fun checkUpdate() {
        // Checks that the platform will allow the specified type of update.
        appUpdate.appUpdateInfo.addOnSuccessListener { appUpdateInfo ->
            if (appUpdateInfo.updateAvailability() == UpdateAvailability.UPDATE_AVAILABLE
                // This example applies an immediate update. To apply a flexible update
                // instead, pass in AppUpdateType.FLEXIBLE
                && appUpdateInfo.isUpdateTypeAllowed(AppUpdateType.IMMEDIATE)
            ) {
                // Request the update.
                appUpdate.startUpdateFlowForResult(appUpdateInfo, AppUpdateType.IMMEDIATE, crossbow.activity, REQUEST_CODE)
                emitSignal("start_update_flow")
            }
        }
    }

    @ExposedToCrossbow
    fun inProgressUpdate() {
        // Checks that the platform will allow the specified type of update.
        appUpdate.appUpdateInfo.addOnSuccessListener { appUpdateInfo ->
            if (appUpdateInfo.updateAvailability() == UpdateAvailability.DEVELOPER_TRIGGERED_UPDATE_IN_PROGRESS) {
                // Request the update.
                appUpdate.startUpdateFlowForResult(appUpdateInfo, AppUpdateType.IMMEDIATE, crossbow.activity, REQUEST_CODE)
                emitSignal("continue_update_flow")
            }
        }
    }
}
