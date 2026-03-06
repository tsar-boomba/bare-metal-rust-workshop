# Bare Metal Rust Workshop

Welome to the workshop! Where dreams become reality.

## Rust Installation

### MacOS/Linux Installation

1. Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Press enter to proceed with default installation
3. Run `. "$HOME/.cargo/env"`
4. Confirm installation with the following:

```bash
rustc --version
cargo --version
```

If the command prints a version number, the installation was successful.

### Windows Installation

Download the official Rust standalone installer:

https://forge.rust-lang.org/infra/other-installation-methods.html

Scroll down to the **Standalone Installers** section and download the stable release for:

**x86_64-pc-windows-msvc**

Run the installer and follow the default setup instructions.

After installation, open **PowerShell** or **Command Prompt** and verify:

```bash
rustc --version
cargo --version
```

## Install elf2uf2-rs

`elf2uf2-rs` is the tool that we will be using to convert our ELF binary into UF2 format and flash it to the pico.

Once Rust is successfully installed, run `cargo install elf2uf2-rs`.
