use anyhow::{Context, Result};
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView, WasiCtxView};

// Generate bindings from the WIT file
// Wasmtime will automatically generate async bindings for functions marked async in WIT
wasmtime::component::bindgen!({
    path: "../plugin/wit",
    world: "plugin",
});

/// Host implementation of the functions that the component imports
struct HostImpl {
    wasi: WasiCtx,
    table: ResourceTable,
}

impl WasiView for HostImpl {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

impl example::plugin::host_functions::Host for HostImpl {
    /// Synchronous log function
    fn log(&mut self, message: String) {
        println!("[Component] {}", message);
    }

    /// Fetch function - internally async but exposed as blocking to component
    fn fetch(&mut self, url: String) -> String {
        println!("[Host] fetch() called");
        println!("[Host] Fetching URL: {}", url);

        // Use futures::executor::block_on instead of tokio's block_on
        // to avoid "cannot start runtime from within runtime" error
        match futures::executor::block_on(async {
            reqwest::get(&url).await?.text().await
        }) {
            Ok(text) => {
                println!("[Host] Successfully fetched {} bytes", text.len());
                text
            }
            Err(e) => {
                eprintln!("[Host] Error fetching: {}", e);
                String::new() // Return empty string on error
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Wasmtime WASIp2 Async Component Model Runner ===\n");
    println!("🚀 Using bleeding-edge features:");
    println!("   - wasm32-wasip2 target (tier 2)");
    println!("   - Wasmtime from GitHub master");
    println!("   - component-model-async");
    println!("   - Native async in WIT!\n");

    // Create the engine with component model
    let mut config = Config::new();
    config.wasm_component_model(true);
    // Note: async_support is for async component exports, not host functions
    // Our host function (fetch) uses tokio internally via block_on
    let engine = Engine::new(&config)?;

    // Create a store with WASI context
    let table = ResourceTable::new();
    let wasi = WasiCtxBuilder::new().build();
    let host = HostImpl { wasi, table };
    let mut store = Store::new(&engine, host);

    // Create a linker and add WASI + host function implementations
    let mut linker = Linker::<HostImpl>::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    example::plugin::host_functions::add_to_linker::<HostImpl, HasSelf<HostImpl>>(&mut linker, |h| h)?;

    // Load the component
    let component_path = "target/wasm32-wasip2/release/plugin.wasm";
    println!("Loading component from: {}\n", component_path);

    let component = Component::from_file(&engine, component_path)
        .context("Failed to load component - make sure to build with 'cargo build --target wasm32-wasip2 --release -p plugin'")?;

    // Instantiate the component
    println!("Instantiating component...");
    let instance = Plugin::instantiate(&mut store, &component, &linker)
        .context("Failed to instantiate component")?;

    println!("Component loaded successfully!\n");

    // Call the component's run() function
    println!("Calling component's run() function...\n");
    instance.call_run(&mut store)?;

    println!("\nComponent execution completed!");

    // Demonstrate calling the add function
    println!("\nTesting add function: add(5, 7)");
    let result = instance.call_add(&mut store, 5, 7)?;
    println!("Result: {}", result);

    println!("\n🎉 Success! Welcome to the future of WebAssembly!");

    Ok(())
}
