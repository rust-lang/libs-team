use super::*;

impl<'a> ModuleVisitor<'a> {
    pub(super) fn visit_item_use(&mut self, node: &syn::ItemUse) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemUse {
                attrs: attrs.clone(),
                ..node.clone()
            });

            self.feature
                .assert_stable(node)
                .visit_item_use(&syn::ItemUse {
                    attrs,
                    ..node.clone()
                })
        }
        // `use` statements may be unstable without being public
        else if let syn::Visibility::Public(_) = node.vis {
            self.feature.assert_stable(node).visit_item_use(node)
        } else {
            visit::visit_item_use(self, node)
        }
    }
}
