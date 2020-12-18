use super::*;

impl<'a> ModuleVisitor<'a> {
    pub(super) fn visit_item_static(&mut self, node: &syn::ItemStatic) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemStatic {
                attrs: attrs.clone(),
                expr: Box::new(util::empty_expr()),
                ..node.clone()
            });

            self.feature
                .assert_stable(node)
                .visit_item_static(&syn::ItemStatic {
                    attrs,
                    ..node.clone()
                })
        } else {
            self.feature.assert_stable(node).visit_item_static(node)
        }
    }
}
