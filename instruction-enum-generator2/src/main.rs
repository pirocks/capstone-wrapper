use std::io::Cursor;

use reqwest::blocking;
use serde_xml_rs::from_reader;
use uops_info::Root;

use wrapper_common::instructions::Instructions;

fn main() -> anyhow::Result<()> {
    let request_builder = blocking::Client::builder().timeout(None);
    let request = request_builder.build()?.get("https://uops.info/instructions.xml").send()?;
    let xml_bytes = request.bytes()?;
    let root: Root = from_reader(Cursor::new(xml_bytes.as_ref()))?;
    let instructions = Instructions::new(root)?;
    let bytes = bincode::serialize(&instructions)?;
    dbg!(bytes.len());
    std::fs::write("out.bin", bytes)?;
    Ok(())
}
