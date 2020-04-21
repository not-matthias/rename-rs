use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, LitStr, Token};

#[derive(Debug, Default)]
struct RenameArgs {
    name: Option<String>,
    append: Option<String>,
}

mod keyword {
    syn::custom_keyword!(name);
    syn::custom_keyword!(append);
}

impl RenameArgs {
    /// Parse the 'name' input parameter.
    fn parse_name(input: ParseStream) -> Result<String> {
        input.parse::<keyword::name>()?;
        input.parse::<Token![=]>()?;
        let name: LitStr = input.parse()?;

        Ok(name.value())
    }

    /// Parse the 'append' input parameter.
    fn parse_append(input: ParseStream) -> Result<String> {
        input.parse::<keyword::append>()?;
        input.parse::<Token![=]>()?;
        let name: LitStr = input.parse()?;

        Ok(name.value())
    }
}

impl Parse for RenameArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RenameArgs {
            name: Self::parse_name(input).ok(),
            append: Self::parse_append(input).ok()
        })
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
    let mut result = input.ident.to_string();

    if let Some(name) = args.name {
        result = name;
    }

    if let Some(append) = args.append {
        result.push_str(&append);
    }

    // Create ident
    //
    input.ident = syn::Ident::new(&result, input.span());

    // Generate the new token stream
    //
    let expanded = quote! {
        #input
    };

    expanded.into()
}
