// TODO: Add enum with all android permissions
// TODO: Add JNI calls to request permissions

#[cfg(target_os = "android")]
pub fn request_permission() {}

/*
/**
 * \brief Gets the internal name for an android permission.
 * \param[in] lJNIEnv a pointer to the JNI environment
 * \param[in] perm_name the name of the permission, e.g.,
 *   "READ_EXTERNAL_STORAGE", "WRITE_EXTERNAL_STORAGE".
 * \return a jstring with the internal name of the permission,
 *   to be used with android Java functions
 *   Context.checkSelfPermission() or Activity.requestPermissions()
 */
 jstring android_permission_name(JNIEnv* lJNIEnv, const char* perm_name) {
    // nested class permission in class android.Manifest,
    // hence android 'slash' Manifest 'dollar' permission
    jclass ClassManifestpermission = lJNIEnv->FindClass(
       "android/Manifest$permission"
    );
    jfieldID lid_PERM = lJNIEnv->GetStaticFieldID(
       ClassManifestpermission, perm_name, "Ljava/lang/String;"
    );
    jstring ls_PERM = (jstring)(lJNIEnv->GetStaticObjectField(
        ClassManifestpermission, lid_PERM
    ));
    return ls_PERM;
}

/**
 * \brief Tests whether a permission is granted.
 * \param[in] app a pointer to the android app.
 * \param[in] perm_name the name of the permission, e.g.,
 *   "READ_EXTERNAL_STORAGE", "WRITE_EXTERNAL_STORAGE".
 * \retval true if the permission is granted.
 * \retval false otherwise.
 * \note Requires Android API level 23 (Marshmallow, May 2015)
 */
bool android_has_permission(struct android_app* app, const char* perm_name) {
    JavaVM* lJavaVM = app->activity->vm;
    JNIEnv* lJNIEnv = nullptr;
    bool lThreadAttached = false;

    // Get JNIEnv from lJavaVM using GetEnv to test whether
    // thread is attached or not to the VM. If not, attach it
    // (and note that it will need to be detached at the end
    //  of the function).
    switch (lJavaVM->GetEnv((void**)&lJNIEnv, JNI_VERSION_1_6)) {
      case JNI_OK:
        break;
      case JNI_EDETACHED: {
        jint lResult = lJavaVM->AttachCurrentThread(&lJNIEnv, nullptr);
        if(lResult == JNI_ERR) {
          throw std::runtime_error("Could not attach current thread");
        }
        lThreadAttached = true;
      } break;
      case JNI_EVERSION:
        throw std::runtime_error("Invalid java version");
    }

    bool result = false;

    jstring ls_PERM = android_permission_name(lJNIEnv, perm_name);

    jint PERMISSION_GRANTED = jint(-1);
    {
       jclass ClassPackageManager = lJNIEnv->FindClass(
          "android/content/pm/PackageManager"
       );
       jfieldID lid_PERMISSION_GRANTED = lJNIEnv->GetStaticFieldID(
          ClassPackageManager, "PERMISSION_GRANTED", "I"
       );
       PERMISSION_GRANTED = lJNIEnv->GetStaticIntField(
          ClassPackageManager, lid_PERMISSION_GRANTED
       );
    }
    {
       jobject activity = app->activity->clazz;
       jclass ClassContext = lJNIEnv->FindClass(
          "android/content/Context"
       );
       jmethodID MethodcheckSelfPermission = lJNIEnv->GetMethodID(
          ClassContext, "checkSelfPermission", "(Ljava/lang/String;)I"
       );
       jint int_result = lJNIEnv->CallIntMethod(
           activity, MethodcheckSelfPermission, ls_PERM
       );
       result = (int_result == PERMISSION_GRANTED);
    }

    if(lThreadAttached) {
      lJavaVM->DetachCurrentThread();
    }

    return result;
}

/**
 * \brief Query file permissions.
 * \details This opens the system dialog that lets the user
 *  grant (or deny) the permission.
 * \param[in] app a pointer to the android app.
 * \note Requires Android API level 23 (Marshmallow, May 2015)
 */
void android_request_file_permissions(struct android_app* app) {
    JavaVM* lJavaVM = app->activity->vm;
    JNIEnv* lJNIEnv = nullptr;
    bool lThreadAttached = false;

    // Get JNIEnv from lJavaVM using GetEnv to test whether
    // thread is attached or not to the VM. If not, attach it
    // (and note that it will need to be detached at the end
    //  of the function).
    switch (lJavaVM->GetEnv((void**)&lJNIEnv, JNI_VERSION_1_6)) {
      case JNI_OK:
        break;
      case JNI_EDETACHED: {
        jint lResult = lJavaVM->AttachCurrentThread(&lJNIEnv, nullptr);
        if(lResult == JNI_ERR) {
          throw std::runtime_error("Could not attach current thread");
        }
        lThreadAttached = true;
      } break;
      case JNI_EVERSION:
        throw std::runtime_error("Invalid java version");
      }

    jobjectArray perm_array = lJNIEnv->NewObjectArray(
      2,
      lJNIEnv->FindClass("java/lang/String"),
      lJNIEnv->NewStringUTF("")
    );

    lJNIEnv->SetObjectArrayElement(
      perm_array, 0,
      android_permission_name(lJNIEnv, "READ_EXTERNAL_STORAGE")
    );

    lJNIEnv->SetObjectArrayElement(
      perm_array, 1,
      android_permission_name(lJNIEnv, "WRITE_EXTERNAL_STORAGE")
    );

    jobject activity = app->activity->clazz;

    jclass ClassActivity = lJNIEnv->FindClass(
       "android/app/Activity"
    );

    jmethodID MethodrequestPermissions = lJNIEnv->GetMethodID(
       ClassActivity, "requestPermissions", "([Ljava/lang/String;I)V"
    );

    // Last arg (0) is just for the callback (that I do not use)
    lJNIEnv->CallVoidMethod(
       activity, MethodrequestPermissions, perm_array, 0
    );

    if(lThreadAttached) {
       lJavaVM->DetachCurrentThread();
    }
}

void check_android_permissions(struct android_app* app) {
    bool OK = android_has_permission(
       app, "READ_EXTERNAL_STORAGE"
    ) && android_has_permission(
       app, "WRITE_EXTERNAL_STORAGE"
    );
    if(!OK) {
       android_request_file_permissions(app);
    }
}
*/
