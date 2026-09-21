package com.crossbow.library

import android.app.Activity

object CrossbowLib {
    /** Initializes ndk-context for the Miniquad Java runtime. */
    @JvmStatic
    external fun initializeAndroidContext(activity: Activity)

    /** Releases a context initialized by [initializeAndroidContext]. */
    @JvmStatic
    external fun releaseAndroidContext()

    /**
     * Invoked on the main thread to initialize Crossbow native layer.
     */
    @JvmStatic
    external fun initialize(instance: Crossbow)

    /**
     * Forward the results from a permission request.
     * @see Activity.onRequestPermissionsResult
     * @param permission Request permission
     * @param result True if the permission was granted, false otherwise
     */
    @JvmStatic
    external fun requestPermissionResult(permission: String?, result: Boolean)
}
