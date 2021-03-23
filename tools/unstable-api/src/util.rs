use syn::visit::Visit;

use std::path::PathBuf;

pub(crate) fn path_is_str(path: &syn::Path, s: &str) -> bool {
    path.get_ident().map(|ident| ident == s).unwrap_or(false)
}

pub(crate) fn lit_is_str(lit: &syn::Lit, s: &str) -> bool {
    if let syn::Lit::Str(lit) = lit {
        lit.value() == s
    } else {
        false
    }
}

pub(crate) fn empty_block() -> syn::Block {
    syn::Block {
        brace_token: Default::default(),
        stmts: vec![syn::Stmt::Expr(empty_expr())],
    }
}

pub(crate) fn empty_expr() -> syn::Expr {
    // This is just a `..` token, which is technically a valid expression,
    // but looks like a placeholder.
    syn::ExprRange {
        attrs: Vec::new(),
        from: None,
        to: None,
        limits: syn::RangeLimits::HalfOpen(Default::default()),
    }
    .into()
}

pub(crate) trait AttributeExt {
    fn is_unstable(&self, feature: &str) -> bool;
    fn is(&self, attr: &str) -> bool;
    fn mod_path(&self) -> Option<PathBuf>;
}

impl AttributeExt for syn::Attribute {
    fn is(&self, attr: &str) -> bool {
        path_is_str(&self.path, attr)
    }

    fn is_unstable(&self, feature: &str) -> bool {
        struct FeatureVisitor<'a> {
            feature: &'a str,
            matches: bool,
        }

        impl<'a, 'ast> Visit<'ast> for FeatureVisitor<'a> {
            fn visit_meta_name_value(&mut self, node: &'ast syn::MetaNameValue) {
                self.matches |=
                    path_is_str(&node.path, "feature") && lit_is_str(&node.lit, self.feature)
            }
        }

        if path_is_str(&self.path, "unstable") {
            if let Ok(meta) = self.parse_meta() {
                let mut visitor = FeatureVisitor {
                    feature: feature,
                    matches: false,
                };

                visitor.visit_meta(&meta);

                return visitor.matches;
            }
        }

        false
    }

    fn mod_path(&self) -> Option<PathBuf> {
        if path_is_str(&self.path, "path") {
            if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
                lit: syn::Lit::Str(path),
                ..
            })) = self.parse_meta()
            {
                return Some(path.value().into());
            }
        }

        None
    }
}
