use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_use(&mut self, node: &syn::ItemUse) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemUse {
                attrs: attrs.clone(),
                ..node.clone()
            });

            self.feature.assert_stable().visit_item_use(&syn::ItemUse {
                attrs,
                ..node.clone()
            })
        } else {
            self.feature.assert_stable().visit_item_use(node)
        }
    }
}
