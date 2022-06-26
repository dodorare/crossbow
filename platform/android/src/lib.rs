use jni::{
    objects::{JClass, JString},
    sys::jboolean,
    JNIEnv,
};

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C" fn Java_com_dodorare_crossbow_CrossbowLib_requestPermissionResult(
    _env: JNIEnv,
    _class: JClass,
    _p_permission: JString,
    p_result: jboolean,
) {
    println!("p_result: {:?}", p_result);
}

pub fn init() {
    println!("init");
}
