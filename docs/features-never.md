This file lists features from JavaScript/TypeScript that will **never** be accepted in the `mik` compiler.
* All DOM methods — the compiler does not support host ambient objects
* `eval` — the compiler generates instructions that are determined at compile time, not at runtime
* `with` — deprecated
* `var` — removing it ensures code predictability. Use `let` or `const` instead
