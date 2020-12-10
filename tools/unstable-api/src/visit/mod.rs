use anyhow::{anyhow, ensure, Context, Error};
use syn::{
    export::ToTokens,
    ext::IdentExt,
    visit::{self, Visit},
};

use std::{fs, path::PathBuf};

use crate::util::{self, AttributeExt};

mod visit_item_const;
mod visit_item_enum;
mod visit_item_fn;
mod visit_item_impl;
mod visit_item_macro;
mod visit_item_macro2;
mod visit_item_mod;
mod visit_item_static;
mod visit_item_struct;
mod visit_item_trait;
mod visit_item_trait_alias;
mod visit_item_type;
mod visit_item_union;
mod visit_item_use;

pub fn pub_unstable(mut crate_root: PathBuf, feature: &str) -> Result<(), Error> {
    let crate_name = crate_root
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| anyhow!("missing crate root name"))?
        .to_owned();
    crate_root.push("src");
    crate_root.push("lib.rs");

    let mut visitor = UnstableVisitor::new(
        crate_name,
        crate_root,
        Feature {
            name: feature,
            inherited: false,
        },
    );
    visitor.visit()?;

    Ok(())
}

#[derive(Debug)]
struct UnstableVisitor<'a> {
    feature: Feature<'a>,
    root_file_path: PathBuf,
    mod_file_path: PathBuf,
    current_mod: String,
    inline_mods: Vec<String>,
    discovered_modules: Vec<DiscoveredModule>,
}

#[derive(Debug)]
struct DiscoveredModule {
    name: String,
    parents: Vec<String>,
    path: Option<PathBuf>,
}

impl<'a, 'ast> Visit<'ast> for UnstableVisitor<'a> {
    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.visit_item_const(node)
    }

    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.visit_item_enum(node)
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.visit_item_fn(node)
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        self.visit_item_impl(node)
    }

    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        self.visit_item_macro(node)
    }

    fn visit_item_macro2(&mut self, node: &'ast syn::ItemMacro2) {
        self.visit_item_macro2(node)
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.visit_item_mod(node)
    }

    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.visit_item_static(node)
    }

    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.visit_item_struct(node)
    }

    fn visit_item_trait_alias(&mut self, node: &'ast syn::ItemTraitAlias) {
        self.visit_item_trait_alias(node)
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.visit_item_trait(node)
    }

    fn visit_item_type(&mut self, node: &'ast syn::ItemType) {
        self.visit_item_type(node)
    }

    fn visit_item_union(&mut self, node: &'ast syn::ItemUnion) {
        self.visit_item_union(node)
    }

    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        self.visit_item_use(node)
    }
}

impl<'a> UnstableVisitor<'a> {
    fn new(current_mod: String, mod_file_path: PathBuf, feature: Feature<'a>) -> Self {
        UnstableVisitor {
            feature,
            root_file_path: {
                match mod_file_path.file_stem().and_then(|stem| stem.to_str()) {
                    // For `mod.rs` and `lib.rs` we set the root path to `./`
                    Some("mod") | Some("lib") => {
                        let mut root_file_path = mod_file_path.clone();
                        root_file_path.pop();
                        root_file_path
                    }
                    // For `x.rs` we set the root path to `./x`
                    _ => {
                        let mut root_file_path = mod_file_path.clone();
                        root_file_path.set_extension("");
                        root_file_path
                    }
                }
            },
            mod_file_path,
            inline_mods: vec![],
            current_mod,
            discovered_modules: vec![],
        }
    }

    fn visit(&mut self) -> Result<(), Error> {
        let content = fs::read_to_string(&self.mod_file_path)
            .context(format!("reading {:?}", self.mod_file_path))?;

        self.visit_file(&syn::parse_file(&content)?);

        while let Some(mut next) = self.resolve_next_mod_file_path()? {
            next.visit()?;
        }

        Ok(())
    }

    fn visit_unstable_item(&mut self, item: impl ToTokens) {
        println!("// mod {}", self.current_mod);
        println!("{}", item.into_token_stream());
        println!();
    }

    fn resolve_next_mod_file_path(&mut self) -> Result<Option<UnstableVisitor<'a>>, Error> {
        if let Some(next) = self.discovered_modules.pop() {
            let next_mod = {
                let mut next_mod = self.current_mod.clone();

                for inline_mod in self.inline_mods.iter().chain(Some(&next.name)) {
                    next_mod.push_str("::");
                    next_mod.push_str(inline_mod);
                }

                next_mod
            };

            if let Some(path) = next.path {
                let path = self.root_file_path.join(path);

                ensure!(
                    path.exists(),
                    "could not find module `{}` at its given path {:?}",
                    next.name,
                    path
                );

                return Ok(Some(UnstableVisitor::new(
                    next_mod,
                    path.clone(),
                    self.feature,
                )));
            }

            // If there are inline parent modules then include them in the path
            let mut root_file_path = self.root_file_path.clone();
            for parent in next.parents {
                root_file_path.push(parent);
            }

            let paths_to_try = vec![
                {
                    // {root_file_path}/{next}.rs
                    let mut alongside = root_file_path.clone();
                    alongside.push(&next.name);
                    alongside.set_extension("rs");
                    alongside
                },
                {
                    // {root_file_path}/{next}/mod.rs
                    let mut nested = root_file_path.clone();
                    nested.push(&next.name);
                    nested.push("mod.rs");
                    nested
                },
            ];

            for path in &paths_to_try {
                if path.exists() {
                    return Ok(Some(UnstableVisitor::new(
                        next_mod,
                        path.clone(),
                        self.feature,
                    )));
                }
            }

            Err(anyhow!(
                "could not find module `{}` in any of {:?} (this is a bug)",
                next_mod,
                paths_to_try
            ))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Feature<'a> {
    name: &'a str,
    inherited: bool,
}

impl<'a> Feature<'a> {
    fn is_unstable(self, attrs: &[syn::Attribute], vis: Option<&syn::Visibility>) -> bool {
        match vis {
            // If no visibility is given we assume it's public
            // If stability is inherited then always consider the item unstable
            Some(syn::Visibility::Public(_)) | None => {
                self.inherited || attrs.iter().any(|attr| attr.is_unstable(self.name))
            }
            _ => false,
        }
    }

    fn strip_attrs(self, attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
        attrs
            .iter()
            .filter(|attr| attr.is("derive") || attr.is("fundamental"))
            .cloned()
            .collect()
    }

    fn assert_stable(self) -> AssertStableVisitor<'a> {
        AssertStableVisitor {
            feature: Feature {
                name: self.name,
                inherited: false,
            },
        }
    }
}

#[derive(Debug)]
struct AssertStableVisitor<'a> {
    feature: Feature<'a>,
}

impl<'a, 'ast> Visit<'ast> for AssertStableVisitor<'a> {
    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        assert!(
            !node.is_unstable(self.feature.name),
            "encountered an unexpected unstable attribute (this is a bug)"
        );
    }
}

#[derive(Debug)]
struct FilteredUnstableItemVisitor<'a, T> {
    feature: Feature<'a>,
    items: Vec<T>,
}

impl<'a, T> FilteredUnstableItemVisitor<'a, T> {
    fn is_unstable(&self) -> bool {
        self.feature.inherited || self.items.len() > 0
    }

    fn visit_unstable_item(&mut self, item: impl Into<T>) {
        self.items.push(item.into());
    }
}
