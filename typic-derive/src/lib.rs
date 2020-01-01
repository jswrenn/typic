extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse_macro_input;

fn candidate(definition: &syn::ItemStruct) -> Option<syn::ItemStruct> {
    use syn::{Ident, Visibility};

    let all_public = definition.fields.iter().all(|field| {
        if let Visibility::Public(_) = field.vis {
            true
        } else {
            false
        }
    });

    if all_public {
        return None;
    }

    let mut skeleton = definition.clone();

    skeleton.ident = Ident::new(
        &format!("{}Candidate", definition.ident)[..],
        definition.ident.span(),
    );

    skeleton.vis = Visibility::Inherited;

    Some(skeleton)
}

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
            quote! {typic::hir::product::Nil},
            |rest, field| quote! {typic::hir::product::Cons<#field, #rest>},
        );

    let candidate = candidate(&definition);

    let candidate_name = if let Some(ref candidate) = candidate {
        candidate.ident.clone()
    } else {
        name.clone()
    };

    (quote! {
      #[repr(C)]
      #definition

      #[doc(hidden)]
      #candidate

      impl #generics typic::hir::Candidate for #name #generics {
        type Candidate = #candidate_name #generics;
      }

      impl #generics typic::hir::Type for #name #generics {
        type Padding = typic::hir::padding::Padded;
        type Representation = #fields;
        //type Representation = typic::hir::product::Cons<#fields, typic::hir::product>;
      }

    })
    .into()
}

fn impl_union(definition: syn::ItemUnion) -> TokenStream {
    let name = &definition.ident;
    let generics = definition.generics.clone();

    let mut iter = definition.fields.named.iter().map(|field| field.ty.clone());

    let fields = if let Some(first) = iter.next() {
        iter.rfold(
            quote! {typic::hir::coproduct::Nil<#first>},
            |rest, field| quote! {typic::hir::coproduct::Cons<#field, #rest>},
        )
    } else {
        unimplemented!()
    };

    (quote! {
      #[repr(C)]
      #definition

      impl #generics typic::hir::Candidate for #name #generics {
        type Candidate = Self;
      }

      impl #generics typic::hir::Type for #name #generics {
        type Padding = typic::hir::padding::Padded;
        type Representation = #fields;
      }
    })
    .into()
}

#[proc_macro_attribute]
pub fn repr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let definition: syn::Item = parse_macro_input!(input);

    match definition {
        syn::Item::Struct(definition) => impl_struct(definition),
        syn::Item::Union(definition) => impl_union(definition),
        _ => unimplemented!(),
    }
}
