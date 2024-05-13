// Use `README.md` as documentation home page, to reduce duplication
#![doc = include_str!("../README.md")]

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Implement `Unscrupulous`, and ensure all fields implement the trait.
#[proc_macro_derive(Unscrupulous)]
pub fn derive_unscrupulous(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Quasi-quotation macro expects plain identifiers
    let name = input.ident;

    // Split generics based on where they are declared
    let (generics, generics_of_ty, where_clause) = input.generics.split_for_impl();

    // Build the output, then hand the resulting tokens back to the compiler
    proc_macro::TokenStream::from(quote! {
        unsafe impl #generics unscrupulous::Unscrupulous for #name #generics_of_ty #where_clause {}
    })
}
