use proc_macro2::TokenStream;

use super::{Feature, FilteredUnstableItemVisitor, ModuleVisitor, Visit, util};

impl<'ast> Visit<'ast> for FilteredUnstableItemVisitor<'_, syn::TraitItem> {
    fn visit_trait_item_const(&mut self, node: &'ast syn::TraitItemConst) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::TraitItem::Const(syn::TraitItemConst {
                attrs: attrs.clone(),
                default: node
                    .default
                    .as_ref()
                    .map(|(eq, _)| (*eq, util::empty_expr())),
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_trait_item_const(&syn::TraitItemConst {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature
                .assert_stable(node)
                .visit_trait_item_const(node);
        }
    }

    fn visit_trait_item_macro(&mut self, node: &'ast syn::TraitItemMacro) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::TraitItem::Macro(syn::TraitItemMacro {
                attrs: attrs.clone(),
                mac: syn::Macro {
                    tokens: TokenStream::default(),
                    ..node.mac.clone()
                },
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_trait_item_macro(&syn::TraitItemMacro {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature
                .assert_stable(node)
                .visit_trait_item_macro(node);
        }
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::TraitItem::Fn(syn::TraitItemFn {
                attrs: attrs.clone(),
                default: node.default.as_ref().map(|_| util::empty_block()),
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_trait_item_fn(&syn::TraitItemFn {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature.assert_stable(node).visit_trait_item_fn(node);
        }
    }

    fn visit_trait_item_type(&mut self, node: &'ast syn::TraitItemType) {
        if self.feature.is_unstable(&node.attrs, None) {
            let attrs = self.feature.strip_attrs(&node.attrs);
            self.visit_unstable_item(syn::TraitItem::Type(syn::TraitItemType {
                attrs: attrs.clone(),
                ..node.clone()
            }));
            self.feature
                .assert_stable(node)
                .visit_trait_item_type(&syn::TraitItemType {
                    attrs,
                    ..node.clone()
                });
        } else {
            self.feature.assert_stable(node).visit_trait_item_type(node);
        }
    }
}

impl ModuleVisitor<'_> {
    pub(super) fn visit_item_trait(&mut self, node: &syn::ItemTrait) {
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
                attrs,
                items: visitor.items,
                ..node.clone()
            });
        }
    }
}
