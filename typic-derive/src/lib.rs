#![allow(warnings)]
extern crate proc_macro;

use if_chain::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use std::cmp::Ord;
use std::cmp::{max, min};
use syn;
use syn::visit::Visit;
use syn::{parse_macro_input, parse_quote};
use syn::{Attribute, Lit, Meta, NestedMeta, Visibility};

#[proc_macro_attribute]
pub fn typicrepr(_args: TokenStream, input: TokenStream) -> TokenStream {
    repr(_args, input)
}

fn impl_struct(definition: syn::ItemStruct) -> TokenStream {
    let name = &definition.ident;
    let attrs = &definition.attrs;
    let generics = &definition.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let all_public = definition.fields.iter().all(|field| {
        if let Visibility::Public(_) = field.vis {
            true
        } else {
            false
        }
    });

    let transparent = if all_public {
        quote! {
          unsafe impl #impl_generics typic::Transparent
          for #name #ty_generics #where_clause
          {}
        }
    } else {
        quote! {}
    };

    let mut repr = Repr::default();
    attrs
        .into_iter()
        .for_each(|attr| repr.visit_attribute(attr));

    if let Some(Method::Transparent) = repr.method {
        return (quote! {
          #definition

          #transparent

          impl #impl_generics typic::internal::Type
          for #name #ty_generics #where_clause
          {
            #[doc(hidden)]
            type ReprAlign =
                <#name #ty_generics as typic::internal::Type>::ReprAlign;

            #[doc(hidden)]
            type ReprPacked =
                <#name #ty_generics as typic::internal::Type>::ReprPacked;

            #[doc(hidden)]
            type HighLevel =
                <#name #ty_generics as typic::internal::Type>::HighLevel;
          }
        })
        .into();
    }

    let repr_align = repr
        .align
        .map(|n| format_ident!("U{}", n))
        .unwrap_or(format_ident!("MinAlign"));

    let repr_packed = repr
        .packed
        .map(|n| format_ident!("U{}", n))
        .unwrap_or(format_ident!("MaxAlign"));

    // no repr
    if let None = repr.method {
        return (quote! {
          #definition

          #transparent

          impl #impl_generics typic::internal::Type
          for #name #ty_generics #where_clause
          {
            #[doc(hidden)] type ReprAlign = typic::internal::#repr_align;
            #[doc(hidden)] type ReprPacked = typic::internal::#repr_packed;
            #[doc(hidden)] type HighLevel = Self;
          }
        })
        .into();
    }

    // otherwise, it's a C repr
    assert_eq!(repr.method, Some(Method::C));

    let fields = definition
        .fields
        .iter()
        .map(|field| field.ty.clone())
        .rfold(
            quote! {typic::internal::PNil},
            |rest, field| quote! {typic::internal::PCons<#field, #rest>},
        );

    (quote! {
      #definition

      #transparent

      impl #impl_generics typic::internal::Type
      for #name #ty_generics #where_clause
      {
        #[doc(hidden)] type ReprAlign = typic::internal::#repr_align;
        #[doc(hidden)] type ReprPacked = typic::internal::#repr_packed;
        #[doc(hidden)] type HighLevel = #fields;
      }
    })
    .into()
}

#[proc_macro_attribute]
pub fn repr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: TokenStream2 = args.into();
    let input: TokenStream2 = input.into();
    let definition: syn::Item = parse_quote!(#[repr(#args)] #input);

    match definition {
        syn::Item::Struct(definition) => impl_struct(definition),
        _ => unimplemented!(),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Method {
    C,
    Packed,
    Transparent,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Size {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
struct Repr {
    method: Option<Method>,
    align: Option<u32>,
    packed: Option<u32>,
    size: Option<Size>,
}

impl<'ast> Visit<'ast> for Repr {
    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        if_chain! {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta();
            if let Some(ident) = meta_list.path.get_ident();
            if ident.to_string() == "repr";
            then {
                for meta in meta_list.nested {
                    match meta {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            let ident = (if let Some(ident) = path.get_ident() {
                                ident.to_string()
                            } else {
                                continue;
                            });

                            match &ident[..] {
                                "C" =>
                                    self.method = Some(Method::C),
                                "transparent" =>
                                    self.method = Some(Method::Transparent),

                                "packed" =>
                                    self.packed = self.packed.min(Some(1)),

                                "i8"    => self.size = Some(Size::I8),
                                "i16"   => self.size = Some(Size::I16),
                                "i32"   => self.size = Some(Size::I32),
                                "i64"   => self.size = Some(Size::I64),
                                "i132"  => self.size = Some(Size::I128),
                                "isize" => self.size = Some(Size::ISize),
                                "u8"    => self.size = Some(Size::U8),
                                "u16"   => self.size = Some(Size::U16),
                                "u32"   => self.size = Some(Size::U32),
                                "u64"   => self.size = Some(Size::U64),
                                "u132"  => self.size = Some(Size::U128),
                                "usize" => self.size = Some(Size::USize),
                                _ => {},
                            }
                        },
                        NestedMeta::Meta(Meta::List(meta_list)) => {
                            if_chain! {
                                if let Some(ident) = meta_list.path.get_ident();
                                if meta_list.nested.len() == 1;
                                if let Some(n) =  meta_list.nested.first();
                                if let NestedMeta::Lit(Lit::Int(n)) = n;
                                let ident = ident.to_string();
                                if let Ok(n) = n.base10_parse::<u32>();
                                then {
                                    match &ident[..] {
                                        "align" => {
                                            self.align = self.align.max(Some(n));
                                        },
                                        "packed" => {
                                             self.packed = self.packed.min(Some(n));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {},
                    }
                }
            }
        }
    }
}
