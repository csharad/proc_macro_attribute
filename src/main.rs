#![feature(proc_macro)]
// #![feature(custom_attribute)] cannot use because it conflicts with proc_macro

extern crate proc_macro;

use proc_macro::{component, events, event};

#[component]
struct Div {
    // Tried to use a undeclared/undefined #[prop] attribute that #[component] could use but
    // did not work as compiler does not recognize it.
    // To circumvent that added a placeholder definition of the #[prop] attribute in the `proc_macro`
    // module but still does not recognize. Weird because this kind of logic works on impl bodies.

    // Solution: Read it on proc_macro/lib.rs
    #[prop]
    color: String,
    clicked: bool
}

// Works here painlessly
#[events]
impl Div {
    #[event]
    fn ev() {

    }
}

// This example won't actually work. Because I have not removed the `#[prop]` attribute from the struct
fn main() {
    println!("Hello, world!");
}
