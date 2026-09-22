//! Minimal "hello" plugin — simple-mode plain-text reply.
//!
//! Run it locally:
//!
//! ```bash
//! cargo run --example hello -- Alice
//! ```
//!
//! Or install into WhatsRook after building:
//!
//! ```text
//! .install hello /path/to/target/release/examples/hello
//! .hello World
//! ```

use whatsrook_sdk::{Request, respond};

fn main() {
    let req = Request::load();
    let query = req.query();

    if query.is_empty() {
        respond(format!("Usage: {}hello <name>", req.prefix()));
        return;
    }

    respond(format!("Hello, {}! 👋", query));
}
