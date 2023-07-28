[Crates and source files](https://doc.rust-lang.org/reference/crates-and-source-files.html)

# Crates and source files
Each compilation processes a single crate in source form, and if successful, produces a single crate in binary form: either an executable or some sort of library.
A crate contains a tree of nested module scopes. Any item within a crate has a canonical module path denoting its location within the crate's module tree.

Every source file is a module, but not every module needs its own source file: module definitions can be nested within one file.