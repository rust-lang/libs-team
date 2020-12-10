use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_type(&mut self, node: &syn::ItemType) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemType {
                attrs: attrs.clone(),
                ..node.clone()
            });

            self.feature
                .assert_stable()
                .visit_item_type(&syn::ItemType {
                    attrs,
                    ..node.clone()
                })
        } else {
            self.feature.assert_stable().visit_item_type(node)
        }
    }
}
