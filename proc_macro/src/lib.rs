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

// Similar to component but for impls
#[proc_macro_attribute]
pub fn events(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

// Just a placeholder to identify event methods
#[proc_macro_attribute]
pub fn event(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}