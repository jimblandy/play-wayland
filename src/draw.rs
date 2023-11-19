use crate::buffer::MmapBuffer;

pub fn draw(buf: &mut MmapBuffer) {
    for (i, pixel) in buf.bytes.chunks_mut(4).enumerate() {
        // We're going to draw a partially transparent circle filled
        // with a blue-green gradient.
        //
        // Our buffer is in `Argb8888` format: 32 bits per pixel,
        // providing red, green, blue, and alpha. The `Argb` means
        // that `A` is the most significant byte and `b` the least.
        // These appear in little-endian byte order, so `A` is the
        // third byte, and `b` is the first.
        let x = i & 511;
        let y = i >> 9;
        let in_circle = {
            let cx = x as i32 - 256;
            let cy = y as i32 - 256;
            cx * cx + cy * cy < 200 * 200
        };
        if in_circle {
            pixel[0] = (x / 2) as u8;
            pixel[1] = (y / 2) as u8;
            pixel[2] = 0;
            pixel[3] = 192; // mostly opaque
        } else {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 0; // transparent
            
        }
    }
}
