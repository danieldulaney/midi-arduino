extern crate byteorder;

use self::byteorder::{BigEndian, ByteOrder};

use std::iter::Iterator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChunkKind {
    Header,
    Track,
    Unknown,
}

impl ChunkKind {
    fn from_data(data: &[u8]) -> ChunkKind {
        match &data[..4] {
            b"MThd" => ChunkKind::Header,
            b"MTrk" => ChunkKind::Track,
            _ => ChunkKind::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Chunk<'d> {
    kind: ChunkKind,
    data: &'d [u8],
}

impl<'d> Chunk<'d> {
    fn from_data(data: &'d [u8]) -> Result<Chunk<'d>, String> {
        println!("Getting chunk from data length {}", data.len());

        if data.len() < 8 {
            return Err("Not enough data to read a chunk header".to_owned());
        }

        let kind = ChunkKind::from_data(data);
        let length: usize = BigEndian::read_u32(&data[4..]) as usize;

        println!("Chunk has kind {:?} and length {}", kind, length);

        if data.len() - 8 < length {
            return Err("Chunk length field exceeds remaining data".to_owned());
        }

        Ok(Chunk {
            kind,
            data: &data[8..8 + (length)],
        })
    }

    pub fn kind(&self) -> ChunkKind {
        self.kind
    }

    pub fn data(&self) -> &'d [u8] {
        self.data
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn total_len(&self) -> usize {
        self.data.len() + 8
    }
}

#[derive(Debug)]
pub struct Parser<'d> {
    data: &'d [u8],
    location: usize,
}

impl<'d> Parser<'d> {
    pub fn new(data: &'d [u8]) -> Parser {
        Parser { data, location: 0 }
    }
}

impl<'d> Iterator for Parser<'d> {
    type Item = Chunk<'d>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.location >= self.data.len() {
            return None;
        }

        if let Ok(chunk) = Chunk::from_data(&self.data[self.location..]) {
            self.location += chunk.total_len();
            Some(chunk)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_kind_from_data() {
        assert_eq!(ChunkKind::Unknown, ChunkKind::from_data(b"bad_chunk"));
        assert_eq!(ChunkKind::Header, ChunkKind::from_data(b"MThd"));
        assert_eq!(ChunkKind::Header, ChunkKind::from_data(b"MThdandmoredata"));
        assert_eq!(ChunkKind::Track, ChunkKind::from_data(b"MTrk"));
        assert_eq!(ChunkKind::Track, ChunkKind::from_data(b"MTrkandmoredata"));
    }

    #[test]
    #[should_panic]
    fn chunk_kind_too_short() {
        ChunkKind::from_data(b"123");
    }

    #[test]
    #[should_panic]
    fn chunk_kind_zero_length() {
        ChunkKind::from_data(b"");
    }
}
