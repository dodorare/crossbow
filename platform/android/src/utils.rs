use jni::{errors::Result, objects::JString, JNIEnv};
use std::ffi::CStr;

pub fn jstring_to_string(env: &JNIEnv, jstring: JString) -> Result<String> {
    let utf_chars = env.get_string_utf_chars(jstring)?;
    // Construct a `String` from that char buffer.
    let string = unsafe { CStr::from_ptr(utf_chars).to_str().unwrap().to_string() };
    env.release_string_utf_chars(jstring, utf_chars)?;
    Ok(string)
}

pub fn get_jni_sig(p_type: &str) -> &'static str {
    let name_sigs = [
        ("void", "V"),
        ("boolean", "Z"),
        ("int", "I"),
        ("float", "F"),
        ("double", "D"),
        ("java.lang.String", "Ljava/lang/String;"),
        (
            "org.godotengine.godot.Dictionary",
            "Lorg/godotengine/godot/Dictionary;",
        ),
        ("[I", "[I"),
        ("[B", "[B"),
        ("[F", "[F"),
        ("[Ljava.lang.String;", "[Ljava/lang/String;"),
        // (nullptr, "V")
    ];
    for name_sig in name_sigs {
        if p_type == name_sig.0 {
            return name_sig.1;
        }
    }
    "Ljava/lang/Object;"
}
