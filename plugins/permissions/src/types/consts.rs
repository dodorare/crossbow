// JNI classes
pub const ANDROID_MANIFEST_PERMISSION: &str = "android/Manifest$permission";
pub const ANDROID_CONTEXT: &str = "android/content/Context";
pub const ANDROID_PACKAGE_MANAGER: &str = "android/content/pm/PackageManager";
pub const ANDROID_ACTIVITY: &str = "android/app/Activity";

// JNI methods
pub const REQUEST_PERMISSIONS_METHOD: &str = "requestPermissions";
pub const CHECK_SELF_PERMISSION_METHOD: &str = "checkSelfPermission";

// JNI signatures
pub const JAVA_STRING_SIGNATURE: &str = "java/lang/String";
pub const MANIFEST_PERMISSION_SIGNATURE: &str = "Ljava/lang/String;";
pub const REQUEST_PERMISSIONS_SIGNATURE: &str = "([Ljava/lang/String;I)V";
pub const CHECK_SELF_PERMISSION_SIGNATURE: &str = "(Ljava/lang/String;)I";
pub const PRIMITIVE_INT_SIGNATURE: &str = "I";

// JNI static fields
pub const PERMISSIONS_GRANTED: &str = "PERMISSION_GRANTED";
pub const PERMISSION_DENIED: &str = "PERMISSION_DENIED";

// JNI types
pub const ARRAY_LENGTH: i32 = 1;
pub const OBJECT_INDEX: i32 = 0;
