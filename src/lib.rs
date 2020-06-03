extern crate cfg_if;
extern crate js_sys;
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
use liquers_core::query::{Environment, ActionParameter};

use std::sync::{Arc, Mutex};
use std::convert::{TryFrom, TryInto};
use std::cell::RefCell;

//use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

/*
lazy_static! {
    static ref REGISTRY:Mutex<HashMapActionRegistry<Value>> = Mutex::new(HashMapActionRegistry::<Value>::new());
}
*/
thread_local!{
    static REGISTRY : RefCell<HashMapActionRegistry<Value>> = RefCell::new(HashMapActionRegistry::<Value>::new());
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn register_hello() {
    use liquers_core::action_registry::*;
    //let registry = &mut *REGISTRY.lock().expect("Registry lock failed");
    REGISTRY.with(
        |registry|{
            let hello = |x:String| format!("Hello, {}!",x);
            (*registry).borrow_mut().register_callable_action("root", "hello", Box::new(Function1(Box::new(hello))));
        }
    );  
}

struct JsFunction(js_sys::Function);

impl CallableAction<Value> for JsFunction{
    fn call_action(&self, input:Value, arguments:&Vec<ActionParameter>) -> Result<Value, Error>{
        let f_input:JsValue = value_to_jsvalue(input);
        let mut js_arguments = js_sys::Array::of1(&f_input);
        for (i,arg) in arguments.iter().enumerate(){
            match arg{
                ActionParameter::String(arg,_) => js_arguments.push(&JsValue::from_str(&arg)),
                ActionParameter::Link(arg,pos) => Err(Error::General{message:format!("Link argument not supported (arg. {} at {})",i+1,pos)})?,
            };
        }
        let this = JsValue::NULL;

        let out = self.0.apply(&this, &js_arguments).map_err(|e| Error::General{message:format!("Execution error: {:?}",e)})?;
        let result = jsvalue_to_value(out)?;
        Ok(result)
    }
}

#[wasm_bindgen]
pub fn register(ns:&str, name:&str, f: js_sys::Function) -> Result<JsValue,JsValue> {
    use liquers_core::action_registry::*;
    //let mut registry = REGISTRY.lock().map_err(|e| JsValue::from(format!("Registry lock failed: {}",e)))?;
    //let name:&str = f.name().into();
    REGISTRY.with(|registry|{
        (*registry).borrow_mut().register_callable_action(ns, name, Box::new(JsFunction(f)));
    });
    Ok(JsValue::NULL)
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
pub fn eval_query(input:JsValue, query:&str) -> Result<JsValue,JsValue>{
    //let mut registry = REGISTRY.lock().map_err(|e| JsValue::from(format!("Registry lock failed: {}",e)))?;
    let input = jsvalue_to_value(input).map_err(|e| format!("{}",e))?;
    let result=REGISTRY.with(|registry|{
        (*registry).borrow_mut().eval(input,query).map_err(|e| format!("{}",e))
    })?;
    //let result = (*registry).eval(input,query).map_err(|e| format!("{}",e))?;
    Ok(value_to_jsvalue(result))
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, LiQuer!");
}

