use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Result};

pub fn implement_enum_is(input: DeriveInput) -> Result<TokenStream> {
    if let Data::Enum(data_enum) = &input.data {
        let enum_ident = &input.ident;
        let mut functions = Vec::with_capacity(data_enum.variants.len());

        for variant in &data_enum.variants {
            let variant_ident = &variant.ident;
            let method_name_str = format!("is_{}", variant_ident.to_string().to_snake_case());
            let method_name = syn::Ident::new(&method_name_str, variant_ident.span());

            functions.push(quote! {
                pub const fn #method_name(&self) -> bool {
                    matches!(self, #enum_ident::#variant_ident { .. })
                }
            })
        }

        #[cfg(feature = "impl-enuminfo")]
        let enuminfo_quote = quote! {
            impl enuminfo::EnumIs for #enum_ident {}
        };

        #[cfg(not(feature = "impl-enuminfo"))]
        let enuminfo_quote = quote! {};

        // Build final code
        let enum_ident = &input.ident;
        let expanded = quote! {
            impl #enum_ident {
                #(#functions)*
            }

            #enuminfo_quote
        };

        Ok(expanded)
    } else {
        Err(Error::new(input.span(), "This macro only works on Enums"))
    }
}
