use mca::{RegionIter, RegionReader};
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FTBChunksObject {
    pub chunks: HashMap<String, Vec<XZ>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct XZ {
    pub x: i32,
    pub z: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    File::open("./save.d/ftbchunks/78bd4083-96b7-4d98-a2aa-2e0571822154.snbt")?
        .read_to_end(&mut data)?;

    let obj: FTBChunksObject = fastnbt::from_bytes(data.as_slice()).unwrap();

    println!("{:?}", obj);

    Ok(())
}

fn read_mca() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    File::open("./save.d/region/r.-1.0.mca")?.read_to_end(&mut data)?;
    let region = RegionReader::new(&data)?;
    let chunks = region.iter();
    for (index, chunk) in chunks.enumerate() {
        if let Some(chunk) = chunk.unwrap() {
            let (x, y) = RegionIter::get_chunk_coordinate(index);
            // println!("got chunk {} {}", x, y);

            // Decompress the chunk data
            // This will most commonly be either ZLib or LZ4 compressed
            // let decompressed = chunk.decompress()?;

            // You can now bring your own NBT parser to parse the actual chunk data here
            // I recommend either `simdnbt` or `fastnbt` for this.
        }
    }

    Ok(())
}
