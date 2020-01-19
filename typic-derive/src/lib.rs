#![allow(warnings)]
extern crate proc_macro;

use if_chain::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn;
use syn::parse_macro_input;

mod synstructure;

#[proc_macro_attribute]
pub fn typicrepr(_args: TokenStream, input: TokenStream) -> TokenStream {
    repr(_args, input)
}

fn impl_struct(definition: syn::ItemStruct) -> TokenStream {
    let name = &definition.ident;
    let generics = definition.generics.clone();

    let fields = definition
        .fields
        .iter()
        .map(|field| field.ty.clone())
        .rfold(
            quote! {typic::highlevel::PNil},
            |rest, field| quote! {typic::highlevel::PCons<#field, #rest>},
        );

    (quote! {
      #[repr(C)]
      #definition

      impl #generics typic::hir::Type for #name #generics {
        type Padding = typic::hir::padding::Padded;
        type Representation = #fields;
      }

    })
    .into()
}

#[proc_macro_attribute]
pub fn repr(args: TokenStream, input: TokenStream) -> TokenStream {
    let definition: syn::Item = parse_macro_input!(input);

    match definition {
        syn::Item::Struct(definition) => impl_struct(definition),
        _ => unimplemented!(),
    }
}
