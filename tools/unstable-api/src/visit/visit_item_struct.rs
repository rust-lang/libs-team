use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_struct(&mut self, node: &syn::ItemStruct) {
        impl<'a, 'ast> Visit<'ast> for FilteredUnstableItemVisitor<'a, syn::Field> {
            fn visit_field(&mut self, node: &'ast syn::Field) {
                if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::Field {
                        attrs: attrs.clone(),
                        ..node.clone()
                    });

                    self.feature.assert_stable().visit_field(&syn::Field {
                        attrs,
                        ..node.clone()
                    })
                } else {
                    self.feature.assert_stable().visit_field(node)
                }
            }
        }

        let is_unstable = self.feature.is_unstable(&node.attrs, Some(&node.vis));
        let mut visitor = FilteredUnstableItemVisitor::<syn::Field> {
            feature: Feature {
                name: self.feature.name,
                inherited: is_unstable,
            },
            items: vec![],
        };
        visitor.visit_item_struct(node);

        // An enum can be stable but contain unstable variants
        if visitor.is_unstable() {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemStruct {
                attrs: attrs.clone(),
                fields: match &node.fields {
                    syn::Fields::Named(fields) => syn::Fields::Named(syn::FieldsNamed {
                        named: visitor.items.into_iter().collect(),
                        ..fields.clone()
                    }),
                    syn::Fields::Unnamed(fields) => syn::Fields::Unnamed(syn::FieldsUnnamed {
                        unnamed: visitor.items.into_iter().collect(),
                        ..fields.clone()
                    }),
                    syn::Fields::Unit => syn::Fields::Unit,
                },
                ..node.clone()
            });
        }
    }
}
