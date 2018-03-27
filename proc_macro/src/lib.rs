#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro2;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn component(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    // actually do some modification to the struct and also read field annotated with `#[prop]`
    // to create prop struct
    input
}

// Just as a placeholder attribute for the component attribute to work on
#[proc_macro_attribute]
pub fn prop(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}