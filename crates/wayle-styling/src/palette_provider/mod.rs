pub(crate) mod matugen;
pub(crate) mod pywal;
pub(crate) mod wallust;

use wayle_config::infrastructure::themes::Palette;

use crate::Error;

pub trait PaletteProvider {
    fn load() -> Result<Palette, Error>;
}
