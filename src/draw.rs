use crate::buffer::MmapBuffer;

use anyhow::{Result, bail};
use font_kit::{
    canvas::{Canvas, Format, RasterizationOptions},
    hinting::HintingOptions,
};

use pathfinder_geometry::{
    transform2d::Transform2F,
    vector::{Vector2F, Vector2I},
};

pub fn draw(buf: &mut MmapBuffer) -> Result<()> {
    // We're going to draw a hiragana 'あ' character filled
    // with a blue-green gradient.
    //
    // Our buffer is in `Argb8888` format: 32 bits per pixel,
    // providing red, green, blue, and alpha. The `Argb` means
    // that `A` is the most significant byte and `b` the least.
    // These appear in little-endian byte order, so `A` is the
    // third byte, and `b` is the first.

    let canvas = rasterize_glyph(512, 400.0, 400.0)?;
    assert_eq!(canvas.format, Format::A8);

    // Fill the buffer with the gradient, and take the alpha from the
    // rasterized glyph.
    for (i, pixel) in buf.bytes.chunks_mut(4).enumerate() {
        let x = i & 511;
        let y = i >> 9;
        let alpha = canvas.pixels[y * canvas.stride + x];

        // Wayland assumes pixels' other components have already been
        // multiplied by their alphas.
        pixel[0] = ((x / 2) * alpha as usize / 255) as u8;
        pixel[1] = ((y / 2) * alpha as usize / 255) as u8;
        pixel[2] = 0;
        pixel[3] = alpha; // mostly opaque
    }

    Ok(())
}

fn rasterize_glyph(canvas_size: i32, glyph_size: f32, baseline: f32) -> Result<Canvas> {
    let font = {
        let mut font_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        font_path.push("DroidSansJapanese.ttf");
        let handle = font_kit::handle::Handle::from_path(font_path, 0);
        handle.load()?
    };
    let Some(glyph_id) = font.glyph_for_char('あ') else {
        bail!("Couldn't find glyph in {} for 'あ'", font.full_name());
    };

    let mut canvas = Canvas::new(Vector2I::splat(canvas_size), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        glyph_size,
        Transform2F::from_translation(Vector2F::new(0.0, baseline)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )?;

    Ok(canvas)
}
