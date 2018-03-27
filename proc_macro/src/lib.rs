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

    // Solution: To make the attribute on fields work, read the attribute and remove it afterward
    // from the field, otherwise the compiler searches for its meaning again.

    input
}

// Just as a placeholder attribute for the component attribute to work on.
//
// Solution: The struct fields don't even require placeholder attributes, you could write the
// attribute definition in the struct attribute
#[proc_macro_attribute]
pub fn prop(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}


// Similar to component but for impls
#[proc_macro_attribute]
pub fn events(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

// Just a placeholder to identify event methods. (Required: cannot be derived from within the
// #[events] macro)
#[proc_macro_attribute]
pub fn event(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}