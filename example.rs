use std::{env::args, fs::File, io::Read, process};
use syn::{
  parse_file,
  Item::Use,
  ItemUse,
  UseTree::{Glob, Group, Name, Path, Rename},
};