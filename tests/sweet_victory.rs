extern crate midi_arduino;

use midi_arduino::{Division, Format, Header, MessageParser, Parser};

static SWEET_VICTORY_DATA: &'static [u8] = include_bytes!("data/sweet_victory.mid");

#[test]
fn sweet_victory() {
    let data = SWEET_VICTORY_DATA.to_vec();

    let mut chunk_parser = Parser::new(&data);

    let header_chunk = chunk_parser.next().unwrap();
    let header: Header = header_chunk.into();

    println!("{:?}", header);
    assert_eq!(
        header,
        Header {
            format: Format::SimultaneousTrack,
            tracks: 18,
            division: Division::QuarterTicks(480),
        }
    );

    for chunk in chunk_parser {
        println!(
            "Encountered chunk with type {:?} and length {}",
            chunk.kind(),
            chunk.data_len()
        );

        let mut message_parser: MessageParser = chunk.into();

        for message in message_parser {
            println!("{:?}", message);
        }
    }

    unimplemented!();
}
