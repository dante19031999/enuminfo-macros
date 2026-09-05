use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Result};

pub fn implement_enum_class(input: DeriveInput) -> Result<TokenStream> {
    if let Data::Enum(_) = &input.data {
        let enum_ident = &input.ident;

        // Build enuminfo
        #[cfg(feature = "impl-enuminfo")]
        let enuminfo_quote = quote! {
            impl enuminfo::EnumClass for #enum_ident {
                fn class() -> &'static str
                where
                    Self: Sized
                {
                    Self::class()
                }
            }
        };

        #[cfg(not(feature = "impl-enuminfo"))]
        let enuminfo_quote = quote! {};

        // Build final code
        let enum_name = enum_ident.to_string();
        let expanded = quote! {
            impl #enum_ident {
                fn class() -> &'static str {
                    #enum_name
                }
            }

            #enuminfo_quote
        };

        Ok(expanded)
    } else {
        Err(Error::new(input.span(), "This macro only works on Enums"))
    }
}
