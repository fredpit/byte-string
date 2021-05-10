//! A statically allocated StringBuffer
//!
//! Perfect for embedded projects.
#![no_std]

use core::fmt::Write;
use core::convert::Into;
use core::str::from_utf8_unchecked;

pub struct ByteString<const N: usize> {
    buf:    [u8; N],
    pos:    usize,
}

impl<const N: usize> ByteString<N> {
    pub fn new() -> Self {
        ByteString {
            buf:    [0u8; N],
            pos:    0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pos == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        self.pos = 0;
    }

    #[inline]
    pub fn slice(&self) -> &[u8] {
        &self.buf[0..self.pos]
    }

    pub fn char_at(&self, pos: usize) -> Option<char> {
        if pos < self.pos {
            Some(self.buf[pos] as char)
        } else {
            None
        }
    }

    pub fn append(&mut self, b: u8) {
        if self.pos < self.buf.len() {
            self.buf[self.pos] = b;
            self.pos += 1;
        }
    }
}

impl<const N: usize>Write for ByteString<N> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for b in s.bytes() { self.append(b); }
        Ok(())
    }
}

impl<'a, const N: usize>Into<&'a str> for &'a ByteString<N> {
    fn into(self) -> &'a str {
        unsafe { from_utf8_unchecked(self.slice()) }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;
    #[test]
    fn it_works() {
        let mut bs = crate::ByteString::<20>::new();
        bs.append('A' as u8);
        assert_eq!(bs.len(), 1);
        let _ = write!(bs, "BCD");
        assert_eq!(bs.len(), 4);
        let st : &str = (&bs).into();
        assert_eq!(st, "ABCD");
        let slice = bs.slice();
        assert_eq!(slice[0], 65u8);
        assert_eq!(slice[1], 66u8);
        assert_eq!(slice[2], 67u8);
        assert_eq!(slice[3], 68u8);
        let _ = write!(bs, "12345678901234567890");
        assert_eq!(bs.len(), 20);
        bs.clear();
        assert_eq!(bs.len(), 0);
        assert_eq!(bs.is_empty(), true);
        bs.append('B' as u8);
        assert_eq!(bs.is_empty(), false);
    }
}
