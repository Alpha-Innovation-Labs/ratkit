use std::io::{self, Write};

use crate::vt100::{BufferView, ScreenDiffer};

pub fn write_screen_diff<V: BufferView, W: Write>(
    differ: &mut ScreenDiffer,
    view: &V,
    out: &mut W,
) -> io::Result<()> {
    let mut buffer = String::new();
    differ
        .diff(&mut buffer, view)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    out.write_all(buffer.as_bytes())
}
