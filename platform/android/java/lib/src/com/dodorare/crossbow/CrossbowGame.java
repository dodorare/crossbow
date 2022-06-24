package com.dodorare.crossbow;

import android.content.Intent;
import android.os.Bundle;
import android.util.Log;

import com.google.androidgamesdk.GameActivity;
import androidx.annotation.CallSuper;
import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.core.view.WindowCompat;

/**
 * Base activity for Android Games intending to use GameActivity as the primary and only screen.
 */
public class CrossbowGame extends CrossbowLib {
	private static final String TAG = CrossbowApp.class.getSimpleName();

	@Override
	public void onCreate(Bundle savedInstanceState) {
        // When true, the app will fit inside any system UI windows.
        // When false, we render behind any system UI windows.
        WindowCompat.setDecorFitsSystemWindows(getWindow(), false);
        hideSystemUI();
        // You can set IME fields here or in native code using GameActivity_setImeEditorInfoFields.
        // We set the fields in native_engine.cpp.
        // super.setImeEditorInfoFields(InputType.TYPE_CLASS_TEXT,
        //     IME_ACTION_NONE, IME_FLAG_NO_FULLSCREEN );
        super.onCreate(savedInstanceState);

		Log.v(TAG, "Called onCreated method.");
	}

	@Override
	public void onDestroy() {
		Log.v(TAG, "Destroying Crossbow app...");
		super.onDestroy();
		// onCrossbowForceQuit(crossbowFragment);
	}

	// @Override
	// public final void onCrossbowForceQuit(Crossbow instance) {
	// 	if (instance == crossbowFragment) {
	// 		Log.v(TAG, "Force quitting Crossbow instance");
	// 		ProcessPhoenix.forceQuit(this);
	// 	}
	// }

	// @Override
	// public final void onCrossbowRestartRequested(Crossbow instance) {
	// 	if (instance == crossbowFragment) {
	// 		// It's very hard to properly de-initialize Crossbow on Android to restart the game
	// 		// from scratch. Therefore, we need to kill the whole app process and relaunch it.
	// 		//
	// 		// Restarting only the activity, wouldn't be enough unless it did proper cleanup (including
	// 		// releasing and reloading native libs or resetting their state somehow and clearing statics).
	// 		Log.v(TAG, "Restarting Crossbow instance...");
	// 		ProcessPhoenix.triggerRebirth(this);
	// 	}
	// }

	// @Override
	// public void onNewIntent(Intent intent) {
	// 	super.onNewIntent(intent);
	// 	if (crossbowFragment != null) {
	// 		crossbowFragment.onNewIntent(intent);
	// 	}
	// }

	// @CallSuper
	// @Override
	// public void onActivityResult(int requestCode, int resultCode, Intent data) {
	// 	super.onActivityResult(requestCode, resultCode, data);
	// 	if (crossbowFragment != null) {
	// 		crossbowFragment.onActivityResult(requestCode, resultCode, data);
	// 	}
	// }

	@CallSuper
	@Override
	public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
		super.onRequestPermissionsResult(requestCode, permissions, grantResults);
		if (crossbowFragment != null) {
			crossbowFragment.onRequestPermissionsResult(requestCode, permissions, grantResults);
		}
	}

	// @Override
	// public void onBackPressed() {
	// 	if (crossbowFragment != null) {
	// 		crossbowFragment.onBackPressed();
	// 	} else {
	// 		super.onBackPressed();
	// 	}
	// }

	/**
	 * Used to initialize the Crossbow fragment instance in {@link CrossbowApp#onCreate(Bundle)}.
	 */
	@NonNull
	protected Crossbow initCrossbowInstance() {
		return new Crossbow();
	}

	@Nullable
	protected final Crossbow getCrossbowFragment() {
		return crossbowFragment;
	}
}
