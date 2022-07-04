package com.dodorare.crossbow

import com.dodorare.crossbow.plugin.CrossbowPluginRegistry

import android.content.Intent
import android.content.Context
import android.os.Bundle
import android.content.pm.PackageManager
import androidx.fragment.app.Fragment
import androidx.annotation.CallSuper
import androidx.core.app.ActivityCompat

class Crossbow : Fragment(), ActivityCompat.OnRequestPermissionsResultCallback {
    private var crossbowHost: CrossbowHost? = null
    private var pluginRegistry: CrossbowPluginRegistry? = null

	private var mCurrentIntent: Intent? = null

	fun onNewIntent(intent: Intent) {
		mCurrentIntent = intent;
	}

    override fun onCreate(icicle: Bundle?) {
        super.onCreate(icicle)
        pluginRegistry = CrossbowPluginRegistry.initializePluginRegistry(this)

        // CrossbowLib.initialize(activity, this, activity!!.assets)
    }

    override fun onAttach(context: Context) {
        super.onAttach(context)
        if (parentFragment is CrossbowHost) {
            crossbowHost = parentFragment as CrossbowHost?
        } else if (activity is CrossbowHost) {
            crossbowHost = activity as CrossbowHost?
        }
    }

    override fun onDetach() {
        super.onDetach()
        crossbowHost = null
    }

    @CallSuper
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onMainActivityResult(requestCode, resultCode, data)
        }
    }

    @CallSuper
    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        // for (plugin in pluginRegistry!!.getAllPlugins()) {
        //     plugin.onMainRequestPermissionsResult(requestCode, permissions, grantResults)
        // }
        for (i in permissions.indices) {
            CrossbowLib.requestPermissionResult(
                permissions[i],
                grantResults[i] == PackageManager.PERMISSION_GRANTED
            )
        }
    }

    /**
     * Invoked on the render thread when the Crossbow setup is complete.
     */
    @CallSuper
    protected fun onCrossbowSetupCompleted() {
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onCrossbowSetupCompleted()
        }
        if (crossbowHost != null) {
            crossbowHost?.onCrossbowSetupCompleted()
        }
    }

    /**
     * Invoked on the render thread when the Crossbow main loop has started.
     */
    @CallSuper
    protected fun onCrossbowMainLoopStarted() {
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onCrossbowMainLoopStarted()
        }
        if (crossbowHost != null) {
            crossbowHost?.onCrossbowMainLoopStarted()
        }
    }

    override fun onDestroy() {
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onMainDestroy()
        }
        // CrossbowLib.ondestroy()
        super.onDestroy()
    }

    override fun onPause() {
        super.onPause()
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onMainPause()
        }
    }

    override fun onResume() {
        super.onResume()
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onMainResume()
        }
    }

    fun onBackPressed() {
        var shouldQuit = true
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            if (plugin.onMainBackPressed()) {
                shouldQuit = false
            }
        }
        if (shouldQuit) {
            // CrossbowLib.back()
        }
    }

	fun runOnUiThread(action: Runnable) {
		if (activity != null) {
			activity!!.runOnUiThread(action)
		}
	}

    // fun requestPermission(p_name: String?): Boolean {
    //     return PermissionsUtil.requestPermission(p_name, activity)
    // }

    // fun requestPermissions(): Boolean {
    //     return PermissionsUtil.requestManifestPermissions(activity)
    // }

    // val grantedPermissions: Array<String>
    //     get() = PermissionsUtil.getGrantedPermissions(activity)
}
