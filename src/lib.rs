//! A statically allocated StringBuffer
//!
//! Perfect for embedded projects.
//! 
//! # Basic usage :
//! '''
//! // A string buffer of 20 bytes.
//! let mut string_buf = ByteString::<20>::new();
//! 
//! // It's writable.
//! let _ = write!(&mut string_buf, "{} x {} = {}", 2, 3, 2*3);
//! 
//! // It can be reuse.
//! string_buf.clear();
//! string_buf.from_str("Hello World !!");
//! 
//! // It can be converted to ['str']
//! let my_str = string_buf.str();
//! let the_str : &str = (&string_buf).into();
//! '''
//! 
#![no_std]

use core::fmt::Write;
use core::convert::Into;
use core::str::from_utf8_unchecked;

pub struct ByteString<const N: usize> {
    buf:    [u8; N],
    pos:    usize,
}

/// Methods for ByteString
impl<const N: usize> ByteString<N> {
    /// Create a string buffer
    /// 
    /// # Example:
    /// '''
    /// let mut sb = ByteString::<100>::new();
    /// '''
    pub fn new() -> Self {
        ByteString {
            buf:    [0u8; N],
            pos:    0,
        }
    }

    /// Get the length of the string
    #[inline]
    pub fn len(&self) -> usize {
        self.pos
    }

    /// Get the size of the buffer
    #[inline]
    pub fn size(&self) -> usize {
        self.buf.len()
    }

    /// Test if the string is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pos == 0
    }

    /// Clear the string
    #[inline]
    pub fn clear(&mut self) {
        self.pos = 0;
    }

    /// Get the slice (&[u8]) containing the string
    #[inline]
    pub fn slice(&self) -> &[u8] {
        &self.buf[0..self.pos]
    }

    /// Get the char at position in the string
    pub fn char_at(&self, pos: usize) -> Option<char> {
        if pos < self.pos {
            Some(self.buf[pos] as char)
        } else {
            None
        }
    }

    /// Append a byte to the string
    pub fn append(&mut self, b: u8) {
        if self.pos < self.buf.len() {
            self.buf[self.pos] = b;
            self.pos += 1;
        }
    }

    /// Append a string to the string buffer
    pub fn append_str(&mut self, s: &str) {
        for b in s.bytes() { self.append(b); }
    }
 
    /// convert string to string buffer
    pub fn from_str(&mut self, s: &str) {
        self.clear();
        self.append_str(s);
    }

    /// Test if string contains a byte
    pub fn has_byte(&self, b: u8) -> bool {
        for bb in &self.buf[0..self.pos] {
            if *bb == b { return true }
        }
        false
    }

    /// Delete last byte of the string
    pub fn del_last(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    /// Remove space at end of the string
    pub fn trim_end(&mut self) {
        while self.pos > 0 {
            if self.buf[self.pos-1] != 0x20u8 {
                break
            }
            self.pos -= 1;
        }
    }

    /// Convert string buffer to string
    #[inline]
    pub fn str(&self) -> &str {
        self.into()
    }
}


impl<const N: usize> Write for ByteString<N> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for b in s.bytes() { self.append(b); }
        Ok(())
    }
}

impl<'a, const N: usize> Into<&'a str> for &'a ByteString<N> {
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
        assert_eq!(bs.size(), 20);
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
        bs.del_last();
        assert_eq!(bs.len(), 3);
        let st : &str = (&bs).into();
        assert_eq!(st, "ABC");
        let _ = write!(bs, "   ");
        bs.trim_end();
        assert_eq!(bs.len(), 3);
        let _ = write!(bs, "12345678901234567890");
        assert_eq!(bs.len(), 20);
        bs.clear();
        assert_eq!(bs.len(), 0);
        assert_eq!(bs.is_empty(), true);
        bs.append('B' as u8);
        assert_eq!(bs.is_empty(), false);
        bs.clear();
        bs.append_str("BCD");
        assert_eq!(bs.len(), 3);
        bs.from_str("Hello");
        assert_eq!(bs.len(), 5);
        assert_eq!(bs.str(), "Hello");
    }
}
