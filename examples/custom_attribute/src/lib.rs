use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::{parse_quote, Expr, Ident, Item, ItemMod, Token};

#[derive(Debug)]
struct AttrEntry {
    name: Ident,
    value: Expr,
}

impl Parse for AttrEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let value = match input.peek(Token![=]) {
            true => {
                let _: Token![=] = input.parse()?;
                input.parse()?
            }
            false => parse_quote! { true },
        };

        Ok(Self { name, value })
    }
}

impl AttrEntry {
    fn emit(self) -> Item {
        let name = &self.name;
        let value = &self.value;

        parse_quote! {
           pub fn #name() -> bool {
                #value
            }
        }
    }
}

struct CustomAttr {
    names: Punctuated<AttrEntry, Token![,]>,
}

impl Parse for CustomAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(CustomAttr {
            names: input.parse_terminated(AttrEntry::parse)?,
        })
    }
}

impl CustomAttr {
    fn emit(self) -> impl Iterator<Item = Item> {
        self.names.into_iter().map(AttrEntry::emit)
    }
}

#[proc_macro_attribute]
pub fn custom_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    custom_attr_internal(attr.into(), item.into()).into()
}

fn custom_attr_internal(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let names: CustomAttr = syn::parse2(attr).unwrap();
    let mut module: ItemMod = syn::parse2(item).unwrap();

    let mut content = names.emit().peekable();

    if let Some((_, items)) = &mut module.content {
        items.extend(content);
    } else if content.peek().is_some() {
        let brace = Brace::default();
        let items = content.collect();
        module.content = Some((brace, items));
    }

    quote!(#module)
}

#[cfg(test)]
mod tests {
    use runtime_macros_derive::emulate_attribute_expansion_fallible;

    use super::custom_attr_internal;
    use std::{env, fs};

    #[test]
    fn code_coverage() {
        let mut path = env::current_dir().unwrap();
        path.push("tests");
        path.push("tests.rs");
        let file = fs::File::open(path).unwrap();
        emulate_attribute_expansion_fallible(file, "custom_attr", custom_attr_internal).unwrap();
    }

    #[test]
    fn syntax_error() {
        // This code makes sure that the given file doesn't compile.
        let mut path = env::current_dir().unwrap();
        path.push("tests");
        path.push("compile-fail");
        path.push("syntax_error.rs");
        let file = fs::File::open(path).unwrap();
        assert!(
            emulate_attribute_expansion_fallible(file, "custom_attr", custom_attr_internal)
                .is_err()
        );
    }
}
