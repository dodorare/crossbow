package com.dodorare.crossbow;

import android.app.Activity;
import android.app.NativeActivity;
import android.util.Log;
import android.os.Bundle;
import android.content.pm.PackageManager;
import androidx.core.app.ActivityCompat;
import androidx.annotation.Nullable;
import androidx.annotation.CallSuper;

/**
 * Wrapper for NativeActivity and native library.
 */
public class CrossbowNativeActivity extends NativeActivity implements ActivityCompat.OnRequestPermissionsResultCallback {
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
	}

	@Override
    public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
		super.onRequestPermissionsResult(requestCode, permissions, grantResults);
		// for (CrossbowPlugin plugin : pluginRegistry.getAllPlugins()) {
		// 	plugin.onMainRequestPermissionsResult(requestCode, permissions, grantResults);
		// }

		for (int i = 0; i < permissions.length; i++) {
			// Log.v(TAG, "Permission granted");
			crossbowInstance.requestPermissionResult(permissions[i], grantResults[i] == PackageManager.PERMISSION_GRANTED);
		}
	}
}
