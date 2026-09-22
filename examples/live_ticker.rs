//! Live ticker plugin — demonstrates in-place message editing.
//!
//! Sends an initial reply, then edits it in-place every 1.5 seconds for 5 ticks,
//! simulating a live data feed.
//!
//! Install into WhatsRook:
//!
//! ```text
//! .install ticker /path/to/target/release/examples/live_ticker
//! .ticker
//! ```

use std::thread;
use std::time::Duration;

use whatsrook_sdk::{
    respond, send_done, send_edit_live, send_loader, send_poll, send_react, send_reply_live,
    Request,
};

fn main() {
    let req = Request::load();

    // Only group admins may use this command.
    if req.is_group() && !req.is_admin() {
        send_react("❌");
        respond("This feature is for group admins only.");
        return;
    }

    // Echo quoted context if the user replied to a message.
    if let Some(quoted) = req.quoted_text() {
        eprintln!("[ticker] user replied to: {}", quoted);
    }

    // Show loader while setting up.
    send_loader(Some("Initializing live tracker..."));

    // React to acknowledge the command.
    send_react("🚀");

    // Ask which asset to track via an interactive poll.
    send_poll("Which asset to track?", &["BTC", "ETH", "Gold"]);

    // Send the initial live message and capture its ID for in-place edits.
    let Some(msg_id) = send_reply_live("⏳ Initializing live tracker...") else {
        respond("Failed to start live session.");
        return;
    };

    // Simulate 5 ticks of a live data feed.
    for tick in 1..=5 {
        thread::sleep(Duration::from_millis(1500));
        send_edit_live(&msg_id, &format!("📈 Tracker tick #{tick}..."));
    }

    // Final edit to show completion.
    send_edit_live(&msg_id, "✅ Live session complete.");
    send_done();
}
