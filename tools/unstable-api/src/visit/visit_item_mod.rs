use super::*;

impl<'a> ModuleVisitor<'a> {
    pub(super) fn visit_item_mod(&mut self, node: &syn::ItemMod) {
        // If the module isn't an inline item then it will have a file to find
        if node.content.is_none() {
            let path = node.attrs.iter().filter_map(|attr| attr.mod_path()).next();

            self.discovered_modules.push(DiscoveredModule {
                original: syn::ItemMod {
                    attrs: self.feature.strip_attrs(&node.attrs),
                    content: Some((Default::default(), vec![])),
                    ..node.clone()
                },
                name: node.ident.unraw().to_string(),
                inherit_feature: self.feature.is_unstable(&node.attrs, Some(&node.vis)),
                path,
            });
        }
        // If the node is an inline item then it may contain more unstable items
        // It may also contain external child modules
        else {
            let mut next = ModuleVisitor::new(
                Module {
                    original: syn::ItemMod {
                        attrs: self.feature.strip_attrs(&node.attrs),
                        ..node.clone()
                    },
                    items: vec![],
                    children: vec![],
                },
                {
                    // Still create a file path for this inline module in case it has any external modules
                    let mut path = self.root_file_path.clone();
                    path.push(node.ident.unraw().to_string());
                    path
                },
                self.feature.inherit(self.feature.is_unstable(&node.attrs, Some(&node.vis))),
            );

            next.visit_module_inline().expect("failed to visit inline module");

            // If the module contains unstable items then retain it
            if next.module.is_unstable() {
                self.module.children.push(next.module);
            }
        }
    }
}
