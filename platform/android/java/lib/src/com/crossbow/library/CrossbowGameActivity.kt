package com.crossbow.library

import android.content.Intent
import android.os.Bundle
import android.view.ViewGroup
import androidx.activity.addCallback
import com.google.androidgamesdk.GameActivity

/** GameActivity host that overlays Crossbow plugin UI without replacing the game SurfaceView. */
open class CrossbowGameActivity : GameActivity() {
    protected lateinit var crossbow: Crossbow

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        crossbow = Crossbow(this)
        findViewById<ViewGroup>(contentViewId).addView(crossbow.view)
        onBackPressedDispatcher.addCallback(this) {
            if (!crossbow.onBackPressed()) finish()
        }
    }

    @Suppress("DEPRECATION")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        crossbow.onActivityResult(requestCode, resultCode, data)
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        crossbow.onRequestPermissionsResult(requestCode, permissions, grantResults)
    }

    override fun onPause() {
        crossbow.onPause()
        super.onPause()
    }

    override fun onResume() {
        super.onResume()
        if (::crossbow.isInitialized) crossbow.onResume()
    }

    override fun onDestroy() {
        if (::crossbow.isInitialized) crossbow.onDestroy()
        super.onDestroy()
    }
}
