package com.dodorare.crossbow;

import android.app.Activity;
import com.google.androidgamesdk.GameActivity;

/**
 * Wrapper for GameActivity and native library.
 */
public class CrossbowGameActivity extends GameActivity {
	static {
        // Optional: reload the native library.
        // However this is necessary when any of the following happens:
        //     - crossbow_android library is not configured to the following line in the manifest:
        //        <meta-data android:name="android.app.lib_name" android:value="crossbow_android" />
        //     - GameActivity derived class calls to the native code before calling
        //       the super.onCreate() function.
		System.loadLibrary("crossbow_android");
	}
}
