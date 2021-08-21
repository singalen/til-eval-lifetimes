use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{DerefMut};

use serde::{Serialize, Deserialize};

/// Here 'world is the root object for all the in-game world.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TalkValue<'world> {
    Int(i64),
    String(String),
    Bool(bool),
    Object(Box<TalkObject<'world>>)
}

impl<'world> Debug for TalkValue<'world> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Object(o) => write!(f, "{:?}", o),
        }
    }
}

impl<'world> TalkValue<'world> {
    #[allow(dead_code)] pub fn new_obj() -> Self { TalkValue::Object(Box::new(TalkObject::new())) }
    #[allow(dead_code)] pub fn new_str(s: &str) -> Self { TalkValue::String(s.to_string()) }
    #[allow(dead_code)] pub fn new_int(i: i64) -> Self { TalkValue::Int(i) }

    #[allow(dead_code)]
    pub fn as_bool(&self) -> bool {
        match self {
            TalkValue::Int(i) => *i != 0,
            TalkValue::String(s) => !s.is_empty(),
            TalkValue::Bool(b) => *b,
            TalkValue::Object(o) => !o.is_empty()
        }
    }

    #[allow(dead_code)]
    pub fn as_object(&mut self) -> Result<&mut TalkObject<'world>, TalkEvalError> {
        match self {
            TalkValue::Int(_) => Err(TalkEvalError::new("Object expected, got Int")),
            TalkValue::String(_) => Err(TalkEvalError::new("Object expected, got String")),
            TalkValue::Bool(_) => Err(TalkEvalError::new("Object expected, got Bool")),
            TalkValue::Object(o) => Ok(o.deref_mut()),
        }
    }

    #[allow(dead_code)]
    pub fn into_int(self) -> Result<i64, TalkEvalError> {
        match self {
            TalkValue::Int(i) => Ok(i),
            TalkValue::String(s) => Err(TalkEvalError::new(&format!("Integer value expected, got String {}", s))),
            TalkValue::Bool(b) => Err(TalkEvalError::new(&format!("Integer value expected, got Bool {}", b))),
            TalkValue::Object(_) => Err(TalkEvalError::new(&"Integer value expected, got Object".to_string())),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TalkObject<'world> {
    // fn get(&self, name: &str) -> Option<&TalkValue>;
    // fn set(&mut self, name: &str, val: TalkValue);
    #[serde(flatten)]
    map: HashMap<String, TalkValue<'world>>,

    // Now try to add a reference to external object...
    #[serde(skip)]
    proxy: Option<&'world mut dyn TalkObjectProxy<'world>>,
}

impl<'world> Debug for TalkObject<'world> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, proxy: {}", self.map, self.proxy.is_some())
    }
}


/// Implement this to use other objects to evaluate in Talk.
pub trait TalkObjectProxy<'world> {
    fn get(&mut self, name: &str) -> Option<&mut TalkValue<'world>>;
    fn set(&mut self, name: &str, val: TalkValue<'world>);
    fn is_empty(&self) -> bool { false }
}

impl<'world> TalkObjectProxy<'world> for TalkObject<'world> {
    fn get(&mut self, name: &str) -> Option<&mut TalkValue<'world>> {
        // Hack: allow undefined fields, in order to load scripts.
        if !self.map.contains_key(name) {
            self.map.insert(name.to_string(), TalkValue::new_obj());
        }

        self.map.get_mut(name)
    }

    fn set(&mut self, name: &str, val: TalkValue<'world>) {
        self.map.insert(name.to_string(), val);
    }

    fn is_empty(&self) -> bool { self.map.is_empty() }
}

impl<'world> TalkObject<'world> {
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

type EvalResult<'world> = Result<&'world mut TalkValue<'world>, TalkEvalError>;

pub trait Eval<'world, 'ast: 'world> {
    fn eval(&'ast self, context: &'world mut TalkObject<'world>) -> EvalResult<'world>;
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

impl<'world, 'ast: 'world> Eval<'world, 'ast> for Expression {
    fn eval(&'ast self, context: &'world mut TalkObject<'world>) -> EvalResult<'world> {
        // match self {
        //     Expression::Assignment { azz } => azz.eval(context),
        //     Expression::OrTest { or_test } => or_test.eval(context),
        // }
        context
            .get("42")
            .ok_or(TalkEvalError::new("oops"))
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::{Expression, TalkObject, Eval, TalkObjectProxy, TalkValue};

    struct DummyProxy<'world> {
        value: TalkValue<'world>,
    }

    impl<'world> DummyProxy<'world> {
        pub fn new() -> Self {
            Self { value: TalkValue::Bool(true) }
        }
    }

    impl<'world> TalkObjectProxy<'world> for DummyProxy<'world> {
        fn get(&mut self, _name: &str) -> Option<&mut TalkValue<'world>> {
            println!("get()");
            Some(&mut self.value)
        }

        fn set(&mut self, _name: &str, val: TalkValue<'world>) {
            println!("set()");
            self.value = val;
        }
    }

    #[test]
    fn test_bool() {
        let mut context = TalkObject::new();
        let expr = Expression::Dummy;

        let mut dummy = DummyProxy::new();
        context.proxy = Some(&mut dummy);

        let a = expr.eval(&mut context);
        println!("{:?}", a);
    }
}
