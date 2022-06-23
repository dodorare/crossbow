package com.dodorare.crossbow;

import android.content.Intent;
import android.os.Bundle;
import android.util.Log;

import androidx.annotation.CallSuper;
import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.fragment.app.Fragment;
import androidx.fragment.app.FragmentActivity;

/**
 * Base activity for Android apps intending to use Crossbow as the primary and only screen.
 */
public class CrossbowApp extends FragmentActivity {
	private static final String TAG = CrossbowApp.class.getSimpleName();

	@Nullable
	private Crossbow crossbowFragment;

	@Override
	public void onCreate(Bundle savedInstanceState) {
		super.onCreate(savedInstanceState);
		setContentView(R.layout.crossbow_app_layout);

		Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.crossbow_fragment_container);
		if (currentFragment instanceof Crossbow) {
			Log.v(TAG, "Reusing existing Crossbow fragment instance.");
			crossbowFragment = (Crossbow)currentFragment;
		} else {
			Log.v(TAG, "Creating new Crossbow fragment instance.");
			crossbowFragment = initCrossbowInstance();
			getSupportFragmentManager().beginTransaction().replace(R.id.crossbow_fragment_container, crossbowFragment).setPrimaryNavigationFragment(crossbowFragment).commitNowAllowingStateLoss();
		}
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
