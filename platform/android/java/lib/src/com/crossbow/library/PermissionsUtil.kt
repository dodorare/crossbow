package com.crossbow.library

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import android.content.pm.PermissionInfo
import android.net.Uri
import android.os.Build
import android.os.Environment
import android.provider.Settings
import android.util.Log
import kotlin.collections.List
import androidx.core.content.ContextCompat

/**
 * This class includes utility functions for Android permissions related operations.
 */
object PermissionsUtil {
    private val TAG = PermissionsUtil::class.java.simpleName
    const val REQUEST_RECORD_AUDIO_PERMISSION = 1
    const val REQUEST_CAMERA_PERMISSION = 2
    const val REQUEST_VIBRATE_PERMISSION = 3
    const val REQUEST_ALL_PERMISSION_REQ_CODE = 1001
    const val REQUEST_MANAGE_EXTERNAL_STORAGE_REQ_CODE = 2002

    /**
     * Request a dangerous permission. name must be specified in [this](https://github.com/aosp-mirror/platform_frameworks_base/blob/master/core/res/AndroidManifest.xml)
     * @param name the name of the requested permission.
     * @param activity the caller activity for this method.
     * @return true/false. "true" if permission was granted otherwise returns "false".
     */
    fun requestPermission(name: String, activity: Activity): Boolean {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) {
            // Not necessary, asked on install already
            return true
        }
        if (name == "RECORD_AUDIO" && ContextCompat.checkSelfPermission(
                activity,
                Manifest.permission.RECORD_AUDIO
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            activity.requestPermissions(
                arrayOf(Manifest.permission.RECORD_AUDIO),
                REQUEST_RECORD_AUDIO_PERMISSION
            )
            return false
        }
        if (name == "CAMERA" && ContextCompat.checkSelfPermission(
                activity,
                Manifest.permission.CAMERA
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            activity.requestPermissions(
                arrayOf(Manifest.permission.CAMERA),
                REQUEST_CAMERA_PERMISSION
            )
            return false
        }
        if (name == "VIBRATE" && ContextCompat.checkSelfPermission(
                activity,
                Manifest.permission.VIBRATE
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            activity.requestPermissions(
                arrayOf(Manifest.permission.VIBRATE),
                REQUEST_VIBRATE_PERMISSION
            )
            return false
        }
        return true
    }

    /**
     * Request dangerous permissions which are defined in the Android manifest file from the user.
     * @param activity the caller activity for this method.
     * @return true/false. "true" if all permissions were granted otherwise returns "false".
     */
    fun requestManifestPermissions(activity: Activity): Boolean {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) {
            return true
        }
        val manifestPermissions: Array<String>
        manifestPermissions = try {
            getManifestPermissions(activity)
        } catch (e: PackageManager.NameNotFoundException) {
            e.printStackTrace()
            return false
        }
        if (manifestPermissions.size == 0) return true
        val requestedPermissions: MutableList<String> = ArrayList()
        for (manifestPermission in manifestPermissions) {
            try {
                if (manifestPermission == Manifest.permission.MANAGE_EXTERNAL_STORAGE) {
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && !Environment.isExternalStorageManager()) {
                        try {
                            val intent =
                                Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION)
                            intent.setData(
                                Uri.parse(
                                    String.format(
                                        "package:%s",
                                        activity.getPackageName()
                                    )
                                )
                            )
                            activity.startActivityForResult(
                                intent,
                                REQUEST_MANAGE_EXTERNAL_STORAGE_REQ_CODE
                            )
                        } catch (ignored: Exception) {
                            val intent = Intent(Settings.ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION)
                            activity.startActivityForResult(
                                intent,
                                REQUEST_MANAGE_EXTERNAL_STORAGE_REQ_CODE
                            )
                        }
                    }
                } else {
                    val permissionInfo: PermissionInfo = getPermissionInfo(activity, manifestPermission)
                    val protectionLevel = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                        permissionInfo.getProtection()
                    } else {
                        @Suppress("DEPRECATION")
                        permissionInfo.protectionLevel
                    }
                    if (protectionLevel == PermissionInfo.PROTECTION_DANGEROUS && ContextCompat.checkSelfPermission(
                            activity,
                            manifestPermission
                        ) != PackageManager.PERMISSION_GRANTED
                    ) {
                        requestedPermissions.add(manifestPermission)
                    }
                }
            } catch (e: PackageManager.NameNotFoundException) {
                // Skip this permission and continue.
                Log.w(TAG, "Unable to identify permission $manifestPermission", e)
            }
        }
        if (requestedPermissions.isEmpty()) {
            // If list is empty, all of dangerous permissions were granted.
            return true
        }
        activity.requestPermissions(
            requestedPermissions.toTypedArray(),
            REQUEST_ALL_PERMISSION_REQ_CODE
        )
        return false
    }

    /**
     * With this function you can get the list of dangerous permissions that have been granted to the Android application.
     * @param activity the caller activity for this method.
     * @return granted permissions list
     */
    fun getGrantedPermissions(activity: Activity): Array<String> {
        val manifestPermissions: Array<String>
        manifestPermissions = try {
            getManifestPermissions(activity)
        } catch (e: PackageManager.NameNotFoundException) {
            e.printStackTrace()
            return arrayOf<String>()
        }
        if (manifestPermissions.size == 0) return manifestPermissions
        val grantedPermissions: MutableList<String> = ArrayList()
        for (manifestPermission in manifestPermissions) {
            try {
                if (manifestPermission == Manifest.permission.MANAGE_EXTERNAL_STORAGE) {
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && Environment.isExternalStorageManager()) {
                        grantedPermissions.add(manifestPermission)
                    }
                } else {
                    val permissionInfo: PermissionInfo = getPermissionInfo(activity, manifestPermission)
                    val protectionLevel = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                        permissionInfo.getProtection()
                    } else {
                        @Suppress("DEPRECATION")
                        permissionInfo.protectionLevel
                    }
                    if (protectionLevel == PermissionInfo.PROTECTION_DANGEROUS && ContextCompat.checkSelfPermission(
                            activity,
                            manifestPermission
                        ) == PackageManager.PERMISSION_GRANTED
                    ) {
                        grantedPermissions.add(manifestPermission)
                    }
                }
            } catch (e: PackageManager.NameNotFoundException) {
                // Skip this permission and continue.
                Log.w(TAG, "Unable to identify permission $manifestPermission", e)
            }
        }
        return grantedPermissions.toTypedArray()
    }

    /**
     * Check if the given permission is in the AndroidManifest.xml file.
     * @param activity the caller activity for this method.
     * @param permission the permission to look for in the manifest file.
     * @return "true" if the permission is in the manifest file of the activity, "false" otherwise.
     */
    fun hasManifestPermission(activity: Activity, permission: String): Boolean {
        try {
            for (p in getManifestPermissions(activity)) {
                if (permission == p) return true
            }
        } catch (ignored: PackageManager.NameNotFoundException) {
        }
        return false
    }

    /**
     * Returns the permissions defined in the AndroidManifest.xml file.
     * @param activity the caller activity for this method.
     * @return manifest permissions list
     * @throws PackageManager.NameNotFoundException the exception is thrown when a given package, application, or component name cannot be found.
     */
    @Throws(PackageManager.NameNotFoundException::class)
    private fun getManifestPermissions(activity: Activity): Array<String> {
        val packageManager: PackageManager = activity.getPackageManager()
        val packageInfo: PackageInfo =
            packageManager.getPackageInfo(activity.getPackageName(), PackageManager.GET_PERMISSIONS)
        return if (packageInfo.requestedPermissions == null) arrayOf<String>() else packageInfo.requestedPermissions
    }

    /**
     * Returns the information of the desired permission.
     * @param activity the caller activity for this method.
     * @param permission the name of the permission.
     * @return permission info object
     * @throws PackageManager.NameNotFoundException the exception is thrown when a given package, application, or component name cannot be found.
     */
    @Throws(PackageManager.NameNotFoundException::class)
    private fun getPermissionInfo(activity: Activity, permission: String): PermissionInfo {
        val packageManager: PackageManager = activity.getPackageManager()
        return packageManager.getPermissionInfo(permission, 0)
    }
}
