Who said JavaScript cannot be BLAZINGLY FAST?

# mik
`mik` is a JavaScript/TypeScript ahead-of-time compiler, written in Rust and powered by the LLVM backend. It delivers blazingly fast code execution and compact binary sizes.

## Name Choice
The name `mik` is derived from _Miklagarðr_, the Old Norse name for Constantinople, the capital of the Byzantine Empire. _Miklagarðr_ literally means "the great city." `mik` aspires to be as great as Constantinople, akin to the grand city its name evokes.

## Build Locally
To build this compiler locally, you must have LLVM installed.
This project uses `llvm-sys = 211.0.0`, therefore your environment must expose:
```bash
LLVM_SYS_211_PREFIX=/path/to/llvm
```
That directory must be the root of an LLVM toolchain. `llvm-sys` will locate `llvm-config` inside that directory. If `llvm-config` cannot be found, the build will fail. If your system package manager does not ship `llvm-config` for this version, you should build LLVM from source and point `LLVM_SYS_211_PREFIX` at that built toolchain.
