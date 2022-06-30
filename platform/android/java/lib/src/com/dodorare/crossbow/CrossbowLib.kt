package com.dodorare.crossbow

import android.app.Activity

class CrossbowLib {
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

	external fun requestPermissionResult(p_permission: String, p_result: Boolean)
}
