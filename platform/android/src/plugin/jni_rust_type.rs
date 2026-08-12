use crate::{error::*, utils::*};
use jni::{
    jni_sig, jni_str,
    objects::{JByteArray, JDoubleArray, JFloatArray, JIntArray, JObject, JObjectArray, JString},
    Env,
};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub enum JniRustType {
    Void,
    String(String),
    StringArray(Vec<String>),
    Boolean(bool),
    Int(i64),
    IntArray(Vec<i64>),
    ByteArray(Vec<u8>),
    Float(f32),
    Double(f64),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    ObjectArray(Vec<JniRustType>),
    Map(HashMap<String, JniRustType>),
}

impl JniRustType {
    /// Try to unwrap to Void.
    pub fn into_void(self) -> Option<()> {
        match self {
            Self::Void => Some(()),
            _ => None,
        }
    }

    /// Try to unwrap to Boolean.
    pub fn into_bool(self) -> Option<bool> {
        match self {
            Self::Boolean(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to String.
    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to StringArray.
    pub fn into_string_array(self) -> Option<Vec<String>> {
        match self {
            Self::StringArray(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to Int.
    pub fn into_int(self) -> Option<i64> {
        match self {
            Self::Int(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to IntArray.
    pub fn into_int_array(self) -> Option<Vec<i64>> {
        match self {
            Self::IntArray(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to ByteArray.
    pub fn into_byte_array(self) -> Option<Vec<u8>> {
        match self {
            Self::ByteArray(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to Float.
    pub fn into_float(self) -> Option<f32> {
        match self {
            Self::Float(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to Double.
    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to FloatArray.
    pub fn into_float_array(self) -> Option<Vec<f32>> {
        match self {
            Self::FloatArray(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to DoubleArray.
    pub fn into_double_array(self) -> Option<Vec<f64>> {
        match self {
            Self::DoubleArray(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to ObjectArray.
    pub fn into_object_array(self) -> Option<Vec<Self>> {
        match self {
            Self::ObjectArray(val) => Some(val),
            _ => None,
        }
    }

    /// Try to unwrap to Map.
    pub fn into_map(self) -> Option<HashMap<String, Self>> {
        match self {
            Self::Map(val) => Some(val),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Void => 0,
            Self::String(s) => s.len(),
            Self::StringArray(s) => s.len(),
            Self::Boolean(_) => 1,
            Self::Int(_) => 1,
            Self::IntArray(i) => i.len(),
            Self::ByteArray(b) => b.len(),
            Self::Float(_) => 1,
            Self::Double(_) => 1,
            Self::FloatArray(f) => f.len(),
            Self::DoubleArray(d) => d.len(),
            Self::ObjectArray(j) => j.len(),
            Self::Map(m) => m.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Void => true,
            _ => self.len() > 0,
        }
    }

    // TODO: Test this function. It's not tested yet and possibly can fall with errors.
    pub fn from_jobject<'local>(env: &mut Env<'local>, obj: JObject<'local>) -> Result<Self> {
        if obj.is_null() {
            return Ok(Self::Void);
        }
        let class = env.get_object_class(&obj)?;
        let name = get_class_name(env, &class)?;

        let result = match name.as_str() {
            "V" => Self::Void,
            "java.lang.String" => {
                let string = env.cast_local::<JString>(obj)?;
                let val = jstring_to_string(env, &string)?;
                Self::String(val)
            }
            "[Ljava.lang.String;" => {
                let array = env.cast_local::<JObjectArray<JString>>(obj)?;
                let count = array.len(env)?;
                let mut arr = Vec::with_capacity(count);
                for i in 0..count {
                    let val = array.get_element(env, i)?;
                    arr.push(jstring_to_string(env, &val)?);
                }
                Self::StringArray(arr)
            }
            "java.lang.Boolean" => {
                let val = env.call_method(&obj, jni_str!("booleanValue"), jni_sig!("()Z"), &[])?;
                Self::Boolean(val.z()?)
            }
            "java.lang.Integer" | "java.lang.Long" => {
                let val = env.call_method(&obj, jni_str!("longValue"), jni_sig!("()J"), &[])?;
                Self::Int(val.j()?)
            }
            "[I" => {
                let array = env.cast_local::<JIntArray>(obj)?;
                let mut values = vec![0; array.len(env)?];
                array.get_region(env, 0, &mut values)?;
                Self::IntArray(values.into_iter().map(i64::from).collect())
            }
            "[B" => {
                let array = env.cast_local::<JByteArray>(obj)?;
                let arr = env.convert_byte_array(&array)?;
                Self::ByteArray(arr)
            }
            "java.lang.Float" => {
                let res = env.call_method(&obj, jni_str!("floatValue"), jni_sig!("()F"), &[])?;
                Self::Float(res.f()?)
            }
            "java.lang.Double" => {
                let res = env.call_method(&obj, jni_str!("doubleValue"), jni_sig!("()D"), &[])?;
                Self::Double(res.d()?)
            }
            "[D" => {
                let array = env.cast_local::<JDoubleArray>(obj)?;
                let mut values = vec![0.0; array.len(env)?];
                array.get_region(env, 0, &mut values)?;
                Self::DoubleArray(values)
            }
            "[F" => {
                let array = env.cast_local::<JFloatArray>(obj)?;
                let mut values = vec![0.0; array.len(env)?];
                array.get_region(env, 0, &mut values)?;
                Self::FloatArray(values)
            }
            "[Ljava.lang.Object;" => {
                let array = env.cast_local::<JObjectArray>(obj)?;
                let count = array.len(env)?;
                let mut arr = Vec::with_capacity(count);
                for i in 0..count {
                    let val = array.get_element(env, i)?;
                    let inner = Self::from_jobject(env, val)?;
                    arr.push(inner);
                }
                Self::ObjectArray(arr)
            }
            "java.util.HashMap" | "com.crossbow.library.Dictionary" => {
                let arr = env.call_method(
                    &obj,
                    jni_str!("get_keys"),
                    jni_sig!("()[Ljava/lang/String;"),
                    &[],
                )?;
                let keys = Self::from_jobject(env, arr.l()?)?
                    .into_string_array()
                    .ok_or(AndroidError::WrongJniRustType)?;

                let arr = env.call_method(
                    &obj,
                    jni_str!("get_values"),
                    jni_sig!("()[Ljava/lang/Object;"),
                    &[],
                )?;
                let vals = Self::from_jobject(env, arr.l()?)?;

                let mut map = HashMap::new();
                let values = vals
                    .into_object_array()
                    .ok_or(AndroidError::WrongJniRustType)?;
                map.extend(keys.into_iter().zip(values));
                Self::Map(map)
            }
            _ => {
                return Err(AndroidError::UnsupportedJniRustType(name.to_owned()));
            }
        };
        Ok(result)
    }
}

impl Display for JniRustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Void => "".to_owned(),
            Self::String(s) => s.to_owned(),
            Self::StringArray(arr) => {
                let mut result = "".to_owned();
                for s in arr {
                    result = format!("{}{},", result, s);
                }
                result
            }
            Self::Boolean(b) => b.to_string(),
            Self::Int(i) => i.to_string(),
            Self::IntArray(arr) => {
                let mut result = "".to_owned();
                for i in arr {
                    result = format!("{}{},", result, i);
                }
                result
            }
            Self::ByteArray(arr) => std::str::from_utf8(arr).unwrap().to_owned(),
            Self::Float(f) => f.to_string(),
            Self::Double(d) => d.to_string(),
            Self::DoubleArray(arr) => {
                let mut result = "".to_owned();
                for i in arr {
                    result = format!("{}{},", result, i);
                }
                result
            }
            Self::FloatArray(arr) => {
                let mut result = "".to_owned();
                for i in arr {
                    result = format!("{}{},", result, i);
                }
                result
            }
            Self::ObjectArray(arr) => {
                let mut result = "".to_owned();
                for i in arr {
                    result = format!("{}{},", result, i);
                }
                result
            }
            Self::Map(map) => {
                let mut result = "".to_owned();
                for (k, v) in map {
                    result = format!("{}{}:{},", result, k, v);
                }
                result
            }
        };
        write!(f, "{}", val)
    }
}
