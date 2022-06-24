package com.dodorare.crossbow;

import android.app.Activity;

/**
 * Wrapper for native library
 */
public class CrossbowLib extends GameActivity {
	static {
        // Optional: reload the native library.
        // However this is necessary when any of the following happens:
        //     - crossbow_android library is not configured to the following line in the manifest:
        //        <meta-data android:name="android.app.lib_name" android:value="crossbow_android" />
        //     - GameActivity derived class calls to the native code before calling
        //       the super.onCreate() function.
		System.loadLibrary("crossbow_android");
	}

	/**
	 * Invoked on the main thread to initialize Crossbow native layer.
	 */
	public static native void initialize(Activity activity, Crossbow p_instance, Object p_asset_manager, boolean use_apk_expansion);

	/**
	 * Invoked on the main thread to clean up Crossbow native layer.
	 * @see androidx.fragment.app.Fragment#onDestroy()
	 */
	public static native void ondestroy();

	/**
	 * Forward the results from a permission request.
	 * @see Activity#onRequestPermissionsResult(int, String[], int[])
	 * @param p_permission Request permission
	 * @param p_result True if the permission was granted, false otherwise
	 */
	public static native void requestPermissionResult(String p_permission, boolean p_result);
}
