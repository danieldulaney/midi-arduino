use std;
use std::iter::Iterator;

/// Takes a slice beginning with a MIDI varlen; returns a u64 containing the
/// decoded number and the number of bytes that encoding took up.
///
/// # Panics
///
/// Panics if passed a zero-length slice, or if the number to decode exceeds a
/// u64.
pub fn parse_varlen(data: &[u8]) -> (u64, usize) {
    if data.len() == 0 {
        panic!("Zero-length data tried to parse as varlen");
    }

    let mut last_byte = std::usize::MAX;

    for byte in 0.. {
        last_byte = byte;

        if data[byte as usize] & 0x80 == 0 {
            break;
        }
    }

    let mut result: u64 = 0;

    for index in 0..=last_byte {
        // Shift each byte over 7 more
        let shift_amount = 7 * (last_byte - index);

        // Mask off the first bit, upcast to u64, then shift by the given amount
        let shifted_byte = ((data[index] & 0x7F) as u64) << shift_amount;

        // Put this step into the result
        result = result | shifted_byte;
    }

    return (result, last_byte + 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_varlen() {
        // Test cases from http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html section 1.1
        assert_eq!((0, 1), parse_varlen(&[0x00]));
        assert_eq!((0x40, 1), parse_varlen(&[0x40]));
        assert_eq!((0x7F, 1), parse_varlen(&[0x7F]));
        assert_eq!((0x80, 2), parse_varlen(&[0x81, 0x00]));
        assert_eq!((0x2000, 2), parse_varlen(&[0xC0, 0x00]));
        assert_eq!((0x3FFF, 2), parse_varlen(&[0xFF, 0x7F]));
        assert_eq!((0x4000, 3), parse_varlen(&[0x81, 0x80, 0x00]));
        assert_eq!((0x100000, 3), parse_varlen(&[0xC0, 0x80, 0x00]));
        assert_eq!((0x1FFFFF, 3), parse_varlen(&[0xFF, 0xFF, 0x7F]));
        assert_eq!((0x200000, 4), parse_varlen(&[0x81, 0x80, 0x80, 0x00]));
        assert_eq!((0x08000000, 4), parse_varlen(&[0xC0, 0x80, 0x80, 0x00]));
        assert_eq!((0x0FFFFFFF, 4), parse_varlen(&[0xFF, 0xFF, 0xFF, 0x7F]));
    }
}
