## `rewrite_use`

### Usage

```bash
cargo install rewrite_use
```

```bash
rewrite_use <file>
```

### Example

```rust
// example.rs
use std::{env::args, fs::File, io::Read, process};
use syn::{
  parse_file,
  Item::Use,
  ItemUse,
  UseTree::{Glob, Group, Name, Path, Rename},
};
```

```rust
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process;
use syn::parse_file;
use syn::Item::Use;
use syn::ItemUse;
use syn::UseTree::Group;
use syn::UseTree::Glob;
use syn::UseTree::Name;
use syn::UseTree::Path;
use syn::UseTree::Rename;
```