#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate num_rational;
#[macro_use] extern crate rocket;

pub mod food;
fn main() {
    println!("Hello, world!");
}
