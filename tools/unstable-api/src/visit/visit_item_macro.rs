use proc_macro2::TokenStream;

use super::{ModuleVisitor, Visit};

impl ModuleVisitor<'_> {
    pub(super) fn visit_item_macro(&mut self, node: &syn::ItemMacro) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemMacro {
                attrs: attrs.clone(),
                mac: syn::Macro {
                    tokens: TokenStream::default(),
                    ..node.mac.clone()
                },
                ..node.clone()
            });

            self.feature
                .assert_stable(node)
                .visit_item_macro(&syn::ItemMacro {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature.assert_stable(node).visit_item_macro(node);
        }
    }
}
