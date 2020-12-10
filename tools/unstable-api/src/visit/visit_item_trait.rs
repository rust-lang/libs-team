use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_trait(&mut self, node: &syn::ItemTrait) {
        impl<'a, 'ast> Visit<'ast> for FilteredUnstableItemVisitor<'a, syn::TraitItem> {
            fn visit_trait_item_const(&mut self, node: &'ast syn::TraitItemConst) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::TraitItemConst {
                        attrs: attrs.clone(),
                        // Retain the default block, but clear out its value
                        // That way we'll output items with default impls differently to those without
                        default: node
                            .default
                            .as_ref()
                            .map(|(eq, _)| (*eq, util::empty_expr())),
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable()
                        .visit_trait_item_const(&syn::TraitItemConst {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable().visit_trait_item_const(node)
                }
            }

            fn visit_trait_item_macro(&mut self, node: &'ast syn::TraitItemMacro) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::TraitItemMacro {
                        attrs: attrs.clone(),
                        mac: syn::Macro {
                            tokens: Default::default(),
                            ..node.mac.clone()
                        },
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable()
                        .visit_trait_item_macro(&syn::TraitItemMacro {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable().visit_trait_item_macro(node)
                }
            }

            fn visit_trait_item_method(&mut self, node: &'ast syn::TraitItemMethod) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::TraitItemMethod {
                        attrs: attrs.clone(),
                        // Retain the default block, but clear out its value
                        // That way we'll output items with default impls differently to those without
                        default: node.default.as_ref().map(|_| util::empty_block()),
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable()
                        .visit_trait_item_method(&syn::TraitItemMethod {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable().visit_trait_item_method(node)
                }
            }

            fn visit_trait_item_type(&mut self, node: &'ast syn::TraitItemType) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::TraitItemType {
                        attrs: attrs.clone(),
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable()
                        .visit_trait_item_type(&syn::TraitItemType {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable().visit_trait_item_type(node)
                }
            }
        }

        let is_unstable = self.feature.is_unstable(&node.attrs, Some(&node.vis));
        let mut visitor = FilteredUnstableItemVisitor {
            feature: Feature {
                name: self.feature.name,
                inherited: is_unstable,
            },
            items: vec![],
        };
        visitor.visit_item_trait(node);

        // A trait can be stable but contain unstable methods
        if visitor.is_unstable() {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemTrait {
                attrs: attrs.clone(),
                items: visitor.items,
                ..node.clone()
            });
        }
    }
}
