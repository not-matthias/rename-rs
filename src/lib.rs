use proc_macro::TokenStream;

use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, LitStr, Token};

use quote::quote;

#[derive(Debug, Default)]
struct RenameArgs {
    name: Option<String>,
    prepend: Option<String>,
    append: Option<String>,
}

mod keyword {
    syn::custom_keyword!(name);
    syn::custom_keyword!(prepend);
    syn::custom_keyword!(append);
}

macro_rules! parse_keyword {
    ($fn_name: ident, $keyword: path) => {
        /// Parses a specific keyword.
        fn $fn_name (input: ParseStream) -> Result<String> {
            // Parse `$keyword = "<value>"`
            //
            input.parse::<$keyword>()?;
            input.parse::<Token![=]>()?;
            let value: LitStr = input.parse()?;

            // Allow optional ','
            //
            let _ = input.parse::<Token![,]>();

            Ok(value.value())
        }
    };
}

impl RenameArgs {
    parse_keyword!(parse_name, keyword::name);
    parse_keyword!(parse_prepend, keyword::prepend);
    parse_keyword!(parse_append, keyword::append);
}

impl Parse for RenameArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RenameArgs {
            name: Self::parse_name(input).ok(),
            prepend: Self::parse_prepend(input).ok(),
            append: Self::parse_append(input).ok(),
        })
    }
}

/// The implementation for the `rename` attribute macro.
#[proc_macro_attribute]
pub fn rename(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as RenameArgs);
    let mut input = parse_macro_input!(input as syn::ItemStruct);

    // Set the new name
    //
    let mut result = input.ident.to_string();

    // Name
    if let Some(name) = args.name {
        result = name;
    }

    // Prepend
    if let Some(prepend) = args.prepend {
        result.insert_str(0, &prepend);
    }

    // Append
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
