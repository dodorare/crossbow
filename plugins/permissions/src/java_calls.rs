use jni::{
    objects::{JClass, JObject, JString, JValue},
    JNIEnv,
};

use crate::error::Result;

fn rust_result_to_java_result<'e, T>(env: &JNIEnv<'e>, result: Result<T>) -> JObject<'e>
where
    T: std::fmt::Display,
{
    let (is_ok, value) = match result {
        Ok(v) => (true, format!("{}", v)),
        Err(e) => (false, format!("{:?}", e)),
    };

    /*
     * We're about to construct a Java class from JNI.
     * To do this, we need three things:
     *
     * 1) Get the class handle from JNI. Simple enough, as we just need to
     *    call env.find_class() with the fully qualified class name
     *    as the parameter. This name is basically "$PACKAGE.$CLASS",
     *    except that all the dots ('.') are replaced by slashes ('/').
     *
     *    JNI docs state that find_class() is a rather slow call,
     *    so if we expect to be calling the Rust code from Java a lot,
     *    it might be a good idea to add some caching mechanism here.
     *
     * 2) Get the constructor signature. Remember that Java allows overloading
     *    not only "normal" methods, but also constructors. Since a class
     *    can have several constructors, we need to tell JNI which one
     *    we want to use.
     *
     *    A simple way to learn function signatures is to compile
     *    the class we're intersted in, and then use the javap tool:
     *    $ cd android/app/src/main/java/pl/svgames/blog/RustOnAndroid/
     *    $ javac ./Result.java
     *    $ javap -s ./Result.class
     *
     * 3) Create the array holding arguments to the constructor.
     *    Our Result class's constructor expects two:
     *    1. An "is_ok" boolean - true for success, false for error
     *    2. A string holding either the success value,
     *       or the error message.
     *
     * Once we got all three points covered, we just call env.new_object()
     * and hope for the best.
     */
    let class = env
        .find_class("com/crossbow/JavaResult")
        .unwrap();
    let args: [JValue<'e>; 2] = [
        JValue::Bool(u8::from(is_ok)),
        JValue::Object(JObject::from(env.new_string(value).unwrap())),
    ];
    env.new_object(class, "(ZLjava/lang/String;)V", &args)
        .unwrap()
}

#[no_mangle]
pub extern "system" fn Java_pl_svgames_blog_RustOnAndroid_RpnCalculator_rpn<'e>(
    env: JNIEnv<'e>,
    _class: JClass,
    expression: JString,
) -> JObject<'e> {
    rust_result_to_java_result(&env, rpn_wrapper(&env, expression))
}
