package com.dodorare.crossbow

import android.app.Activity
import android.app.NativeActivity
import android.util.Log
import android.os.Bundle
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat

open class CrossbowNativeActivity : NativeActivity(), ActivityCompat.OnRequestPermissionsResultCallback {
    companion object {
        init {
            // This is necessary when any of the following happens:
            //     - crossbow_android library is not configured to the following line in the manifest:
            //        <meta-data android:name="android.app.lib_name" android:value="crossbow_android" />
            //     - GameActivity derived class calls to the native code before calling
            //       the super.onCreate() function.
            System.loadLibrary("crossbow_android")
        }
    }
	private var crossbowInstance: CrossbowLib? = null

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		Log.v(TAG, "Creating new CrossbowLib instance")
		crossbowInstance = CrossbowLib()
	}

	override fun onRequestPermissionsResult(
		requestCode: Int,
		permissions: Array<out String>,
		grantResults: IntArray
	) {
		// TODO: Replace with https://tedblob.com/onrequestpermissionsresult-deprecated-android-java/

		super.onRequestPermissionsResult(requestCode, permissions, grantResults)
		// for (CrossbowPlugin plugin : pluginRegistry.getAllPlugins()) {
		// 	plugin.onMainRequestPermissionsResult(requestCode, permissions, grantResults)
		// }

		for (i in permissions.indices) {
			crossbowInstance?.requestPermissionResult(permissions[i], grantResults[i] == PackageManager.PERMISSION_GRANTED)
		}
	}
}
