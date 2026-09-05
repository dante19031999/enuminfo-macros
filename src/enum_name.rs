use std::collections::HashSet;
use crate::rename_case::RenameCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, LitStr};

pub fn implement_enum_name(input: DeriveInput) -> syn::Result<TokenStream> {
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
                    }else {
                        Ok(())
                    }
                })?;
            }
        }

        // Iter variants
        for variant in &data_enum.variants {
            let mut renamed = None;

            // Look for Rename #[enuminfo(rename="XXX")]
            for attr in &variant.attrs {
                if attr.path().is_ident("enuminfo") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("rename") {
                            let value: LitStr = meta.value()?.parse()?;
                            renamed = Some(value);
                            Ok(())
                        } else {
                            Err(meta.error("Unknown attribute within enuminfo"))
                        }
                    })?;
                }
            }

            // Calculate name
            let variant_ident = &variant.ident;
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
                #enum_ident::#variant_ident { .. } => #name,
            });

            raw_name_arms.push(quote! {
                #enum_ident::#variant_ident { .. } => #raw_name,
            });
        }

        #[cfg(feature = "impl-enuminfo")]
        let enuminfo_quote = quote! {
            impl enuminfo::EnumName for #enum_ident {
                fn name(&self) -> &'static str {
                    #enum_ident::name(self)
                }

                fn raw_name(&self) -> &'static str {
                    #enum_ident::raw_name(self)
                }
            }
        };

        #[cfg(not(feature = "impl-enuminfo"))]
        let enuminfo_quote = quote! {};

        let expanded = quote! {
            impl #enum_ident {
                pub const fn name(&self) -> &'static str {
                    match self {
                        #(#name_arms)*
                    }
                }

                pub const fn raw_name(&self) -> &'static str {
                    match self {
                        #(#raw_name_arms)*
                    }
                }
            }

            #enuminfo_quote
        };

        Ok(expanded)
    } else {
        Err(Error::new(input.span(), "This macro only works on Enums"))
    }
}
