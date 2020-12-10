use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_fn(&mut self, node: &syn::ItemFn) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemFn {
                attrs: attrs.clone(),
                block: Box::new(util::empty_block()),
                ..node.clone()
            });

            self.feature.assert_stable().visit_item_fn(&syn::ItemFn {
                attrs,
                ..node.clone()
            })
        } else {
            self.feature.assert_stable().visit_item_fn(node)
        }
    }
}
