use crate::rename_case::RenameCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields, LitStr};

pub fn implement_enum_variants(input: DeriveInput) -> syn::Result<TokenStream> {
    if let Data::Enum(data_enum) = &input.data {
        let enum_ident = &input.ident;

        let mut rename_all = RenameCase::None;
        let mut variants = Vec::with_capacity(data_enum.variants.len());

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
                        } else if meta.path.is_ident("ignore_variants") {
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

            variants.push(quote! {
                #construct_variant,
            });

        }

        #[cfg(feature = "impl-enuminfo")]
        let enuminfo_quote = quote! {
            impl enuminfo::EnumVariants for #enum_ident {
                fn variants() -> &'static [Self]
                where
                    Self: Sized
                {
                    Self::VARIANTS
                }
            }
        };

        #[cfg(not(feature = "impl-enuminfo"))]
        let enuminfo_quote = quote! {};

        let expanded = quote! {
            impl #enum_ident {
                pub const VARIANTS: &'static [#enum_ident] = &[
                        #(#variants)*
                ];
            }

            #enuminfo_quote
        };

        Ok(expanded)
    } else {
        Err(Error::new(input.span(), "This macro only works on Enums"))
    }
}
