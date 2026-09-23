use super::{ModuleVisitor, visit};

impl ModuleVisitor<'_> {
    pub(super) fn visit_file(&mut self, node: &syn::File) {
        if self.feature.is_unstable(&node.attrs, None) {
            self.feature.inherited = true;
        }

        visit::visit_file(self, node);
    }
}
