use chunk::Chunk;
use std::convert::From;
use std::iter::Iterator;
use std::mem;
use varlen::parse_varlen;

#[derive(Debug, Clone)]
pub enum Event<'d> {
    Meta(Meta<'d>),
}

#[derive(Debug, Clone)]
pub enum Meta<'d> {
    ProgramName(&'d [u8]),
}

#[derive(Debug, Clone)]
pub struct Message<'d> {
    pub delta: u64,
    pub event: Event<'d>,
}

#[derive(Debug)]
pub struct MessageParser<'d> {
    data: &'d [u8],
    location: usize,
    previous: Option<Message<'d>>,
}

impl<'d> From<Chunk<'d>> for MessageParser<'d> {
    fn from(chunk: Chunk<'d>) -> MessageParser<'d> {
        MessageParser {
            data: chunk.data,
            location: 0,
            previous: None,
        }
    }
}

impl<'d> Iterator for MessageParser<'d> {
    type Item = Message<'d>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            None
        } else {
            let mut temp: Option<Message<'d>> = None;
            mem::swap(&mut self.previous, &mut temp);

            let (next_message, size) = Message::parse(&self.data[self.location..], temp);

            self.location += size;
            self.previous = Some(next_message.clone());

            Some(next_message)
        }
    }
}

impl<'d> Message<'d> {
    fn parse(data: &'d [u8], previous: Option<Message<'d>>) -> (Message<'d>, usize) {
        let (delta, delta_size) = parse_varlen(data);

        println!("Got message with delta {}", delta);

        let (event, event_size) = Event::parse(&data[delta_size..], previous);

        (Message { delta, event }, delta_size + event_size)
    }
}

impl<'d> Event<'d> {
    fn parse(data: &'d [u8], previous: Option<Message<'d>>) -> (Event<'d>, usize) {
        match data[0] {
            0xFF => Meta::parse(data).into(),
            _ => unimplemented!("Event {:02x} {:02x} {:02x} {:02x}", data[0], data[1], data[2], data[3]),
        }
    }
}

impl<'d> From<Meta<'d>> for Event<'d> {
    fn from(m: Meta<'d>) -> Event<'d> {
        Event::Meta(m),
    }
}

impl<'d> Meta<'d> {
    fn parse(data: &'d[u8]) -> (Meta<'d>, usize) {
        unimplemented!();
    }
}
