extern crate cfg_if;
extern crate wasm_bindgen;
extern crate liquers_core;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use]
extern crate lazy_static;

use liquers_core::value::*;
use liquers_core::error::*;
use liquers_core::action_registry::*;
use liquers_core::query::Environment;
use std::sync::{Mutex};
use std::convert::{TryFrom, TryInto};

//use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref REGISTRY:Mutex<HashMapActionRegistry<Value>> = Mutex::new(HashMapActionRegistry::<Value>::new());
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn register_hello() {
    use liquers_core::action_registry::*;
    let registry = &mut *REGISTRY.lock().expect("Registry lock failed");    
    let hello = |x:String| format!("Hello, {}!",x);
    registry.register_callable_action("root", "hello", Box::new(Function1(Box::new(hello))));
}

pub fn jsvalue_to_value(jsvalue: JsValue) -> Result<Value, Error>{
    if let Some(x) = jsvalue.as_f64(){
        Ok(Value::from(x))
    }
    else if let Some(x) = jsvalue.as_bool(){
        Ok(Value::from(x))
    }
    else if let Some(x) = jsvalue.as_string(){
        Ok(Value::from(x))
    }
    else if jsvalue.is_undefined(){
        Ok(Value::None)
    }
    else if jsvalue.is_null(){
        Ok(Value::None)
    }
    else{
        Err(Error::ConversionError{message:format!("Can't convert js value liquer value {:?}",jsvalue)})
    }
}

pub fn value_to_jsvalue(value:Value)->JsValue{
    match value{
        Value::None => JsValue::null(),
        Value::Text(x) => JsValue::from_str(&x),
        Value::Integer(x) => JsValue::from_f64(x as f64),
        Value::Real(x) => JsValue::from_f64(x),           
        Value::Bool(x) => JsValue::from_bool(x)
    }
}

#[wasm_bindgen]
pub fn eval(input:JsValue, query:&str) -> Result<JsValue,JsValue>{
    let mut registry = REGISTRY.lock().map_err(|e| JsValue::from(format!("Registry lock failed: {}",e)))?;
    
    let input = jsvalue_to_value(input).map_err(|e| format!("{}",e))?;
    let result = (*registry).eval(input,query).map_err(|e| format!("{}",e))?;
    Ok(value_to_jsvalue(result))
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, LiQuer!");
}

