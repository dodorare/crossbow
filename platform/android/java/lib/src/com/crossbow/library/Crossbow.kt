package com.crossbow.library

import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.view.ViewGroup
import android.widget.FrameLayout
import androidx.annotation.Keep
import com.crossbow.library.plugin.CrossbowPluginRegistry

/**
 * Owns Crossbow's Android plugin lifecycle and overlay view.
 * Construct it only after the host has loaded the application's native library.
 */
class Crossbow(val activity: Activity) {
    val pluginRegistry: CrossbowPluginRegistry =
        CrossbowPluginRegistry.initializePluginRegistry(this)

    val view = FrameLayout(activity).apply {
        layoutParams = ViewGroup.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.MATCH_PARENT
        )
    }

    init {
        CrossbowLib.initialize(this)
    }

    /** Called by the native layer once it is ready to register Android plugins. */
    @Keep
    private fun onRenderInit() {
        for (plugin in plugins()) {
            plugin.onRegisterPluginWithCrossbowNative()
        }
        for (plugin in plugins()) {
            plugin.onMainCreate(activity)?.let { pluginView ->
                view.addView(pluginView, if (plugin.shouldBeOnTop()) view.childCount else 0)
            }
        }
    }

    fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        for (plugin in plugins()) {
            plugin.onMainActivityResult(requestCode, resultCode, data)
        }
    }

    fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String>,
        grantResults: IntArray
    ) {
        for (plugin in plugins()) {
            plugin.onMainRequestPermissionsResult(requestCode, permissions, grantResults)
        }
        permissions.forEachIndexed { index, permission ->
            CrossbowLib.requestPermissionResult(
                permission,
                grantResults.getOrNull(index) == PackageManager.PERMISSION_GRANTED
            )
        }
    }

    fun onPause() {
        for (plugin in plugins()) {
            plugin.onMainPause()
        }
    }

    fun onResume() {
        for (plugin in plugins()) {
            plugin.onMainResume()
        }
    }

    fun onBackPressed(): Boolean =
        plugins().fold(false) { handled, plugin ->
            plugin.onMainBackPressed() || handled
        }

    fun onDestroy() {
        for (plugin in plugins()) {
            plugin.onMainDestroy()
        }
        CrossbowPluginRegistry.clearPluginRegistry(pluginRegistry)
    }

    fun runOnUiThread(action: Runnable) = activity.runOnUiThread(action)

    val grantedPermissions: Array<String>
        get() = PermissionsUtil.getGrantedPermissions(activity)

    @Keep
    fun requestPermission(permission: String): Boolean =
        PermissionsUtil.requestPermission(permission, activity)

    @Keep
    fun requestPermissions(): Boolean = PermissionsUtil.requestManifestPermissions(activity)

    private fun plugins() = pluginRegistry.allPlugins
}
