extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use syn::{parse_macro_input, Ident, LitStr};

/// Macro for parsing a CSS-file into a rust struct with fields for every css rule.
///
/// Currently the crate uses a basic regex-formula to parse the CSS. This is obviously not the
/// proper way to do it but it works for now. The regex should be replaced by a proper css parser
/// when that becomes available.
///
/// The macro will convert css classes using kebab-case to snake_case in rust.
///
/// Example input, a file with the following content:
/// ```css
/// .rule_1 {
///     /* Some rules */
/// }
///
/// .rule-2 {
///     /* Some other rules */
/// }
/// ```
///
/// Example output:
/// ```
/// pub struct CssClasses<'a> {
///     rule_1: &'a str,
///     rule_2: &'a str,
/// }
///
/// pub const C: CssClasses<'static> = CssClasses {
///     rule_1: "rule_1",
///     rule_2: "rule-2",
/// };
/// ```
#[proc_macro]
pub fn css_typegen(tokens: TokenStream) -> TokenStream {
    let re = Regex::new(r"(?m)^ *(\.(?P<class>[\w\-_]+)(::?[\w\-_]+)* *)+\{").unwrap();

    let mut field_decls = quote! {};
    let mut field_inits = quote! {};

    let mut classes = vec![];

    for tree in tokens {
        if let TokenTree::Punct(_) = tree {
            continue;
        }
        let token = tree.into();
        let path = parse_macro_input!(token as LitStr).value();

        let mut file = File::open(&path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        re.captures_iter(&contents)
            .map(|capture| capture.name("class"))
            .flatten()
            .map(|m| m.as_str())
            .map(|class| (class.to_owned(), class.replace("-", "_")))
            .for_each(|item| classes.push(item));
    }

    classes.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
    classes.dedup_by(|(_, a), (_, b)| a == b);

    for (ident, rust_ident) in classes {
        let rust_ident = Ident::new(&rust_ident, Span::call_site());
        field_decls.extend(quote! {
            pub #rust_ident: &'a str,
        });
        field_inits.extend(quote! {
            #rust_ident: #ident,
        });
    }

    let css = quote! {
        pub struct CssClasses<'a> {
            #field_decls
        }

        pub const C: CssClasses<'static> = CssClasses {
            #field_inits
        };
    };

    css.into()
}
