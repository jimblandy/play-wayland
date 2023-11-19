use std::path::PathBuf;

use anyhow::{bail, Result};
use font_kit::{
    canvas::{Canvas, Format, RasterizationOptions},
    hinting::HintingOptions,
};
use pathfinder_geometry::{
    transform2d::Transform2F,
    vector::{Vector2F, Vector2I},
};

fn main() -> Result<()> {
    let font = {
        let mut font_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        font_path.push("DroidSansJapanese.ttf");
        let handle = font_kit::handle::Handle::from_path(font_path, 0);
        handle.load()?
    };
    let Some(glyph_id) = font.glyph_for_char('あ') else {
        bail!("Couldn't find glyph in {} for 'あ'", font.full_name());
    };

    let mut canvas = Canvas::new(Vector2I::splat(40), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        32.0,
        Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )?;

    assert_eq!(canvas.format.bytes_per_pixel(), 1);
    let bytes_per_row = canvas.size.x() as usize;

    for row in canvas.pixels.chunks(bytes_per_row) {
        for &pixel in row {
            print!("{}", if pixel < 128 { ' ' } else { '*' });
        }
        println!();
    }

    Ok(())
}
