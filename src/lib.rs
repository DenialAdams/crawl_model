#![recursion_limit = "256"]
#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod data;
pub mod db_model;
pub mod db_schema;
