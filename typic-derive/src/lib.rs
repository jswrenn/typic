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
pub fn repr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let definition: syn::ItemStruct = parse_macro_input!(input);

    let name = &definition.ident;
    let generics = definition.generics.clone();

    let fields = definition
        .fields
        .iter()
        .map(|field| field.ty.clone())
        .rfold(
            quote! {typic::structure::Empty},
            |rest, field| quote! {typic::structure::Fields<#field, #rest>},
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

      impl #generics typic::transmute::Candidate for #name #generics {
        type Candidate = #candidate_name #generics;
      }

      impl #generics typic::Type for #name #generics {
        type Padding = typic::padding::Padded;
        type Representation = #fields;
      }

    })
    .into()
}
