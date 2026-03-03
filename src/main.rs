//! Proof that url::Url::join treats a leading slash as "replace entire path"
//! and a path without leading slash as "append to base path".

use url::Url;

fn main() {
    let base = Url::parse("https://bridge.example.com/api/bridge/").unwrap();

    println!("Base URL: {}\n", base);

    // --- Example 1: path WITHOUT leading slash — appends to base path ---
    let path_no_slash = "sign/bridge_action/sui/eth/1/2/3";
    let joined_relative = base.join(path_no_slash).unwrap();
    println!("1. Join WITHOUT leading slash:");
    println!("   base.join(\"{}\")", path_no_slash);
    println!("   => {}", joined_relative);
    println!("   (base path /api/bridge/ is preserved)\n");

    // --- Example 2: path WITH leading slash — replaces base path ---
    let path_with_slash = "/sign/bridge_action/sui/eth/1/2/3";
    let joined_absolute = base.join(path_with_slash).unwrap();
    println!("2. Join WITH leading slash:");
    println!("   base.join(\"{}\")", path_with_slash);
    println!("   => {}", joined_absolute);
    println!("   (base path /api/bridge/ is DROPPED)\n");

    assert_ne!(joined_relative.as_str(), joined_absolute.as_str());
    println!("Proof: same base URL + different path style => different final URLs.");
}
