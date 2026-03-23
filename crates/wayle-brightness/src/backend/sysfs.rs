use std::{
    fs, io,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use tracing::warn;

use crate::{
    Error,
    types::{BacklightInfo, BacklightType, DeviceName},
};

const BACKLIGHT_DIR: &str = "/sys/class/backlight";
const ATTR_BRIGHTNESS: &str = "brightness";
const ATTR_ACTUAL_BRIGHTNESS: &str = "actual_brightness";
const ATTR_MAX_BRIGHTNESS: &str = "max_brightness";
const ATTR_TYPE: &str = "type";

/// Enumerates all backlight devices from sysfs.
pub(crate) fn enumerate() -> Vec<BacklightInfo> {
    let dir = match fs::read_dir(BACKLIGHT_DIR) {
        Ok(dir) => dir,
        Err(err) => {
            warn!(error = %err, path = BACKLIGHT_DIR, "cannot read backlight directory");
            return Vec::new();
        }
    };

    dir.filter_map(|entry| {
        let entry = entry.ok()?;
        let name = entry.file_name().to_str()?.to_owned();
        read_device(&name).ok()
    })
    .collect()
}

/// Reads a single backlight device's state from sysfs.
pub(crate) fn read_device(name: &str) -> Result<BacklightInfo, Error> {
    let device_name = DeviceName::new(name);
    let base = PathBuf::from(BACKLIGHT_DIR).join(name);

    let raw_type = read_attr(&base.join(ATTR_TYPE))?;
    let backlight_type = BacklightType::from_sysfs(&raw_type).unwrap_or(BacklightType::Raw);

    let brightness = read_u32(&base.join(ATTR_ACTUAL_BRIGHTNESS))?;
    let max_brightness = read_u32(&base.join(ATTR_MAX_BRIGHTNESS))?;

    Ok(BacklightInfo {
        name: device_name,
        backlight_type,
        brightness,
        max_brightness,
    })
}

/// Writes brightness directly to sysfs (fallback when logind unavailable).
pub(crate) fn write_brightness(name: &str, value: u32) -> Result<(), Error> {
    let path = PathBuf::from(BACKLIGHT_DIR)
        .join(name)
        .join(ATTR_BRIGHTNESS);

    let path_str = path.display().to_string();

    fs::write(&path, value.to_string()).map_err(|source| Error::SysfsWrite {
        path: path_str,
        source,
    })
}

/// Path to the `actual_brightness` sysfs attribute.
pub(crate) fn brightness_path(name: &str) -> PathBuf {
    PathBuf::from(BACKLIGHT_DIR)
        .join(name)
        .join(ATTR_ACTUAL_BRIGHTNESS)
}

fn read_attr(path: &Path) -> Result<String, Error> {
    let path_str = path.display().to_string();

    fs::read_to_string(path)
        .map(|content| content.trim().to_owned())
        .map_err(|source| Error::SysfsRead {
            path: path_str,
            source,
        })
}

fn read_u32(path: &Path) -> Result<u32, Error> {
    let content = read_attr(path)?;

    content.trim().parse().map_err(|_| Error::SysfsRead {
        path: path.display().to_string(),
        source: io::Error::new(
            ErrorKind::InvalidData,
            format!("cannot parse '{}' as u32", content.trim()),
        ),
    })
}
