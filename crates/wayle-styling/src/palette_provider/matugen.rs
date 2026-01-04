use wayle_config::infrastructure::themes::Palette;

use crate::{Error, palette_provider::PaletteProvider};

pub struct MatugenProvider;

impl PaletteProvider for MatugenProvider {
    fn load() -> Result<Palette, Error> {
        todo!("Implement matugen palette loading")
    }
}
