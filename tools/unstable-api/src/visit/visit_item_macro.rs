use super::*;

impl<'a> ModuleVisitor<'a> {
    pub(super) fn visit_item_macro(&mut self, node: &syn::ItemMacro) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemMacro {
                attrs: attrs.clone(),
                mac: syn::Macro {
                    tokens: Default::default(),
                    ..node.mac.clone()
                },
                ..node.clone()
            });

            self.feature
                .assert_stable(node)
                .visit_item_macro(&syn::ItemMacro {
                    attrs,
                    ..node.clone()
                })
        } else {
            self.feature.assert_stable(node).visit_item_macro(node)
        }
    }
}
