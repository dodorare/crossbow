@file:Suppress("DEPRECATION")

package com.crossbow.library

import com.crossbow.library.plugin.CrossbowPluginRegistry

import android.content.Intent
import android.content.Context
import android.util.Log
import android.os.Bundle
import android.content.pm.PackageManager
import android.app.Activity
import android.app.Fragment
import android.view.View
import android.view.ViewGroup
import android.view.ViewGroup.LayoutParams
import android.widget.FrameLayout
import androidx.annotation.CallSuper
import androidx.core.app.ActivityCompat
import androidx.annotation.Keep

class Crossbow : Fragment() {
    private var crossbowHost: CrossbowHost? = null
    public var pluginRegistry: CrossbowPluginRegistry? = null

	private var containerLayout: ViewGroup? = null
	private var mCurrentIntent: Intent? = null

	fun onNewIntent(intent: Intent) {
		mCurrentIntent = intent;
	}

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        pluginRegistry = CrossbowPluginRegistry.initializePluginRegistry(this)

		Log.v(TAG, "Initializing CrossbowLib Instance")
        // CrossbowLib.initialize(activity, this, activity!!.assets)
        onRenderInit()
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
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onMainRequestPermissionsResult(requestCode, permissions, grantResults)
        }
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

    /**
     * Used by the native code to complete initialization of plugins and renderer.
     */
    @Keep
    private fun onRenderInit() {
		Log.v(TAG, "Calling Crossbow onRenderInit")

        containerLayout = FrameLayout(activity)
        containerLayout?.setLayoutParams(
            ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT
            )
        )

        for (plugin in pluginRegistry!!.getAllPlugins()) {
            plugin.onRegisterPluginWithCrossbowNative()
        }

		Log.v(TAG, "Crossbow onRenderInit middle")

        // Include the returned non-null views in the Crossbow view hierarchy.
        for (plugin in pluginRegistry!!.getAllPlugins()) {
            val pluginView: View? = plugin.onMainCreate(activity)
            if (pluginView !== null) {
                if (plugin.shouldBeOnTop()) {
                    containerLayout?.addView(pluginView)
                } else {
                    containerLayout?.addView(pluginView, 0)
                }
            }
        }

		Log.v(TAG, "Crossbow onRenderInit finished")
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
