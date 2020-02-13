use crate::data::{Argument, Value, List, ValueType, Dict};
use crate::errors::{CrushResult, argument_error};

pub fn single_argument_type(mut arg: Vec<Argument>) -> CrushResult<ValueType> {
    match arg.len() {
        1 => {
            let a = arg.remove(0);
            match (a.name, a.value) {
                (None, Value::Type(t)) => Ok(t),
                _ => Err(argument_error("Expected a list value")),
            }
        }
        _ => Err(argument_error("Expected a single value")),
    }
}

pub fn single_argument_list(mut arg: Vec<Argument>) -> CrushResult<List> {
    match arg.len() {
        1 => {
            let a = arg.remove(0);
            match (a.name, a.value) {
                (None, Value::List(t)) => Ok(t),
                _ => Err(argument_error("Expected a list value")),
            }
        }
        _ => Err(argument_error("Expected a single value")),
    }
}

pub fn single_argument_dict(mut arg: Vec<Argument>) -> CrushResult<Dict> {
    match arg.len() {
        1 => {
            let a = arg.remove(0);
            match (a.name, a.value) {
                (None, Value::Dict(t)) => Ok(t),
                _ => Err(argument_error("Expected a list value")),
            }
        }
        _ => Err(argument_error("Expected a single value")),
    }
}

pub fn single_argument_field(mut arg: Vec<Argument>) -> CrushResult<Vec<Box<str>>> {
    match arg.len() {
        1 => {
            let a = arg.remove(0);
            match (a.name, a.value) {
                (None, Value::Field(t)) => Ok(t),
                _ => Err(argument_error("Expected a field value")),
            }
        }
        _ => Err(argument_error("Expected a single value")),
    }
}

pub fn single_argument_text(mut arg: Vec<Argument>) -> CrushResult<Box<str>> {
    match arg.len() {
        1 => {
            let a = arg.remove(0);
            match (a.name, a.value) {
                (None, Value::Text(t)) => Ok(t),
                _ => Err(argument_error("Expected a text value")),
            }
        }
        _ => Err(argument_error("Expected a single value")),
    }
}

pub fn single_argument_integer(mut arg: Vec<Argument>) -> CrushResult<i128> {
    match arg.len() {
        1 => {
            let a = arg.remove(0);
            match (a.name, a.value) {
                (None, Value::Integer(i)) => Ok(i),
                _ => Err(argument_error("Expected a text value")),
            }
        }
        _ => Err(argument_error("Expected a single value")),
    }
}