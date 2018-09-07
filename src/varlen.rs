pub fn parse_varlen(data: &[u8]) -> (u64, usize) {
    unimplemented!()
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
