package com.dodorare.crossbow;

import android.app.Activity;

public class CrossbowLib {
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
	 * Forward the results from a permission request.
	 * @see Activity#onRequestPermissionsResult(int, String[], int[])
	 * @param p_permission Request permission
	 * @param p_result True if the permission was granted, false otherwise
	 */
	public static native void requestPermissionResult(String p_permission, boolean p_result);
}
