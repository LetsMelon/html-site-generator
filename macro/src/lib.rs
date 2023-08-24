use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, DeriveInput, Ident, ItemStruct};

#[proc_macro_attribute]
pub fn add_attributes_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    _attributes: crate::attributes::HtmlAttributes
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[proc_macro_derive(DeriveSetHtmlAttributes)]
pub fn derive_set_html_attributes(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // TODO make it possible to get the name of the field in the derive macro call
    let default_attributes_field = Ident::new("_attributes", Span::call_site().into());

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl crate::attributes::SetHtmlAttributes for #name {
            fn get_attributes(&self) -> &crate::attributes::HtmlAttributes { &self. #default_attributes_field }
            fn get_attributes_mut(&mut self) -> &mut crate::attributes::HtmlAttributes { &mut self. #default_attributes_field }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
