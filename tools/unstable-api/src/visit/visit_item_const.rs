use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_const(&mut self, node: &syn::ItemConst) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemConst {
                attrs: attrs.clone(),
                expr: Box::new(util::empty_expr()),
                ..node.clone()
            });

            self.feature
                .assert_stable(node)
                .visit_item_const(&syn::ItemConst {
                    attrs,
                    ..node.clone()
                })
        } else {
            self.feature.assert_stable(node).visit_item_const(node)
        }
    }
}
