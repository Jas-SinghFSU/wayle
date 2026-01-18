pub(crate) mod matugen;
pub(crate) mod pywal;

use wayle_config::infrastructure::themes::Palette;

use crate::Error;

pub(crate) trait PaletteProvider {
    fn load() -> Result<Palette, Error>;
}
