//! Example:
//!
//! ```
//! #[macro_use]
//! extern crate option_constructor_derive;
//!
//! #[derive(OptionConstructor, Debug, PartialEq)]
//! struct Example {
//!     field1: bool,
//!     field2: Option<bool>,
//!     field3: Option<bool>,
//! }
//!
//! fn main() {
//!     let x = Example::new(true).field2(false);
//!     assert_eq!(x, Example {
//!         field1: true,
//!         field2: Some(false),
//!         field3: None,
//!     });
//! }
//! ```

#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[doc(hidden)]
#[proc_macro_derive(OptionConstructor)]
pub fn derive_option_constructor(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = expand_function(ast);
    expanded.to_string().parse().unwrap()
}

fn is_option_ident(f: &(&syn::Ident, &syn::Ty)) -> bool {
    match *f.1 {
        syn::Ty::Path(_, ref path) => {
            match path.segments.first().unwrap().ident.as_ref() {
                "Option" => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn expand_function(ast: syn::MacroInput) -> quote::Tokens {
    let fields: Vec<_> = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).collect()
        }
        syn::Body::Struct(syn::VariantData::Unit) => vec![],
        _ => panic!("#[derive(OptionConstructor)] can only be used with braced structs"),
    };

    let field_compulsory: Vec<_> = fields.iter()
        .filter(|f| !is_option_ident(&f))
        .map(|f| f.0)
        .collect();

    let field_optional: Vec<_> =
        fields.iter().filter(|f| is_option_ident(&f)).map(|f| f.0).collect();
    let field_optional2 = field_optional.clone();

    let none = syn::Ident::from("None");
    let field_all: Vec<_> = fields.iter().map(|f| f.0).collect();
    let values: Vec<_> = fields.iter()
        .map(|f| if is_option_ident(f) { &none } else { f.0 })
        .collect();

    let ty_compulsory: Vec<_> = fields.iter().map(|f| f.1).collect();
    let ty_optional: Vec<_> = fields.iter()
        .filter(|f| is_option_ident(&f))
        .map(|f| {
            if let syn::Ty::Path(_, ref path) = *f.1 {
                if let syn::PathParameters::AngleBracketed(ref param) =
                    path.segments
                        .first()
                        .unwrap()
                        .parameters {
                    return param.types.first().unwrap();
                }
            }

            panic!("no sane type!");
        })
        .collect();

    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn new(#( #field_compulsory: #ty_compulsory, )*) -> #struct_name #ty_generics {
                #struct_name {
                    #( #field_all: #values, )*
                }
            }
            #(
                pub fn #field_optional(mut self, val: #ty_optional) -> Self {
                    self.#field_optional2 = Some(val);
                    self
                }
            )*
        }
    }
}
