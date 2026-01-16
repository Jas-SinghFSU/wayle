use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::error::Error;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "tga", "tiff", "webp", "pnm", "farbfeld", "svg", "jxl",
    "avif",
];

/// Order in which wallpapers are cycled through a directory.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CyclingMode {
    /// Cycle through images in alphabetical order.
    #[default]
    Sequential,
    /// Cycle through images in random order.
    Shuffle,
}

impl Display for CyclingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl CyclingMode {
    /// Returns the mode name as a string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sequential => "sequential",
            Self::Shuffle => "shuffle",
        }
    }
}

impl FromStr for CyclingMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sequential" => Ok(Self::Sequential),
            "shuffle" => Ok(Self::Shuffle),
            _ => Err(format!("Invalid cycling mode: {s}")),
        }
    }
}

/// Shared wallpaper cycling configuration.
///
/// Contains the image pool and timing settings. Per-monitor cycle
/// positions are tracked separately in `MonitorState`.
#[derive(Debug, Clone, PartialEq)]
pub struct CyclingConfig {
    /// Directory containing wallpaper images.
    pub directory: PathBuf,
    /// List of image paths in cycle order.
    pub images: Vec<PathBuf>,
    /// Cycling order mode.
    pub mode: CyclingMode,
    /// Time between wallpaper changes.
    pub interval: Duration,
}

impl CyclingConfig {
    /// Creates a new cycling config by scanning a directory for images.
    ///
    /// # Errors
    ///
    /// Returns error if the directory doesn't exist or contains no valid images.
    pub fn new(directory: PathBuf, mode: CyclingMode, interval: Duration) -> Result<Self, Error> {
        if !directory.exists() {
            return Err(Error::DirectoryNotFound(directory));
        }

        let mut images = scan_directory_for_images(&directory)?;

        if images.is_empty() {
            return Err(Error::NoImagesFound(directory));
        }

        match mode {
            CyclingMode::Sequential => images.sort(),
            CyclingMode::Shuffle => {
                let mut rng = rand::rng();
                images.shuffle(&mut rng);
            }
        }

        Ok(Self {
            directory,
            images,
            mode,
            interval,
        })
    }

    /// Returns the image at the given index (wraps around).
    pub fn image_at(&self, index: usize) -> Option<&PathBuf> {
        if self.images.is_empty() {
            return None;
        }
        self.images.get(index % self.images.len())
    }

    /// Returns the number of images in the cycle.
    pub fn image_count(&self) -> usize {
        self.images.len()
    }

    /// Rescans the directory and updates the image list.
    ///
    /// # Errors
    ///
    /// Returns error if the directory can no longer be read.
    pub fn refresh(&mut self) -> Result<(), Error> {
        let mut new_images = scan_directory_for_images(&self.directory)?;

        if new_images.is_empty() {
            self.images.clear();
            return Ok(());
        }

        match self.mode {
            CyclingMode::Sequential => new_images.sort(),
            CyclingMode::Shuffle => {
                let mut rng = rand::rng();
                new_images.shuffle(&mut rng);
            }
        }

        self.images = new_images;
        Ok(())
    }
}

fn scan_directory_for_images(directory: &Path) -> Result<Vec<PathBuf>, Error> {
    let entries = fs::read_dir(directory)?;

    let images: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        })
        .collect();

    Ok(images)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(image_count: usize) -> CyclingConfig {
        let images: Vec<PathBuf> = (0..image_count)
            .map(|i| PathBuf::from(format!("image_{i}.png")))
            .collect();

        CyclingConfig {
            directory: PathBuf::from("/test"),
            images,
            mode: CyclingMode::Sequential,
            interval: Duration::from_secs(10),
        }
    }

    #[test]
    fn image_at_returns_correct_image() {
        let config = make_config(3);

        assert_eq!(config.image_at(0), Some(&PathBuf::from("image_0.png")));
        assert_eq!(config.image_at(1), Some(&PathBuf::from("image_1.png")));
        assert_eq!(config.image_at(2), Some(&PathBuf::from("image_2.png")));
    }

    #[test]
    fn image_at_wraps_around() {
        let config = make_config(3);

        assert_eq!(config.image_at(3), Some(&PathBuf::from("image_0.png")));
        assert_eq!(config.image_at(4), Some(&PathBuf::from("image_1.png")));
        assert_eq!(config.image_at(5), Some(&PathBuf::from("image_2.png")));
    }

    #[test]
    fn image_at_returns_none_for_empty() {
        let config = CyclingConfig {
            directory: PathBuf::from("/test"),
            images: vec![],
            mode: CyclingMode::Sequential,
            interval: Duration::from_secs(10),
        };

        assert_eq!(config.image_at(0), None);
    }

    #[test]
    fn image_count_returns_correct_count() {
        let config = make_config(5);
        assert_eq!(config.image_count(), 5);
    }
}
