use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{DerefMut};

use serde::{Serialize, Deserialize};

// use crate::talk::ast::*;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TalkValue {
    Int(i64),
    String(String),
    Bool(bool),
    Object(Box<TalkObject>)
}

impl TalkValue {
    #[allow(dead_code)]
    pub fn new_obj() -> Self { TalkValue::Object(Box::new(TalkObject::new())) }
    pub fn new_str(s: &str) -> Self { TalkValue::String(s.to_string()) }
    pub fn new_int(i: i64) -> Self { TalkValue::Int(i) }

    pub fn as_bool(&self) -> bool {
        match self {
            TalkValue::Int(i) => *i != 0,
            TalkValue::String(s) => !s.is_empty(),
            TalkValue::Bool(b) => *b,
            TalkValue::Object(o) => !o.is_empty()
        }
    }

    pub fn as_object(&mut self) -> Result<&mut TalkObject, TalkEvalError> {
        match self {
            TalkValue::Int(_) => Err(TalkEvalError::new("Object expected, got Int")),
            TalkValue::String(_) => Err(TalkEvalError::new("Object expected, got String")),
            TalkValue::Bool(_) => Err(TalkEvalError::new("Object expected, got Bool")),
            TalkValue::Object(o) => Ok(o.deref_mut()),
        }
    }

    pub fn into_int(self) -> Result<i64, TalkEvalError> {
        match self {
            TalkValue::Int(i) => Ok(i),
            TalkValue::String(s) => Err(TalkEvalError::new(&format!("Integer value expected, got String {}", s))),
            TalkValue::Bool(b) => Err(TalkEvalError::new(&format!("Integer value expected, got Bool {}", b))),
            TalkValue::Object(_) => Err(TalkEvalError::new(&"Integer value expected, got Object".to_string())),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TalkObject {
    // fn get(&self, name: &str) -> Option<&TalkValue>;
    // fn set(&mut self, name: &str, val: TalkValue);
    #[serde(flatten)]
    map: HashMap<String, TalkValue>,

    // Now try to add a reference to external object...
    // #[serde(skip)]
    // proxy: Option<&mut dyn TalkObjectProxy>,
}


/// Implement this to use other objects to evaluate in Talk.
pub trait TalkObjectProxy {
    fn get(&mut self, name: &str) -> Option<&mut TalkValue>;
    fn set(&mut self, name: &str, val: TalkValue);
    fn is_empty(&self) -> bool { false }
}

impl TalkObjectProxy for TalkObject {
    fn get(&mut self, name: &str) -> Option<&mut TalkValue> {
        // Hack: allow undefined fields, in order to load scripts.
        if !self.map.contains_key(name) {
            self.map.insert(name.to_string(), TalkValue::new_obj());
        }

        self.map.get_mut(name)
    }

    fn set(&mut self, name: &str, val: TalkValue) {
        self.map.insert(name.to_string(), val);
    }

    fn is_empty(&self) -> bool { self.map.is_empty() }
}

impl TalkObject {
    pub fn new() -> Self { Default::default() }
}

#[derive(Debug)]
pub struct TalkEvalError {
    pub text: String
}

impl TalkEvalError {
    // pub fn new(text: String) -> Self { TalkEvalError{ text } }
    pub fn new(text: &str) -> Self { TalkEvalError{ text: text.to_string() } }
}

type EvalResult = Result<TalkValue, TalkEvalError>;

pub trait Eval {
    fn eval(&self, context: &mut TalkObject) -> EvalResult;
}

// ...

#[derive(Debug, Clone)] // FromPest,
// #[pest_ast(rule(Rule::expression))]
pub enum Expression {
    // Assignment {
    //     azz: Assignment,
    // },
    // OrTest {
    //     or_test: OrTest,
    // }
    Dummy
}

impl Eval for Expression {
    fn eval(&self, context: &mut TalkObject) -> EvalResult {
        // match self {
        //     Expression::Assignment { azz } => azz.eval(context),
        //     Expression::OrTest { or_test } => or_test.eval(context),
        // }
        Ok(TalkValue::new_int(42))
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::{Expression, TalkObject, Eval};

    #[test]
    fn test_bool() {
        let mut context = TalkObject::new();
        let expr = Expression::Dummy;
        let a = expr.eval(&mut context);
    }
}
