package com.crossbow.library

import android.app.NativeActivity
import android.content.Intent
import android.os.Bundle
import android.widget.FrameLayout

open class CrossbowNativeActivity : NativeActivity() {
    protected lateinit var crossbow: Crossbow

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val content = FrameLayout(this)
        setContentView(content)
        crossbow = Crossbow(this)
        content.addView(crossbow.view)
    }

    @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
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

    @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
    override fun onBackPressed() {
        if (!crossbow.onBackPressed()) super.onBackPressed()
    }
}
