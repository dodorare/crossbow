use crate::{error::*, utils::*};
use jni::{
    objects::{JObject, JValue},
    signature::{JavaType, Primitive},
    JNIEnv,
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
    pub fn void(self) -> Result<()> {
        match self {
            Self::Void => Ok(()),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to Boolean.
    pub fn bool(self) -> Result<bool> {
        match self {
            Self::Boolean(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to String.
    pub fn string(self) -> Result<String> {
        match self {
            Self::String(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to StringArray.
    pub fn string_array(self) -> Result<Vec<String>> {
        match self {
            Self::StringArray(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to Int.
    pub fn int(self) -> Result<i64> {
        match self {
            Self::Int(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to IntArray.
    pub fn int_array(self) -> Result<Vec<i64>> {
        match self {
            Self::IntArray(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to ByteArray.
    pub fn byte_array(self) -> Result<Vec<u8>> {
        match self {
            Self::ByteArray(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to Float.
    pub fn float(self) -> Result<f32> {
        match self {
            Self::Float(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to Double.
    pub fn double(self) -> Result<f64> {
        match self {
            Self::Double(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to FloatArray.
    pub fn float_array(self) -> Result<Vec<f32>> {
        match self {
            Self::FloatArray(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to DoubleArray.
    pub fn double_array(self) -> Result<Vec<f64>> {
        match self {
            Self::DoubleArray(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to ObjectArray.
    pub fn object_array(self) -> Result<Vec<Self>> {
        match self {
            Self::ObjectArray(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
        }
    }

    /// Try to unwrap to Map.
    pub fn map(self) -> Result<HashMap<String, Self>> {
        match self {
            Self::Map(val) => Ok(val),
            _ => Err(AndroidError::WrongJniRustType),
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
    pub fn from_jobject(env: &JNIEnv, obj: JObject) -> Result<Self> {
        let class = env.get_object_class(obj)?;
        let name = get_class_name(env, class)?;

        let result = match name.as_str() {
            "V" => Self::Void,
            "java.lang.String" => {
                let val = jstring_to_string(env, obj.into())?;
                Self::String(val)
            }
            "[Ljava.lang.String;" => {
                let count = env.get_array_length(obj.into_inner())?;
                let mut arr = Vec::new();
                for i in 0..count {
                    let val = env.get_object_array_element(obj.into_inner(), i)?;
                    arr.push(jstring_to_string(env, val.into())?);
                }
                Self::StringArray(arr)
            }
            "java.lang.Boolean" => {
                let bool_value = env.get_method_id(class, "booleanValue", "()Z")?;
                let val = env.call_method_unchecked(
                    obj,
                    bool_value,
                    JavaType::Primitive(Primitive::Boolean),
                    &[],
                )?;
                Self::Boolean(val.z()?)
            }
            "java.lang.Integer" | "java.lang.Long" => {
                let nclass = env.find_class("java/lang/Number")?;
                let long_value = env.get_method_id(nclass, "longValue", "()J")?;
                let val = env.call_method_unchecked(
                    obj,
                    long_value,
                    JavaType::Primitive(Primitive::Long),
                    &[],
                )?;
                Self::Int(val.j()?)
            }
            "[I" => {
                let count = env.get_array_length(obj.into_inner())?;
                let mut integers = Vec::new();
                for i in 0..count {
                    let val = env.get_object_array_element(obj.into_inner(), i)?;
                    integers.push(JValue::from(val).j()?);
                }
                Self::IntArray(integers)
            }
            "[B" => {
                let arr = env.convert_byte_array(obj.into_inner())?;
                Self::ByteArray(arr)
            }
            "java.lang.Float" => {
                let nclass = env.find_class("java/lang/Number")?;
                let long_value = env.get_method_id(nclass, "floatValue", "()F")?;
                let res = env.call_method_unchecked(
                    obj,
                    long_value,
                    JavaType::Primitive(Primitive::Float),
                    &[],
                )?;
                Self::Float(res.f()?)
            }
            "java.lang.Double" => {
                let nclass = env.find_class("java/lang/Number")?;
                let long_value = env.get_method_id(nclass, "doubleValue", "()D")?;
                let res = env.call_method_unchecked(
                    obj,
                    long_value,
                    JavaType::Primitive(Primitive::Double),
                    &[],
                )?;
                Self::Double(res.d()?)
            }
            "[D" => {
                let count = env.get_array_length(obj.into_inner())?;
                let mut arr = Vec::new();
                for i in 0..count {
                    let val = env.get_object_array_element(obj.into_inner(), i)?;
                    arr.push(JValue::from(val).d()?);
                }
                Self::DoubleArray(arr)
            }
            "[F" => {
                let count = env.get_array_length(obj.into_inner())?;
                let mut arr = Vec::new();
                for i in 0..count {
                    let val = env.get_object_array_element(obj.into_inner(), i)?;
                    arr.push(JValue::from(val).f()?);
                }
                Self::FloatArray(arr)
            }
            "[Ljava.lang.Object;" => {
                let count = env.get_array_length(obj.into_inner())?;
                let mut arr = Vec::new();
                for i in 0..count {
                    let val = env.get_object_array_element(obj.into_inner(), i)?;
                    let inner = Self::from_jobject(env, val)?;
                    arr.push(inner);
                }
                Self::ObjectArray(arr)
            }
            "java.util.HashMap" | "com.crossbow.library.Dictionary" => {
                let get_keys = env.get_method_id(class, "get_keys", "()[Ljava/lang/String;")?;
                let arr =
                    env.call_method_unchecked(obj, get_keys, JavaType::Object("".to_owned()), &[])?;
                let keys = Self::from_jobject(env, arr.l()?)?.string_array()?;

                let get_values = env.get_method_id(class, "get_values", "()[Ljava/lang/Object;")?;
                let arr = env.call_method_unchecked(
                    obj,
                    get_values,
                    JavaType::Object("".to_owned()),
                    &[],
                )?;
                let vals = Self::from_jobject(env, arr.l()?)?;

                let mut map = HashMap::new();
                let values = vals.object_array()?;
                map.extend(keys.into_iter().zip(values.into_iter()));
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
