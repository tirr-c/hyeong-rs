use std::io::{self, Read};

const MASK: [u8; 4] = [0x7f, 0x1f, 0x0f, 0x07];

pub fn read_codepoint<R: Read>(read: &mut R) -> io::Result<u32> {
    let mut buf = [0];
    read.read_exact(&mut buf)?;
    let first_byte = buf[0];
    let char_count = if first_byte & 0x80 == 0 {
        0
    } else if first_byte & 0xe0 == 0xc0 {
        1
    } else if first_byte & 0xf0 == 0xe0 {
        2
    } else if first_byte & 0xf8 == 0xf0 {
        3
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid UTF-8 sequence",
        ));
    };
    if char_count == 0 {
        return Ok(first_byte as u32);
    }

    let mut buf = vec![0; char_count];
    read.read_exact(&mut buf)?;
    if !buf.iter().all(|b| *b & 0xc0 == 0x80) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid UTF-8 sequence",
        ));
    }
    let base = (first_byte & MASK[char_count]) as u32;
    let result = buf
        .iter()
        .fold(base, |c, &b| (c << 6) | ((b & 0x3f) as u32));
    Ok(result)
}

#[cfg(test)]
#[test]
fn codepoint_from_slice() {
    let codepoint = [0x61];
    let mut read = &codepoint[..];
    assert_eq!(read_codepoint(&mut read).unwrap(), 0x61);
    let codepoint = [0xc2, 0xa2];
    let mut read = &codepoint[..];
    assert_eq!(read_codepoint(&mut read).unwrap(), 0xa2);
    let codepoint = [0xe2, 0x82, 0xac];
    let mut read = &codepoint[..];
    assert_eq!(read_codepoint(&mut read).unwrap(), 0x20ac);
    let codepoint = [0xf0, 0x90, 0x8d, 0x88];
    let mut read = &codepoint[..];
    assert_eq!(read_codepoint(&mut read).unwrap(), 0x10348);
}
