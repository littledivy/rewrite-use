use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
  let mut args = env::args();
  let _ = args.next();

  let filename = match (args.next(), args.next()) {
    (Some(filename), None) => filename,
    _ => {
      eprintln!("Usage: rewrite-use path/to/filename.rs");
      process::exit(1);
    }
  };

  let mut file = File::open(&filename).expect("Unable to open file");

  let mut src = String::new();
  file.read_to_string(&mut src).expect("Unable to read file");

  let syntax = syn::parse_file(&src).expect("Unable to parse file");

  for item in syntax.items {
    match item {
      syn::Item::Use(syn::ItemUse { tree, .. }) => {
        let mut paths = vec![];
        fn flatten(tree: &syn::UseTree, prefix: &str, paths: &mut Vec<String>) {
          match tree {
            syn::UseTree::Path(syn::UsePath { ident, tree, .. }) => {
              let prefix = if prefix.is_empty() {
                ident.to_string()
              } else {
                format!("{}::{}", prefix, ident)
              };
              flatten(tree, &prefix, paths);
            }
            syn::UseTree::Name(syn::UseName { ident, .. }) => {
              let path = if prefix.is_empty() {
                ident.to_string()
              } else {
                format!("{}::{}", prefix, ident)
              };
              paths.push(path);
            }
            syn::UseTree::Glob(_) => {
              paths.push(prefix.to_string());
            }
            syn::UseTree::Group(syn::UseGroup { items, .. }) => {
              for item in items {
                flatten(item, prefix, paths);
              }
            }
            syn::UseTree::Rename(syn::UseRename { rename, .. }) => {
              let path = if prefix.is_empty() {
                rename.to_string()
              } else {
                format!("{}::{}", prefix, rename)
              };
              paths.push(path);
            }
          }
        }
        flatten(&tree, "", &mut paths);

        // Print the paths
        for path in paths {
          println!("use {};", path);
        }
      }
      _ => {}
    }
  }
}
