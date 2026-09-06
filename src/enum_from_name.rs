use std::collections::HashSet;
use crate::rename_case::RenameCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields, LitStr};

pub fn implement_enum_from_name(input: DeriveInput) -> syn::Result<TokenStream> {
    if let Data::Enum(data_enum) = &input.data {
        let enum_ident = &input.ident;

        let mut rename_all = RenameCase::None;
        let mut name_arms = Vec::with_capacity(data_enum.variants.len());
        let mut raw_name_arms = Vec::with_capacity(data_enum.variants.len());
        let mut seen_names = HashSet::new();

        // Look for Rename All #[enuminfo(rename_all="XXX")]
        for attr in &input.attrs {
            // 1. Verify attribute es #[enuminfo(...)]
            if attr.path().is_ident("enuminfo") {
                // 2. Process parenthesis content
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("rename_all") {
                        // Parse key = "value"` as
                        // LitStr
                        let value: LitStr = meta.value()?.parse()?;
                        rename_all = RenameCase::from_str(&value.value());
                        Ok(())
                    } else {
                        Err(meta.error("Unknown attribute within enuminfo"))
                    }
                })?;
            }
        }

        // Iter variants
        for variant in &data_enum.variants {
            let mut renamed = None;
            let mut ignore = false;

            // Look for Rename #[enuminfo(rename="XXX")]
            for attr in &variant.attrs {
                if attr.path().is_ident("enuminfo") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("rename") {
                            let value: LitStr = meta.value()?.parse()?;
                            renamed = Some(value);
                            Ok(())
                        } else if meta.path.is_ident("ignore_from_name") {
                            ignore = true;
                            Ok(())
                        } else {
                            Ok(())
                        }
                    })?;
                }
            }

            // Only empty enums allowed
            if ignore {
                continue;
            }
            if !variant.fields.is_empty() {
                return Err(Error::new(
                    variant.span(),
                    "This macro only works on Unit Enums",
                ));
            }

            // Instantation
            let variant_ident = &variant.ident;
            let construct_variant = match &variant.fields {
                Fields::Unit => quote! { #enum_ident::#variant_ident },
                Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
                    quote! { #enum_ident::#variant_ident() }
                }
                Fields::Named(fields) if fields.named.is_empty() => {
                    quote! { #enum_ident::#variant_ident {} }
                }
                _ => {
                    return Err(Error::new(
                        variant.span(),
                        "This macro only works on Unit Enums (variants with fields are not allowed)",
                    ));
                }
            };

            // Calculate name
            let raw_name = variant.ident.to_string();
            let name = if let Some(renamed) = renamed {
                renamed.value()
            } else {
                rename_all.apply(variant.ident.to_string().as_str())
            };

            // Check names
            if !seen_names.insert(name.clone()) {
                return Err(Error::new(
                    variant.span(),
                    format!("Duplicate name '{}' found in enum variants", name),
                ));
            }

            name_arms.push(quote! {
               #name => Some(#construct_variant),
            });

            raw_name_arms.push(quote! {
               #raw_name => Some(#construct_variant),
            });
        }

        #[cfg(not(feature = "skip-inherent"))]
        let inherent = quote!{
            impl #enum_ident {
                pub fn from_name(name: &str) -> Option<#enum_ident> {
                    match name {
                        #(#name_arms)*
                        _ => None,
                    }
                }

                pub fn from_raw_name(name: &str) -> Option<#enum_ident> {
                    match name {
                        #(#raw_name_arms)*
                        _ => None,
                    }
                }
            }
        };

        #[cfg(feature = "skip-inherent")]
        let inherent = quote!{};

        #[cfg(all(feature = "impl-enuminfo", not(feature = "skip-inherent")))]
        let enuminfo_quote = quote! {
            impl enuminfo::EnumFromName for #enum_ident {
                fn from_name(name: &str) -> Option<Self>
                where
                    Self: Sized
                {
                    #enum_ident::from_name(name)
                }

                fn from_raw_name(name: &str) -> Option<Self>
                where
                    Self: Sized
                {
                    #enum_ident::from_raw_name(name)
                }
            }
        };

        #[cfg(all(feature = "impl-enuminfo", feature = "skip-inherent"))]
        let enuminfo_quote = quote! {
            impl enuminfo::EnumFromName for #enum_ident {
                fn from_name(name: &str) -> Option<Self>
                where
                    Self: Sized
                {
                    match name {
                        #(#name_arms)*
                        _ => None,
                    }
                }

                fn from_raw_name(name: &str) -> Option<Self>
                where
                    Self: Sized
                {
                    match name {
                        #(#raw_name_arms)*
                        _ => None,
                    }
                }
            }
        };

        #[cfg(not(feature = "impl-enuminfo"))]
        let enuminfo_quote = quote! {};


        #[cfg(all(feature = "from-str", not(feature = "skip-inherent")))]
        let from_str = quote! {
            impl std::str::FromStr for #enum_ident {
                type Err = enuminfo::error::EnumFromNameError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::from_name(s).ok_or_else(|| {
                        enuminfo::error::EnumFromNameError::new(
                            stringify!(#enum_ident).to_string(),
                            s.to_string(),
                        )
                    })
                }
            }
        };

        #[cfg(all(feature = "from-str", feature = "skip-inherent", feature = "impl-enuminfo"))]
        let from_str = quote! {
            impl std::str::FromStr for #enum_ident {
                type Err = enuminfo::error::EnumFromNameError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    <Self as enuminfo::EnumFromName>::from_name(s).ok_or_else(|| {
                        enuminfo::error::EnumFromNameError::new(
                            stringify!(#enum_ident).to_string(),
                            s.to_string(),
                        )
                    })
                }
            }
        };

        #[cfg(all(feature = "from-str", feature = "skip-inherent", not(feature = "impl-enuminfo")))]
        let from_str = quote! {
            impl std::str::FromStr for #enum_ident {
                type Err = enuminfo::error::EnumFromNameError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let name = match s {
                        #(#name_arms)*
                        _ => None,
                    };
                    name.ok_or_else(|| {
                        enuminfo::error::EnumFromNameError::new(
                            stringify!(#enum_ident).to_string(),
                            s.to_string(),
                        )
                    })
                }
            }
        };

        #[cfg(not(feature = "from-str"))]
        let from_str = quote! {};

        let expanded = quote! {
            #inherent
            #enuminfo_quote
            #from_str
        };

        Ok(expanded)
    } else {
        Err(Error::new(input.span(), "This macro only works on Enums"))
    }
}
