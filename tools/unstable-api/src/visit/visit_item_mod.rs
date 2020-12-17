use super::*;

use std::mem;

impl<'a> UnstableVisitor<'a> {
    pub(super) fn visit_item_mod(&mut self, node: &syn::ItemMod) {
        if self.feature.is_unstable(&node.attrs, Some(&node.vis)) {
            let attrs = self.feature.strip_attrs(&node.attrs);

            self.visit_unstable_item(syn::ItemMod {
                attrs: attrs.clone(),
                content: Some((Default::default(), vec![])),
                ..node.clone()
            });
        }

        // If the module isn't an inline item then it will have a file to find
        if node.content.is_none() {
            let path = node.attrs.iter().filter_map(|attr| attr.mod_path()).next();

            self.discovered_modules.push(DiscoveredModule {
                name: node.ident.unraw().to_string(),
                inherit_feature: self.feature.is_unstable(&node.attrs, Some(&node.vis)),
                parents: self.inline_modules.clone(),
                path,
            });

            visit::visit_item_mod(self, node);
        }
        // If the node is an inline item then it may contain more unstable items
        // It may also contain external child modules
        else {
            // Capture the current inherited feature status
            // This will be restored at the end
            let inherit_feature = {
                let inherited = self.feature.is_unstable(&node.attrs, Some(&node.vis));
                mem::replace(&mut self.feature.inherited, inherited)
            };
            self.inline_modules.push(node.ident.unraw().to_string());

            visit::visit_item_mod(self, node);

            self.inline_modules.pop();
            self.feature.inherited = inherit_feature
        }
    }
}
