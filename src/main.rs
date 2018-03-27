#![feature(proc_macro)]
// #![feature(custom_attribute)] cannot use because it conflicts with proc_macro

extern crate proc_macro;

use proc_macro::{prop, component};

//#[component]
struct Div {
    // Tried to use a undeclared/undefined #[prop] attribute that #[component] could use but
    // did not work as compiler does not recognize it.
    // To circumvent that added a placeholder definition of the #[prop] attribute in the `proc_macro`
    // module but still does not recognize. Weird because this kind of logic works on:
    //
    // #[events]
    // impl Events {
    //     #[event]         This placeholder #[event] attribute works and does not error out.
    //     fn ev() { ... }
    // }
    //
    #[prop]
    color: String,
    clicked: bool
}

fn main() {
    println!("Hello, world!");
}
