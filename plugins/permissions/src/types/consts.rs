// JNI classes
pub const ANDROID_MANIFEST_PERMISSION: &'static str = "android/Manifest$permission";
pub const ANDROID_CONTEXT: &'static str = "android/content/Context";
pub const ANDROID_PACKAGE_MANAGER: &'static str = "android/content/pm/PackageManager";
pub const ANDROID_ACTIVITY: &'static str = "android/app/Activity";

// JNI methods
pub const REQUEST_PERMISSIONS_METHOD: &'static str = "requestPermissions";
pub const CHECK_SELF_PERMISSION_METHOD: &'static str = "checkSelfPermission";

// JNI signatures
pub const JAVA_STRING_SIGNATURE: &'static str = "java/lang/String";
pub const MANIFEST_PERMISSION_SIGNATURE: &'static str = "Ljava/lang/String;";
pub const REQUEST_PERMISSIONS_SIGNATURE: &'static str = "([Ljava/lang/String;I)V";
pub const CHECK_SELF_PERMISSION_SIGNATURE: &'static str = "(Ljava/lang/String;)I";
pub const PRIMITIVE_INT_SIGNATURE: &'static str = "I";

// JNI static fields
pub const PERMISSIONS_GRANTED: &'static str = "PERMISSION_GRANTED";

// JNI types
pub const ARRAY_LENGTH: i32 = 1;
pub const OBJECT_INDEX: i32 = 0;
