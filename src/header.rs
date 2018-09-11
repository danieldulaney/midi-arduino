use byteorder::{BigEndian, ByteOrder};
use chunk::{Chunk, ChunkKind};

#[derive(Debug, PartialEq)]
pub struct Header {
    pub format: Format,
    pub tracks: u16,
    pub division: Division,
}

#[derive(Debug, PartialEq)]
pub enum Format {
    SingleTrack,
    SimultaneousTrack,
    IndependentTrack,
}

#[derive(Debug, PartialEq)]
pub enum Division {
    QuarterTicks(u16),
}

impl Header {
    pub fn from_chunk<'d>(chunk: Chunk<'d>) -> Header {
        if chunk.kind() != ChunkKind::Header {
            panic!("Tried to parse {:?} chunk as a header", chunk.kind());
        }

        if chunk.data_len() < 6 {
            panic!(
                "Tried to parse a header with less than 6 bytes ({} bytes)",
                chunk.data_len()
            );
        }

        let format_code = BigEndian::read_u16(chunk.data);
        let tracks = BigEndian::read_u16(&chunk.data[2..]);
        let division_code = BigEndian::read_u16(&chunk.data[4..]);

        let format = Format::from_code(format_code);
        let division = Division::from_code(division_code);

        Header {
            format,
            tracks,
            division,
        }
    }
}

impl Format {
    fn from_code(code: u16) -> Format {
        match code {
            0 => Format::SingleTrack,
            1 => Format::SimultaneousTrack,
            2 => Format::IndependentTrack,
            _ => panic!("Unrecognized format code {}", code),
        }
    }
}

impl Division {
    fn from_code(code: u16) -> Division {
        if code & 0x8000 == 0 {
            Division::QuarterTicks(code & 0x7FFF)
        } else {
            unimplemented!("Time-based divisions not currently supported")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Division::*;
    use super::Format::*;
    use super::*;

    #[test]
    fn parse_header() {
        //                                                        Format  Tracks  Division
        let chunk_0_1_16 =
            Chunk::from_data(b"MThd\x00\x00\x00\x06\x00\x00\x00\x01\x00\x10").unwrap();

        let header_0_1_16 = Header {
            format: SingleTrack,
            tracks: 1,
            division: QuarterTicks(16),
        };

        assert_eq!(header_0_1_16, Header::from_chunk(chunk_0_1_16));
    }

    #[test]
    fn good_format_codes() {
        assert_eq!(SingleTrack, Format::from_code(0));
        assert_eq!(SimultaneousTrack, Format::from_code(1));
        assert_eq!(IndependentTrack, Format::from_code(2));
    }

    #[test]
    #[should_panic]
    fn bad_format_code() {
        Format::from_code(3);
    }

    #[test]
    fn division_codes() {}
}
