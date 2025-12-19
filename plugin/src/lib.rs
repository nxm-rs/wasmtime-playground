// Generate bindings directly with wit-bindgen macro, using bleeding-edge version from git
// Configure runtime_path to point to wit_bindgen::rt module
wit_bindgen::generate!({
    path: "../plugin/wit",
    world: "plugin",
    runtime_path: "wit_bindgen::rt",
});

use example::plugin::host_functions;

struct Component;

impl Guest for Component {
    /// Main entry point - called by the host
    fn run() {
        host_functions::log("=================================");
        host_functions::log("Hello from the WASM Component!");
        host_functions::log("=================================");
        host_functions::log("");
        host_functions::log("Demonstrating host function calls...");
        host_functions::log("");

        // Test 1: Fetch from httpbin.org/get
        fetch_and_log("https://httpbin.org/get");
        host_functions::log("");

        // Test 2: Fetch user agent info
        fetch_and_log("https://httpbin.org/user-agent");
        host_functions::log("");

        // Test 3: Fetch IP info
        fetch_and_log("https://httpbin.org/ip");
        host_functions::log("");

        host_functions::log("=================================");
        host_functions::log("Component execution complete!");
        host_functions::log("=================================");
    }

    /// Simple math function to demonstrate exports
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

/// Helper function to fetch a URL and log the result
fn fetch_and_log(url: &str) {
    host_functions::log(&format!("Fetching: {}", url));
    host_functions::log("---------------------------------");

    // fetch is synchronous from the component's perspective
    // The host handles the async execution transparently
    let response = host_functions::fetch(url);

    if !response.is_empty() {
        host_functions::log("✓ Fetch successful!");

        // Log a preview of the response
        let preview = if response.len() > 400 {
            &response[..400]
        } else {
            &response
        };

        host_functions::log("Response:");
        host_functions::log(preview);

        if response.len() > 400 {
            host_functions::log("... (truncated)");
        }
    } else {
        host_functions::log("✗ Fetch failed or returned empty response");
    }
}

export!(Component);
