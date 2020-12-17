use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_macro2(&mut self, node: &syn::ItemMacro2) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemMacro2 {
                attrs: attrs.clone(),
                rules: Default::default(),
                ..node.clone()
            });

            self.feature
                .assert_stable(node)
                .visit_item_macro2(&syn::ItemMacro2 {
                    attrs,
                    ..node.clone()
                })
        } else {
            self.feature.assert_stable(node).visit_item_macro2(node)
        }
    }
}
