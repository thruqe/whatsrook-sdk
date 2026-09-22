//! Media reply plugin — demonstrates image, audio, video, document, sticker,
//! and GIF actions, all driven from the same HTTP-fetching pattern.
//!
//! This example picks a random dog photo from the public `dog.ceo` API and sends
//! it as an image, then reacts to confirm delivery.
//!
//! Install into WhatsRook:
//!
//! ```text
//! .install dog /path/to/target/release/examples/media_reply
//! .dog
//! ```

use serde::Deserialize;

use whatsrook_sdk::{Request, create_http_client, respond_err, send_image, send_react};

#[derive(Deserialize)]
struct DogResponse {
    message: String,
}

fn main() {
    // Request context is available even if we don't need args here.
    let _req = Request::load();

    let client = create_http_client(10);

    let resp: DogResponse = client
        .get("https://dog.ceo/api/breeds/image/random")
        .send()
        .unwrap_or_else(|e| respond_err(format!("Network error: {e}")))
        .json()
        .unwrap_or_else(|e| respond_err(format!("Parse error: {e}")));

    // React first so the user knows we're responding.
    send_react("🐶");

    // Send the fetched image URL with a caption.
    send_image(&resp.message, Some("Here's your random dog! 🐕"));
}
