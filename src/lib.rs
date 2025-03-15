use std::io::Read;

use prost::Message;

mod tiles {
    include!(concat!(env!("OUT_DIR"), "/vector_tile.rs"));
}

pub fn add(left: u64, right: u64) -> u64 {
    let mut file = std::fs::File::open("tile1.mvt").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let tile = tiles::Tile::decode(buf.as_slice()).unwrap();
    println!("{:#?}", tile);

    // test graphics

    // end test graphics

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
