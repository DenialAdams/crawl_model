#![recursion_limit = "256"]
#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

mod data;
mod db_model;
mod db_schema;
