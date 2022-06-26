package com.dodorare.crossbow;

import android.app.Activity;
import android.app.NativeActivity;
import android.util.Log;
import android.os.Bundle;
import androidx.annotation.Nullable;

/**
 * Wrapper for NativeActivity and native library.
 */
public class CrossbowNativeActivity extends NativeActivity {
	static {
        // Optional: reload the native library.
        // However this is necessary when any of the following happens:
        //     - crossbow_android library is not configured to the following line in the manifest:
        //        <meta-data android:name="android.app.lib_name" android:value="crossbow_android" />
        //     - GameActivity derived class calls to the native code before calling
        //       the super.onCreate() function.
		System.loadLibrary("crossbow_android");
	}

	private static final String TAG = CrossbowNativeActivity.class.getSimpleName();

	@Nullable
	private CrossbowLib crossbowInstance;

	@Override
	public void onCreate(Bundle savedInstanceState) {
		super.onCreate(savedInstanceState);

		Log.v(TAG, "Creating new CrossbowLib Instance");
		crossbowInstance = new CrossbowLib();

		crossbowInstance.requestPermissionResult("android.permission.INTERNET", true);
	}
}
