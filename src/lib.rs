#![allow(unreachable_patterns)]

#![feature(custom_attribute)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod tok;
