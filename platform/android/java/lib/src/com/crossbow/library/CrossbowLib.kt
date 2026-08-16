package com.crossbow.library

import android.app.Activity

object CrossbowLib {
    /** Initializes ndk-context for Java-Activity runtimes such as Miniquad. */
    @JvmStatic
    external fun initializeAndroidContext(activity: Activity)

    /** Releases a context initialized by [initializeAndroidContext]. */
    @JvmStatic
    external fun releaseAndroidContext()

    /**
     * Invoked on the main thread to initialize Crossbow native layer.
     */
    @JvmStatic
    external fun initialize(
        activity: Activity,
        instance: Crossbow,
        asset_manager: Any
    )

    /**
     * Invoked on the main thread to clean up Crossbow native layer.
     * @see androidx.fragment.app.Fragment.onDestroy
     */
    @JvmStatic
    external fun onDestroy()

    /**
     * Forward [Activity.onBackPressed] event from the main thread to the GL thread.
     */
    @JvmStatic
    external fun onBackPressed()

	/**
	 * Invoked when the Android app resumes.
	 * @see androidx.fragment.app.Fragment#onResume()
	 */
    @JvmStatic
    external fun focusIn()

	/**
	 * Invoked when the Android app pauses.
	 * @see androidx.fragment.app.Fragment#onPause()
	 */
    @JvmStatic
    external fun focusOut()

    /**
     * Forward the results from a permission request.
     * @see Activity.onRequestPermissionsResult
     * @param permission Request permission
     * @param result True if the permission was granted, false otherwise
     */
    @JvmStatic
    external fun requestPermissionResult(permission: String?, result: Boolean)
}
