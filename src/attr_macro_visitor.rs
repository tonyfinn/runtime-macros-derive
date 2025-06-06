use std::panic::AssertUnwindSafe;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::visit::Visit;
use syn::Attribute;

struct MacroAttrs {
    tokens: TokenStream,
}

impl Parse for MacroAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MacroAttrs {
            tokens: input.parse()?,
        })
    }
}

pub struct AttributeMacroVisitor<F> {
    macro_path: syn::Path,
    macro_fn: AssertUnwindSafe<F>,
}

impl<F> AttributeMacroVisitor<F> {
    pub fn new(macro_path: syn::Path, macro_fn: AssertUnwindSafe<F>) -> Self {
        Self {
            macro_path,
            macro_fn,
        }
    }

    fn expand_item(&self, attrs: &[Attribute], item: &impl ToTokens)
    where
        F: Fn(TokenStream, TokenStream) -> TokenStream,
    {
        attrs
            .iter()
            .filter(|attr| *attr.path() == self.macro_path)
            .for_each(|attr| {
                if let Ok(attr) = attr.parse_args::<MacroAttrs>() {
                    // processing attribute macro with argument(s) in parentheses
                    (*self.macro_fn)(attr.tokens, item.to_token_stream())
                } else {
                    // processing attribute macro without parentheses (and arguments)
                    (*self.macro_fn)(TokenStream::new(), item.to_token_stream())
                };
            })
    }
}

impl<'ast, F> Visit<'ast> for AttributeMacroVisitor<F>
where
    F: Fn(TokenStream, TokenStream) -> TokenStream,
{
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_extern_crate(&mut self, i: &'ast syn::ItemExternCrate) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_extern_crate(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_foreign_mod(&mut self, i: &'ast syn::ItemForeignMod) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_foreign_mod(self, i);
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_impl(self, i);
    }

    fn visit_item_macro(&mut self, i: &'ast syn::ItemMacro) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_macro(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_mod(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_static(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_trait(self, i);
    }

    fn visit_item_trait_alias(&mut self, i: &'ast syn::ItemTraitAlias) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_trait_alias(self, i);
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_type(self, i);
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_union(self, i);
    }

    fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
        self.expand_item(&i.attrs, i);
        syn::visit::visit_item_use(self, i);
    }
}
