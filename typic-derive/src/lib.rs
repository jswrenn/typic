extern crate proc_macro;

use if_chain::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
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

fn enum_repr(args: syn::AttributeArgs) -> (TokenStream2, String) {
  let explicit =
    args.into_iter()
      .find_map(|arg|
          if_chain! {
            if let syn::NestedMeta::Meta(meta) = arg;
            if let syn::Meta::Path(path) = meta;
            if let Some(ident) = path.get_ident();
            then {
              let ident = ident.clone();
              let name = ident.to_string();
              match name.as_str() {
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                 => Some((ident, name)),
                _=> None
              }
            } else {
              None
            }
          }
        );
  if let Some((repr, string)) = explicit {
    (quote!{#repr}, string)
  } else {
    (quote!{isize}, "usize".to_string())
  }
}

fn impl_enum(args: TokenStream, definition: syn::ItemEnum) -> TokenStream {
    let args_clone : TokenStream2 = args.clone().into();
    let args: syn::AttributeArgs = parse_macro_input!(args);
    let (repr, repr_string) = enum_repr(args);

    let name = &definition.ident;
    let generics = definition.generics.clone();


    let mut variant_disrs: Vec<_> = definition.variants.iter()
      .map(|variant| variant.discriminant.clone().map(|d| d.1)).collect();

    {
      let mut disr_iter = variant_disrs.iter_mut();

      let mut last_disr = None;

      if let Some(first) = disr_iter.next() {
        if let None = first {
          let disr = quote!{0}.into();
          let disr : syn::Expr = parse_macro_input!(disr);
          *first = Some(disr);
        }
        last_disr = first.clone();
      }

      for disr in disr_iter {
        if let None = disr {
          let last_disr = last_disr.clone().unwrap();
          let new_disr = quote!{(#last_disr) + 1}.into();
          let new_disr : syn::Expr = parse_macro_input!(new_disr);
          *disr = Some(new_disr);
        }
      }
    }

    let variant_types: Vec<_> =
      definition.variants.iter().map(|variant| variant.clone()).zip(variant_disrs)
      .map(|(mut variant, disr)|
        {
          let disr = disr.unwrap();
          let name = name.clone();
          let repr = repr.clone();
          let size = format_ident!("u{}", repr_string[1..]);
          let variant_name = variant.ident.clone();

          variant.discriminant = None;

          quote! {
            pub struct #variant ;

            impl crate::typic::hir::Type for #variant_name {
              type Padding = crate::typic::hir::padding::Padded;
              type Representation =
                crate::typic::hir::product::Cons<
                  crate::typic::hir::Discriminant<#size, {&#repr::to_ne_bytes(#disr)}>,
                  crate::typic::hir::product::Nil>;
            }
          }
        }
      )
      .collect();

    
    let mut representation = definition.variants.iter().map(|variant| variant.ident.clone());

    let module_name = format_ident!("typic_{}_desugar", name);

    let representation =
      if let Some(name) = representation.next() {
        representation.rfold(
            quote! {typic::hir::coproduct::Nil<#module_name::#name>},
            |rest, name|
              quote! {typic::hir::coproduct::Cons<#module_name::#name, #rest>}
        )
      } else {
        quote!{ typic::hir::Uninhabited; }
      };

    let f : TokenStream = (quote! {
      #[repr( #args_clone )]
      #definition

      impl #generics typic::hir::Candidate for #name #generics {
        type Candidate = Self; // TODO
      }
      
      mod #module_name {
        #(#variant_types)*
      }

      impl #generics typic::hir::Type for #name #generics {
        type Padding = typic::hir::padding::Padded;
        type Representation = #representation;
      }
    })
    .into();
    println!("{}", f.to_string());
    return f;
}

#[proc_macro_attribute]
pub fn repr(args: TokenStream, input: TokenStream) -> TokenStream {
    let definition: syn::Item = parse_macro_input!(input);

    match definition {
        syn::Item::Struct(definition) => impl_struct(definition),
        syn::Item::Union(definition) => impl_union(definition),
        syn::Item::Enum(definition) => impl_enum(args, definition),
        _ => unimplemented!(),
    }
}
