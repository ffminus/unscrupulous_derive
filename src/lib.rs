// Use `README.md` as documentation home page, to reduce duplication
#![doc = include_str!("../README.md")]

use quote::__private::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Type};

/// Implement `Unscrupulous`, and ensure all fields implement the trait.
#[proc_macro_derive(Unscrupulous)]
pub fn derive_unscrupulous(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Quasi-quotation macro expects plain identifiers
    let name = input.ident;

    // Ensure types of nested fields implement `Unscrupulous`
    let assertions = match input.data {
        Data::Struct(data) => assert_fields(data.fields.iter()),
        Data::Enum(data) => assert_fields(data.variants.iter().flat_map(|var| var.fields.iter())),
        Data::Union(data) => assert_fields(data.fields.named.iter()),
    };

    // Split generics based on where they are declared
    let (generics, generics_of_ty, where_clause) = input.generics.split_for_impl();

    // Build the output, then hand the resulting tokens back to the compiler
    proc_macro::TokenStream::from(quote! {
        unsafe impl #generics unscrupulous::Unscrupulous for #name #generics_of_ty #where_clause {}

        #(#assertions)*
    })
}

/// Ensure types of nested fields implement `Unscrupulous`
#[allow(single_use_lifetimes)]
fn assert_fields<'a>(fields: impl Iterator<Item = &'a Field>) -> Vec<TokenStream> {
    fields.map(|f| ensure_type_is_unscrupulous(&f.ty)).collect()
}

/// Assert at compile time that the provided type implements `Unscrupulous`
fn ensure_type_is_unscrupulous(ty: &Type) -> TokenStream {
    quote! {
        const _: fn() = || {
            fn ensure_type_is_unscrupulous<T: unscrupulous::Unscrupulous>() {}
            ensure_type_is_unscrupulous::<#ty>();
        };
    }
}
