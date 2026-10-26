use std::{fs, path::Path};
use syn::visit::{self, Visit};

#[derive(Default)]
struct References(Vec<String>);

impl<'a> Visit<'a> for References {
    fn visit_path(&mut self, path: &'a syn::Path) {
        self.0.push(path.segments.iter().map(|part| part.ident.to_string()).collect::<Vec<_>>().join("::"));
        visit::visit_path(self, path);
    }
    fn visit_item_use(&mut self, item: &'a syn::ItemUse) {
        fn collect(tree: &syn::UseTree, prefix: &str, out: &mut Vec<String>) {
            match tree {
                syn::UseTree::Path(path) => collect(&path.tree, &format!("{prefix}{}::", path.ident), out),
                syn::UseTree::Group(group) => for tree in &group.items { collect(tree, prefix, out); },
                syn::UseTree::Name(name) => out.push(format!("{prefix}{}", name.ident)),
                syn::UseTree::Rename(name) => out.push(format!("{prefix}{}", name.ident)),
                syn::UseTree::Glob(_) => out.push(prefix.trim_end_matches("::").to_owned()),
            }
        }
        collect(&item.tree, "", &mut self.0);
    }
}

fn references(root: &Path, out: &mut References) {
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() { references(&path, out); }
        else if path.extension().is_some_and(|extension| extension == "rs") {
            out.visit_file(&syn::parse_file(&fs::read_to_string(path).unwrap()).unwrap());
        }
    }
}

#[test]
fn schedule_mechanism_has_no_domain_dependencies() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut paths = References::default();
    references(&repo.join("src"), &mut paths);
    assert!(!paths.0.iter().any(|path| path.starts_with("protocol::")));
    let schedule_manifest = fs::read_to_string(repo.join("Cargo.toml")).unwrap();
    assert!(!schedule_manifest.contains("protocol"));
}
