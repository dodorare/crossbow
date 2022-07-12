package com.crossbow.library

// import android.app.Activity
// import com.google.androidgamesdk.GameActivity

// open class CrossbowGameActivity : GameActivity() {
//     companion object {
//         init {
//             // This is necessary when any of the following happens:
//             //     - crossbow_android library is not configured to the following line in the manifest:
//             //        <meta-data android:name="android.app.lib_name" android:value="crossbow_android" />
//             //     - GameActivity derived class calls to the native code before calling
//             //       the super.onCreate() function.
//             System.loadLibrary("crossbow_android")
//         }
//     }

// 	override fun onCreate(savedInstanceState: Bundle?) {
// 		super.onCreate(savedInstanceState)
// 		setContentView(R.layout.crossbow_app_layout)

// 		val currentFragment: Fragment =
//             fragmentManager.findFragmentById(R.id.crossbow_fragment_container)
// 		if (currentFragment is Crossbow) {
// 			Log.v(TAG, "Reusing existing Crossbow fragment instance.")
// 			crossbowFragment = currentFragment as Crossbow
// 		} else {
// 		    Log.v(TAG, "Creating new Crossbow fragment instance")
// 			crossbowFragment = Crossbow()
//             getFragmentManager().beginTransaction()
//                 .replace(R.id.crossbow_fragment_container, crossbowFragment as Fragment)
//                 .setPrimaryNavigationFragment(crossbowFragment as Fragment).commitNowAllowingStateLoss()
// 		}
// 	}
// }
