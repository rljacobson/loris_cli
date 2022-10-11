# Loris

### A Term Rewriting and Computer Algebra System

<img src="loris.jpg" alt="Loris" style="zoom: 33%;" />

This is the TUI for Loris, a term rewriting and computer algebra system. The system is implemented as [lorislib](https://github.com/rljacobson/lorislib), 
which this project brings in as a dependency. 

## Building

There are a few rough edges. One of those rough edges is a dependency of lorislib on a broken crate, 
`permutation-generator`. A 
compiler feature was stabilized, requiring an edit to a single line in that dependency. Right now I just have Cargo.
toml pointing to the sources with a fix:
```toml
# in Cargo.toml for lorislib
permutation-generator = {path = "../../permutation_generator"}
```

Likewise, I need to create a repository for my workspace and use Git submodules, but until I do I have the TUI 
pointing to lorislib via hardcoded path in Cargo.toml:
```toml
# in Cargo.toml for loris
lorislib = {path = "../lorislib"}
```

## Using

Uses syntax similar to Mathematica, except in function definitions the variable appears on both the RHS and LHS:

```mma
f[x_] := 1 + x_^2
```

Only a handful of built-ins and predefined functions exist so far. Expect only basic arithmetic functions and 
system-related built-ins to work. 

Another major limitation is that matcher only has linear matching implemented so far, meaning that a variable can 
only appear one time on the LHS of a function definition.

# License

Copyright (C) 2022 Robert Jacobson. This software is distributed under the 2-Clause BSD License.

# The Slow Loris

The Slow Loris is at risk of extinction due to habitat loss and the illegal wildlife trade. There is demand for 
lorises for use in traditional medicine and as pets. 
