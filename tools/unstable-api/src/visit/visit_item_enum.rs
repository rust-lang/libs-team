use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_enum(&mut self, node: &syn::ItemEnum) {
        impl<'a, 'ast> Visit<'ast> for FilteredUnstableItemVisitor<'a, syn::Variant> {
            fn visit_variant(&mut self, node: &'ast syn::Variant) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::Variant {
                        attrs: attrs.clone(),
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable(node)
                        .visit_variant(&syn::Variant {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable(node).visit_variant(node)
                }
            }
        }

        let is_unstable = self.feature.is_unstable(&node.attrs, Some(&node.vis));
        let mut visitor = FilteredUnstableItemVisitor::<syn::Variant> {
            feature: Feature {
                name: self.feature.name,
                inherited: is_unstable,
            },
            items: vec![],
        };
        visitor.visit_item_enum(node);

        // An enum can be stable but contain unstable variants
        if visitor.is_unstable() {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemEnum {
                attrs: attrs.clone(),
                variants: visitor.items.into_iter().collect(),
                ..node.clone()
            });
        }
    }
}
