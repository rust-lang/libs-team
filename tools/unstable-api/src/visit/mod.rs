use anyhow::{anyhow, ensure, Context, Error};
use syn::{
    export::{Span, ToTokens, TokenStream2 as TokenStream},
    ext::IdentExt,
    visit::{self, Visit},
};
use quote::quote;

use std::{fs, fmt, path::PathBuf};

use crate::util::{self, AttributeExt};

mod visit_file;
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

    let current_mod = Module {
        original: syn::ItemMod {
            attrs: vec![],
            vis: syn::Visibility::Public(syn::VisPublic { pub_token: Default::default() }),
            mod_token: Default::default(),
            ident: syn::Ident::new(&crate_name, Span::call_site()),
            content: None,
            semi: Some(Default::default()),
        },
        items: vec![],
        children: vec![],
    };

    let mut visitor = ModuleVisitor::new(
        current_mod,
        crate_root,
        Feature {
            name: feature,
            inherited: false,
        },
    );
    visitor.visit_module_file()?;

    if visitor.module.is_unstable() {
        println!("{}", visitor.module.to_token_stream());
    }

    Ok(())
}

#[derive(Debug)]
struct ModuleVisitor<'a> {
    feature: Feature<'a>,
    root_file_path: PathBuf,
    module_file_path: PathBuf,
    module: Module,
    inline_modules: Vec<InlineModule>,
    discovered_modules: Vec<DiscoveredModule>,
}

impl<'a> ModuleVisitor<'a> {
    fn parse_file(&self) -> Result<syn::File, Error> {
        let content = fs::read_to_string(&self.module_file_path)
            .context(format!("reading {:?}", self.module_file_path))?;

        let node = syn::parse_file(&content)?;

        Ok(node)
    }
}

struct Module {
    original: syn::ItemMod,
    items: Vec<TokenStream>,
    children: Vec<Module>,
}

impl Module {
    fn is_unstable(&self) -> bool {
        self.items.len() > 0 || self.children.iter().any(Module::is_unstable)
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Module")
            .field("ident", &self.original.ident)
            .field("items", &self.items.len())
            .field("children", &self.children)
            .finish()
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let syn::ItemMod {
            ref vis,
            ref attrs,
            ref ident,
            ref mod_token,
            ..
        } = self.original;

        let items = &self.items;
        let children = &self.children;

        tokens.extend(quote!(
            #(#attrs)*
            #vis #mod_token #ident {
                #(#children)*

                #(#items)*
            }
        ))
    }
}

#[derive(Debug)]
struct InlineModule {
    name: String,
}

struct DiscoveredModule {
    original: syn::ItemMod,
    name: String,
    // A module may have a `#[unstable]` attribute on it
    // TODO: Also support `#![unstable]`
    inherit_feature: bool,
    path: Option<PathBuf>,
}

impl fmt::Debug for DiscoveredModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Module")
            .field("ident", &self.original.ident)
            .finish()
    }
}

impl<'a, 'ast> Visit<'ast> for ModuleVisitor<'a> {
    fn visit_file(&mut self, node: &'ast syn::File) {
        self.visit_file(node)
    }

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

impl<'a> ModuleVisitor<'a> {
    fn new(module: Module, module_file_path: PathBuf, feature: Feature<'a>) -> Self {
        ModuleVisitor {
            feature,
            root_file_path: {
                match module_file_path.file_stem().and_then(|stem| stem.to_str()) {
                    // For `mod.rs` and `lib.rs` we set the root path to `./`
                    Some("mod") | Some("lib") => {
                        let mut root_file_path = module_file_path.clone();
                        root_file_path.pop();
                        root_file_path
                    }
                    // For `x.rs` we set the root path to `./x`
                    _ => {
                        let mut root_file_path = module_file_path.clone();
                        root_file_path.set_extension("");
                        root_file_path
                    }
                }
            },
            module_file_path,
            inline_modules: vec![],
            module,
            discovered_modules: vec![],
        }
    }

    fn visit_module_file(&mut self) -> Result<(), Error> {
        self.visit_file(&self.parse_file()?);

        while let Some(mut discovered) = self.resolve_next_module_file()? {
            discovered.visit_module_file()?;

            // If the module contains unstable items then retain it
            if discovered.module.is_unstable() {
                self.module.children.push(discovered.module);
            }
        }

        Ok(())
    }

    fn visit_module_inline(&mut self) -> Result<(), Error> {
        visit::visit_item_mod(self, &self.module.original.clone());

        while let Some(mut discovered) = self.resolve_next_module_file()? {
            discovered.visit_module_file()?;

            // If the module contains unstable items then retain it
            if discovered.module.is_unstable() {
                self.module.children.push(discovered.module);
            }
        }

        Ok(())
    }

    fn visit_unstable_item(&mut self, item: impl ToTokens) {
        self.module.items.push(item.to_token_stream());
    }

    fn resolve_next_module_file(&mut self) -> Result<Option<ModuleVisitor<'a>>, Error> {
        if let Some(next) = self.discovered_modules.pop() {
            let next_mod = Module {
                original: next.original,
                items: vec![],
                children: vec![],
            };

            if let Some(path) = next.path {
                let path = self.root_file_path.join(path);

                ensure!(
                    path.exists(),
                    "could not find module `{}` at its given path {:?}",
                    next.name,
                    path
                );

                return Ok(Some(ModuleVisitor::new(
                    next_mod,
                    path.clone(),
                    self.feature.inherit(next.inherit_feature),
                )));
            }

            let paths_to_try = vec![
                {
                    // {root_file_path}/{next}.rs
                    let mut alongside = self.root_file_path.clone();
                    alongside.push(&next.name);
                    alongside.set_extension("rs");
                    alongside
                },
                {
                    // {root_file_path}/{next}/mod.rs
                    let mut nested = self.root_file_path.clone();
                    nested.push(&next.name);
                    nested.push("mod.rs");
                    nested
                },
            ];

            for path in &paths_to_try {
                if path.exists() {
                    return Ok(Some(ModuleVisitor::new(
                        next_mod,
                        path.clone(),
                        self.feature.inherit(next.inherit_feature),
                    )));
                }
            }

            Err(anyhow!(
                "could not find module `{:?}` in any of {:?} (this is a bug)",
                next_mod.original.ident,
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
    fn inherit(self, inherit_feature: bool) -> Feature<'a> {
        Feature {
            name: self.name,
            inherited: inherit_feature,
        }
    }

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

    fn assert_stable(self, node: impl ToTokens) -> AssertStableVisitor<'a> {
        AssertStableVisitor {
            tokens: node.to_token_stream(),
            feature: Feature {
                name: self.name,
                inherited: false,
            },
        }
    }
}

#[derive(Debug)]
struct AssertStableVisitor<'a> {
    tokens: TokenStream,
    feature: Feature<'a>,
}

impl<'a, 'ast> Visit<'ast> for AssertStableVisitor<'a> {
    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        assert!(
            !node.is_unstable(self.feature.name),
            "encountered an unexpected unstable attribute in {} (this is a bug)",
            self.tokens,
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
