# Loris

### A Term Rewriting and Computer Algebra System

<img src="loris.jpg" alt="Loris" style="zoom: 33%;" />

This is the TUI for Loris, a term rewriting and computer algebra system. The system is implemented as [lorislib](https://github.com/rljacobson/lorislib), 
which this project brings in as a dependency. This crate provides a [RustyLine](https://crates.io/crates/rustyline)
-based REPL.  When I make a repository for the project as a whole that brings in
lorislib and loris_cli as git submodules, it will be located here: https://github.com/rljacobson/loris.

## Building

There are a few rough edges. Until I create a repository for my workspace I have the TUI 
pointing to lorislib via hardcoded path in Cargo.toml:
```toml
# in Cargo.toml for loris
lorislib = {path = "../lorislib"}
```

## Using

Uses syntax similar to Mathematica:

```mma
f[x_] := 1 + x^2
```

Only a handful of built-ins and predefined functions exist so far. Expect only basic arithmetic functions and 
system-related built-ins to work. 

Another major limitation is that matcher only has linear matching implemented so far, meaning that a variable can 
only appear one time on the LHS of a function definition.

# License

Copyright (C) 2022 Robert Jacobson. This software is distributed under the 2-Clause BSD License.

The source code for the [permutation-generator](https://crates.io/crates/permutation-generator) crate by Thomas 
Villa has been included in its entirety in `lorislib/src/matching/permutation_generator`. The permutaton-generator 
crate is distributed under the MIT license.

# The Slow Loris

The Slow Loris is at risk of extinction due to habitat loss and the illegal wildlife trade. There is demand for 
lorises for use in traditional medicine and as pets. 
