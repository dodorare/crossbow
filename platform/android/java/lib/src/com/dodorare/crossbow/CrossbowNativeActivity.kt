package com.dodorare.crossbow

import android.util.Log
import android.os.Bundle
import android.app.Fragment
import android.app.Activity
import android.content.Intent
import android.app.NativeActivity
import android.content.pm.PackageManager
import android.widget.FrameLayout
import android.widget.FrameLayout.LayoutParams
import androidx.annotation.CallSuper

open class CrossbowNativeActivity : NativeActivity(), CrossbowHost {
    companion object {
        const val CONTENT_VIEW_ID = 10101010

        init {
            // This is necessary when any of the following happens:
            //     - crossbow_android library is not configured to the following line in the manifest:
            //        <meta-data android:name="android.app.lib_name" android:value="crossbow_android" />
            //     - GameActivity derived class calls to the native code before calling
            //       the super.onCreate() function.
            System.loadLibrary("crossbow_android")
        }
    }
	private var crossbowFragment: Crossbow? = null

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

        val frame = FrameLayout(this)
        frame.setId(CONTENT_VIEW_ID)
        setContentView(frame, LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT))

        if (savedInstanceState === null) {
            crossbowFragment = Crossbow()
            fragmentManager.beginTransaction().add(CONTENT_VIEW_ID, crossbowFragment).commit()
        }
	}

    override fun onDestroy() {
        Log.v(TAG, "Destroying Crossbow app...")
        super.onDestroy()
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        if (crossbowFragment !== null) {
            crossbowFragment?.onNewIntent(intent)
        }
    }

    @CallSuper
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (crossbowFragment !== null) {
            crossbowFragment?.onActivityResult(requestCode, resultCode, data)
        }
    }

    @CallSuper
    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (crossbowFragment !== null) {
            crossbowFragment?.onRequestPermissionsResult(requestCode, permissions, grantResults)
        }
    }

    override fun onBackPressed() {
        if (crossbowFragment !== null) {
            crossbowFragment?.onBackPressed()
        } else {
            super.onBackPressed()
        }
    }
}
