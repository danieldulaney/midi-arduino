extern crate midi_arduino;

use midi_arduino::Parser;

static SWEET_VICTORY_DATA: &'static [u8] = include_bytes!("data/sweet_victory.mid");

#[test]
fn sweet_victory() {
    let data = SWEET_VICTORY_DATA.to_vec();

    let mut parser = Parser::new(&data);

    for chunk in parser {
        println!("Type: {:?}, Length: {:?}", chunk.kind(), chunk.data_len());
    }

    unimplemented!();
}
