use super::*;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_impl(&mut self, node: &syn::ItemImpl) {
        impl<'a, 'ast> Visit<'ast> for FilteredUnstableItemVisitor<'a, syn::ImplItem> {
            fn visit_impl_item_const(&mut self, node: &'ast syn::ImplItemConst) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::ImplItemConst {
                        attrs: attrs.clone(),
                        expr: util::empty_expr(),
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable(node)
                        .visit_impl_item_const(&syn::ImplItemConst {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable(node).visit_impl_item_const(node)
                }
            }

            fn visit_impl_item_macro(&mut self, node: &'ast syn::ImplItemMacro) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::ImplItemMacro {
                        attrs: attrs.clone(),
                        mac: syn::Macro {
                            tokens: Default::default(),
                            ..node.mac.clone()
                        },
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable(node)
                        .visit_impl_item_macro(&syn::ImplItemMacro {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable(node).visit_impl_item_macro(node)
                }
            }

            fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::ImplItemMethod {
                        attrs: attrs.clone(),
                        // Retain the default block, but clear out its value
                        // That way we'll output items with default impls differently to those without
                        block: util::empty_block(),
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable(node)
                        .visit_impl_item_method(&syn::ImplItemMethod {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature
                        .assert_stable(node)
                        .visit_impl_item_method(node)
                }
            }

            fn visit_impl_item_type(&mut self, node: &'ast syn::ImplItemType) {
                if self.feature.is_unstable(&node.attrs, None) {
                    let attrs = self.feature.strip_attrs(&node.attrs);

                    self.visit_unstable_item(syn::ImplItemType {
                        attrs: attrs.clone(),
                        ..node.clone()
                    });

                    self.feature
                        .assert_stable(node)
                        .visit_impl_item_type(&syn::ImplItemType {
                            attrs,
                            ..node.clone()
                        })
                } else {
                    self.feature.assert_stable(node).visit_impl_item_type(node)
                }
            }
        }

        let is_unstable = self.feature.is_unstable(&node.attrs, None);
        let mut visitor = FilteredUnstableItemVisitor {
            feature: Feature {
                name: self.feature.name,
                inherited: is_unstable,
            },
            // If the trait itself is unstable then its items will inherit
            // that stability
            items: vec![],
        };
        visitor.visit_item_impl(node);

        // A stable trait impl will always be stable on a stable item but can contain unstable items
        if visitor.is_unstable() {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemImpl {
                attrs: attrs.clone(),
                items: visitor.items,
                ..node.clone()
            });
        }
    }
}
