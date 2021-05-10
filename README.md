# byte-string

A statically allocated StringBuffer

Perfect for embedded projects.

## Basic usage :
'''
// A string buffer of 20 bytes.
let mut string_buf = ByteString::<20>::new();

// It's writable.
let _ = write!(&mut string_buf, "{} x {} = {}", 2, 3, 2*3);

// It can be reuse.
string_buf.clear();
string_buf.from_str("Hello World !!");

// It can be converted to ['str']
let my_str = string_buf.str();
let the_str : &str = (&string_buf).into();
'''

