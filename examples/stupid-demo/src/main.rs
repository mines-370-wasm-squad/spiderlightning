use anyhow::Result;

use stupid::*;
wit_bindgen_rust::import!("../../wit/stupid.wit");

fn main() -> Result<()> {
    println!("Hello, world!");
    Stupid::open_browser("https://example.com/").map_err(|msg| anyhow::anyhow!(msg))
}
