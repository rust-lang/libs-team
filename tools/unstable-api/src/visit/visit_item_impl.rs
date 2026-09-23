use proc_macro2::TokenStream;

use super::{Feature, FilteredUnstableItemVisitor, ModuleVisitor, Visit, util};

impl<'ast> Visit<'ast> for FilteredUnstableItemVisitor<'_, syn::ImplItem> {
    fn visit_impl_item_const(&mut self, node: &'ast syn::ImplItemConst) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::ImplItem::Const(syn::ImplItemConst {
                attrs: attrs.clone(),
                expr: util::empty_expr(),
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_impl_item_const(&syn::ImplItemConst {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature.assert_stable(node).visit_impl_item_const(node);
        }
    }

    fn visit_impl_item_macro(&mut self, node: &'ast syn::ImplItemMacro) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::ImplItem::Macro(syn::ImplItemMacro {
                attrs: attrs.clone(),
                mac: syn::Macro {
                    tokens: TokenStream::default(),
                    ..node.mac.clone()
                },
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_impl_item_macro(&syn::ImplItemMacro {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature.assert_stable(node).visit_impl_item_macro(node);
        }
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::ImplItem::Fn(syn::ImplItemFn {
                attrs: attrs.clone(),
                block: util::empty_block(),
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_impl_item_fn(&syn::ImplItemFn {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature.assert_stable(node).visit_impl_item_fn(node);
        }
    }

    fn visit_impl_item_type(&mut self, node: &'ast syn::ImplItemType) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::ImplItem::Type(syn::ImplItemType {
                attrs: attrs.clone(),
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_impl_item_type(&syn::ImplItemType {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature.assert_stable(node).visit_impl_item_type(node);
        }
    }
}

struct ImplTraitForTypeVisitor<'a, 'b>(&'b mut FilteredUnstableItemVisitor<'a, syn::ImplItem>);

impl<'ast> Visit<'ast> for ImplTraitForTypeVisitor<'_, '_> {
    fn visit_impl_item_const(&mut self, _node: &'ast syn::ImplItemConst) {}

    fn visit_impl_item_macro(&mut self, node: &'ast syn::ImplItemMacro) {
        self.0.visit_impl_item_macro(node);
    }

    fn visit_impl_item_fn(&mut self, _node: &'ast syn::ImplItemFn) {}

    fn visit_impl_item_type(&mut self, node: &'ast syn::ImplItemType) {
        self.0.visit_impl_item_type(node);
    }
}

impl ModuleVisitor<'_> {
    pub(super) fn visit_item_impl(&mut self, node: &syn::ItemImpl) {
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

        if node.trait_.is_some() {
            // 'impl Trait for Type { .. }', where only types are relevant.
            ImplTraitForTypeVisitor(&mut visitor).visit_item_impl(node);
        } else {
            // 'impl Type { .. }', where all items are relevant.
            visitor.visit_item_impl(node);
        }

        // A stable trait impl will always be stable on a stable item but can contain unstable items
        if visitor.is_unstable() {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemImpl {
                attrs,
                items: visitor.items,
                ..node.clone()
            });
        }
    }
}
