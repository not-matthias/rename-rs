use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, LitStr, Token};

#[derive(Debug, Default)]
struct RenameArgs {
    name: String,
}

mod keyword {
    syn::custom_keyword!(name);
}

impl Parse for RenameArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::name>()?;
        input.parse::<Token![=]>()?;
        let name: LitStr = input.parse()?;

        Ok(RenameArgs { name: name.value() })
    }
}

/// Renames the structure to the specified name.
///
/// Useful documentation:
/// - https://stackoverflow.com/tags/rust-proc-macros/info
/// - https://github.com/dtolnay/syn/issues/516
/// - https://docs.serde.rs/syn/type.AttributeArgs.html
///
#[proc_macro_attribute]
pub fn rename(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as RenameArgs);
    let mut input = parse_macro_input!(input as syn::ItemStruct);

    // Set the new name
    //
    input.ident = syn::Ident::new(&args.name, input.span());

    // Generate the new token stream
    //
    let expanded = quote! {
        #input
    };

    expanded.into()
}
