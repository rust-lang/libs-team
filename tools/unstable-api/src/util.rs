use syn::{parse::Parser, token::Brace};

use std::path::PathBuf;

pub fn path_is_str(path: &syn::Path, s: &str) -> bool {
    path.get_ident().is_some_and(|ident| ident == s)
}

pub fn empty_block() -> syn::Block {
    syn::Block {
        brace_token: Brace::default(),
        stmts: vec![syn::Stmt::Expr(empty_expr(), None)],
    }
}

pub fn empty_expr() -> syn::Expr {
    // This is just a `..` token, which is technically a valid expression,
    // but looks like a placeholder.
    syn::parse_quote!(..)
}

pub trait AttributeExt {
    fn is_unstable(&self, feature: &str) -> bool;
    fn is(&self, attr: &str) -> bool;
    fn mod_path(&self) -> Option<PathBuf>;
}

impl AttributeExt for syn::Attribute {
    fn is(&self, attr: &str) -> bool {
        path_is_str(self.path(), attr)
    }

    fn is_unstable(&self, feature: &str) -> bool {
        if path_is_str(self.path(), "unstable") {
            let mut matches = false;
            let _ = self.parse_nested_meta(|meta| {
                if path_is_str(&meta.path, "feature") {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    matches |= value.value() == feature;
                }
                Ok(())
            });
            return matches;
        }

        false
    }

    fn mod_path(&self) -> Option<PathBuf> {
        if path_is_str(self.path(), "path")
            && let syn::Meta::NameValue(meta) = &self.meta
            && let syn::Expr::Lit(expr) = &meta.value
            && let syn::Lit::Str(path) = &expr.lit
        {
            return Some(path.value().into());
        }

        if path_is_str(self.path(), "cfg_attr") {
            if let syn::Meta::List(meta) = &self.meta {
                let nested =
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated
                        .parse2(meta.tokens.clone())
                        .ok()?;
                for meta in nested {
                    if let syn::Meta::NameValue(meta) = meta
                        && path_is_str(&meta.path, "path")
                        && let syn::Expr::Lit(expr) = meta.value
                        && let syn::Lit::Str(path) = expr.lit
                    {
                        return Some(path.value().into());
                    }
                }
            }
            return None;
        }

        None
    }
}
