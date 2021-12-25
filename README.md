# Learn to build win32 apps with Rust
This repository contains source code to some example projects in win32. I'm learning [rust](https://www.rust-lang.org/) at the moment and this is a catalogue of my journey. Hopefully it will be useful to you dear reader.

## Setting up your development environment

I'd recommend using [chocolatey](https://chocolatey.org/) as your package manager.

To install rust,
1. Run Powershell or Cmd with administrator privileges.
2. Run `choco install rust` to install rust.
3. Then, run `choco install rustup.install` to install rustup. It installs `rustfmt` which is a code formatter for rust.


## Setting up your IDE

**vscode**

If you are using [vscode](https://code.visualstudio.com/), I'd recommend installing the `rust-analyzer` extension.

**vim**

If you are running [vim](https://www.vim.org/), I'd recommend installing the `[rust-lang/rust.vim](https://github.com/rust-lang/rust.vim)` plugin

## Running the examples

The examples are organised in their self-contained folder. All files needed to build and run the examples are in that folder. 

To build an example, navigate to the folder in powershell / cmd and type

    cargo build

To run the example, type

    cargo run

For instance, to run the messagebox example,

    PS C:\Code\rust-win32\examples\messagebox> cargo build
        Compiling messagebox v0.1.0 (C:\Code\rust-win32\examples\messagebox)
         Finished dev [unoptimized + debuginfo] target(s) in 1.04s
    PS C:\Code\rust-win32\examples\messagebox> cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.01s
         Running `target\debug\messagebox.exe`
    PS C:\Code\rust-win32\examples\messagebox> 


# Questions?

If you have any questions, please open an issue.