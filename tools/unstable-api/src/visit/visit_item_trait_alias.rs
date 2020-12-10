use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_trait_alias(&mut self, node: &syn::ItemTraitAlias) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemTraitAlias {
                attrs: attrs.clone(),
                ..node.clone()
            });

            self.feature
                .assert_stable()
                .visit_item_trait_alias(&syn::ItemTraitAlias {
                    attrs,
                    ..node.clone()
                })
        } else {
            self.feature.assert_stable().visit_item_trait_alias(node)
        }
    }
}
