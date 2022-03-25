use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

#[no_mangle]
pub extern "C" fn Java_com_startup_hip_RustCode_doStuff<'a>(
    env: JNIEnv<'a>,
    _class: JClass,
    code: JString,
) -> JString<'a> {
    let code_rust = String::from(env.get_string(code).unwrap());
    // TODO: Impl the rust function that return String
    let result = match some_rust_function(code_rust) {
        Ok(value) => format!("OK {}", value),
        Err(e) => format!("ER {:?}", e),
    };
    return env.new_string(result).unwrap();
}

pub fn string(arg: String) -> crate::error::Result<String> {
    let mut string = String::new();
    string.push_str(&arg);
    Ok(string)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_function() {
        let arg = String::from("Call rust from java");
        let result = super::string(arg).unwrap();
        // TODO: use assert macro instead of println
        println!("Result of function is {}", result);
    }
}
