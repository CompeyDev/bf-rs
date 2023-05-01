use std::{fmt::Write, process::exit};
use colored::*;
use std::any::{Any, TypeId};
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
 
type HashKey<K> = (K, TypeId);
type Anything = Box<dyn Any>;
 
pub struct AnyMap<K: Eq + Hash>(HashMap<HashKey<K>, Anything>);
 
impl<K: Eq + Hash> AnyMap<K> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
 
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn insert<V: Any>(&mut self, key: K, val: V) -> Option<V> {
        let boxed = self
            .0
            .insert((key, val.type_id()), Box::new(val))?
            .downcast::<V>()
            .ok()?;
 
        Some(Box::into_inner(boxed))
    }
 
    pub fn get<V: Any>(&self, key: K) -> Option<&V> {
        self.0.get(&(key, TypeId::of::<V>()))?.downcast_ref::<V>()
    }
 
    pub fn get_mut<V: Any>(&mut self, key: K) -> Option<&mut V> {
        self.0
            .get_mut(&(key, TypeId::of::<V>()))?
            .downcast_mut::<V>()
    }
 
    pub fn remove<V: Any>(&mut self, key: K) -> Option<V> {
        let boxed = self
            .0
            .remove(&(key, TypeId::of::<V>()))?
            .downcast::<V>()
            .ok()?;
 
        Some(Box::into_inner(boxed))
    }
}


pub fn strip_code(code: &str) -> String {
    const INSTR_SET: [&str; 8] = [">", "<", "+", "-", ".", ",", "[", "]"];

    let mut stripped_code = String::new();

    for (_, instr) in code.chars().enumerate() {
        let instr_stringified = instr.to_string();

        if !INSTR_SET.contains(&instr_stringified.as_str()) {
            continue;
        }

        match write!(stripped_code, "{}", instr_stringified) {
            Ok(_) => (),
            Err(_) => {
                throw_err("STRIP_ERROR", "failed to write to stream");
            }
        };
    }

    return stripped_code;
}

pub fn throw_err(err_type: &str, msg: &str) -> ! {
    println!("[{}] {}: {}", "error".red().bold(), err_type, msg);
    exit(1);
}
