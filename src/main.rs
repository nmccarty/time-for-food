#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate num_rational;
#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;


pub mod food;
fn main() {
    println!("Hello, world!");
}
