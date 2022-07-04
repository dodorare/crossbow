package com.dodorare.crossbow

import android.app.Activity

object CrossbowLib {
    init {
        // This is necessary when any of the following happens:
        //     - crossbow_android library is not configured to the following line in the manifest:
        //        <meta-data android:name="android.app.lib_name" android:value="crossbow_android" />
        //     - GameActivity derived class calls to the native code before calling
        //       the super.onCreate() function.
        System.loadLibrary("crossbow_android")
    }

    // /**
    //  * Invoked on the main thread to initialize Godot native layer.
    //  */
    // external fun initialize(
    //     activity: Activity?,
    //     p_instance: Crossbow?,
    //     p_asset_manager: Any?
    // )

    // /**
    //  * Invoked on the main thread to clean up Godot native layer.
    //  * @see androidx.fragment.app.Fragment.onDestroy
    //  */
    // external fun ondestroy()

    // /**
    //  * Forward [Activity.onBackPressed] event from the main thread to the GL thread.
    //  */
    // external fun back()

    /**
     * Forward the results from a permission request.
     * @see Activity.onRequestPermissionsResult
     * @param p_permission Request permission
     * @param p_result True if the permission was granted, false otherwise
     */
    external fun requestPermissionResult(p_permission: String?, p_result: Boolean)
}
