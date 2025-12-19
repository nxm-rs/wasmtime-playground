# Wasmtime Plugin Architecture Demo

This project demonstrates a plugin-based architecture using WebAssembly (WASM) and Wasmtime, where plugins can call host functions to perform operations like HTTP requests.

## Architecture

- **Runner** (`runner/`): The host application that runs WASM plugins using Wasmtime
- **Plugin** (`plugin/`): A WASM module (compiled from Rust) that uses host-provided functions

## Features

### Host Functions (provided by runner)

The runner exposes two host functions to WASM plugins:

1. **`log(ptr: *const u8, len: u32)`** - Logging function for plugins to output messages
2. **`fetch(url_ptr: *const u8, url_len: u32, response_ptr: *mut u8) -> u32`** - HTTP fetch proxy that allows plugins to make HTTP requests

### Plugin Capabilities

The plugin demonstrates:
- Calling host functions from WASM
- Making HTTP requests via the host's fetch proxy
- Logging messages to the host
- Memory sharing between host and WASM module

## Building and Running

### Prerequisites

```bash
# Enter the nix development shell
nix develop
```

This provides:
- Rust toolchain with `wasm32-unknown-unknown` target
- wasmtime (latest version 38.x)
- wasm-tools and wabt for inspection

### Build

```bash
# Build the plugin (WASM module)
cargo build --release -p plugin

# Build the runner (host application)
cargo build --release -p runner --target x86_64-unknown-linux-gnu
```

### Run

```bash
./target/x86_64-unknown-linux-gnu/release/runner
```

## Project Structure

```
wasmtime-playground/
├── Cargo.toml              # Workspace configuration
├── runner/                 # Host application
│   ├── Cargo.toml
│   └── src/
│       └── main.rs        # Wasmtime runtime with host functions
├── plugin/                 # WASM plugin
│   ├── Cargo.toml
│   ├── .cargo/
│   │   └── config.toml    # WASM build configuration
│   └── src/
│       └── lib.rs         # Plugin code with extern imports
└── README.md
```

## Key Implementation Details

### Plugin Side (WASM)

The plugin declares host functions using `extern "C"`:

```rust
#![no_std]

#[link(wasm_import_module = "env")]
extern "C" {
    #[link_name = "log"]
    fn host_log(ptr: *const u8, len: u32);
    fn fetch(url_ptr: *const u8, url_len: u32, response_ptr: *mut u8) -> u32;
}

#[no_mangle]
pub extern "C" fn run() {
    // Plugin logic here
}
```

### Runner Side (Host)

The runner provides implementations using Wasmtime's `Linker`:

```rust
let mut linker = Linker::new(&engine);

linker.func_wrap(
    "env",
    "log",
    |caller: Caller<'_, HostState>, ptr: u32, len: u32| -> () {
        // Read string from WASM memory and log it
    },
)?;

linker.func_wrap(
    "env",
    "fetch",
    |caller: Caller<'_, HostState>, url_ptr: u32, url_len: u32, response_ptr: u32| -> u32 {
        // Make HTTP request and write response to WASM memory
    },
)?;
```

## Version Information

- **Wasmtime**: 38.x (latest as of Nov 2025)
- **Rust Edition**: 2024
- **Target**: wasm32-unknown-unknown (for plugins)

## Example Output

```
=== Wasmtime Plugin Runner ===

Loading WASM module from: target/wasm32-unknown-unknown/release/plugin.wasm

WASM module loaded successfully!
Available exports:
  - memory
  - add
  - run

Calling plugin's run() function...

[Plugin Log] =================================
[Plugin Log] Hello from the WASM plugin!
[Plugin Log] =================================
[Plugin Log] Demonstrating host function calls...
[Plugin Log] Test 1: Fetching from httpbin.org/get
[Host] fetch() called
[Host] Fetching URL: https://httpbin.org/get
[Host] Wrote 221 bytes to WASM memory
[Plugin Log] ✓ Fetch successful!
[Plugin Log] Response preview:
[Plugin Log] {
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org"
  },
  "origin": "...",
  "url": "https://httpbin.org/get"
}
[Plugin Log] Plugin execution complete!

Plugin execution completed!
```

## Next Steps / Ideas

- Add more host functions (filesystem access, database queries, etc.)
- Implement plugin sandboxing and resource limits
- Add WASI support for standard library functions
- Create a plugin SDK/API for easier plugin development
- Implement plugin discovery and dynamic loading
- Add inter-plugin communication capabilities
