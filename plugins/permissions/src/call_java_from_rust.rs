use jni::{
    objects::{JClass, JObject, JString, JValue},
    JNIEnv,
};

pub fn rust_result_to_java_result<'e, T>(
    env: &JNIEnv<'e>,
    result: crate::error::Result<T>,
) -> JObject<'e>
where
    T: std::fmt::Display,
{
    let (is_ok, value) = match result {
        Ok(v) => (true, format!("{}", v)),
        Err(e) => (false, format!("{:?}", e)),
    };
    create_java_result(env, is_ok, &value)
}

pub fn actually_do_stuff<'a>(env: JNIEnv<'a>, code: JString) -> crate::error::Result<()> {
    let code = String::from(env.get_string(code).unwrap());
    let intermediate_value = string(code)?;
    println!("The result is {}", intermediate_value);
    Ok(())
}

#[no_mangle]
pub extern "C" fn Java_com_startup_hip_RustCode_doStuff<'a>(
    env: JNIEnv<'a>,
    _class: JClass,
    code: JString,
) -> JString<'a> {
    let code_rust = String::from(env.get_string(code).unwrap());
    let result = match string(code_rust) {
        Ok(value) => format!("OK {}", value),
        Err(e) => format!("ER {:?}", e),
    };
    return env.new_string(result).unwrap();
}

pub fn create_java_result<'e>(env: &JNIEnv<'e>, is_ok: bool, value: &str) -> JObject<'e> {
    let class = env.find_class("com/permissions/Result").unwrap();
    let args: [JValue<'e>; 2] = [
        JValue::Bool(u8::from(is_ok)),
        JValue::Object(JObject::from(env.new_string(value).unwrap())),
    ];
    env.new_object(class, "(ZLjava/lang/String;)V", &args)
        .unwrap()
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
        let arg = String::from("Call java from rust");
        let result = super::string(arg).unwrap();
        assert_eq!(result, "Call java from rust")
    }
}
