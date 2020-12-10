use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_union(&mut self, node: &syn::ItemUnion) {
        let is_unstable = self.feature.is_unstable(&node.attrs, Some(&node.vis));
        let mut visitor = FilteredUnstableItemVisitor::<syn::Field> {
            feature: Feature {
                name: self.feature.name,
                inherited: is_unstable,
            },
            items: vec![],
        };
        visitor.visit_item_union(node);

        // An enum can be stable but contain unstable variants
        if visitor.is_unstable() {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemUnion {
                attrs: attrs.clone(),
                fields: syn::FieldsNamed {
                    named: visitor.items.into_iter().collect(),
                    ..node.fields.clone()
                },
                ..node.clone()
            });
        }
    }
}
