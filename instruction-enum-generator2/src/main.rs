use std::io::Cursor;
use itertools::Itertools;

use reqwest::blocking;
use serde_xml_rs::from_reader;
use uops_info::Root;

use wrapper_common::instructions::Instructions;

fn get_bytes_from_network() -> anyhow::Result<Vec<u8>> {
    let request_builder = blocking::Client::builder().timeout(None);
    let request = request_builder.build()?.get("https://uops.info/instructions.xml").send()?;
    Ok(request.bytes()?.iter().cloned().collect_vec())
}


fn main() -> anyhow::Result<()> {
    // let xml_bytes = get_bytes_from_network()?;
    let xml_bytes = include_bytes!("/home/francis/Downloads/instructions.xml").to_vec();
    let root: Root = from_reader(Cursor::new(xml_bytes))?;
    let instructions = Instructions::new(root)?;
    let bytes = bincode::serialize(&instructions)?;
    dbg!(bytes.len());
    std::fs::write("out.bin", &bytes)?;
    let read_bytes = std::fs::read("out.bin").unwrap();
    assert_eq!(read_bytes, bytes);
    let _: Instructions = bincode::deserialize_from(Cursor::new(read_bytes)).unwrap();
    Ok(())
}
