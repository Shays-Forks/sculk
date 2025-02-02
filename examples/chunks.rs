use fastanvil::Region;
use sculk::chunk::Chunk;
use std::fs::File;

fn main() {
    // Read in the region file
    let file = File::open("r.0.0.mca").unwrap();
    // Parse the region file
    let mut region = Region::from_stream(&file).unwrap();

    // Iterate over every chunk in the region
    for x in 0..32 {
        for z in 0..32 {
            // Get the chunk at the specified chunk coordinates
            let data = region.read_chunk(x, z).unwrap().unwrap();

            // parse the raw chunk data into a Chunk struct
            let chunk = Chunk::from_bytes(&data).unwrap();
        }
    }
}
