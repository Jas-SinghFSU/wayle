use std::{fs, path::Path};

use crate::infrastructure::error::{Error, IoOperation};

pub(super) fn create_default_config_file(path: &Path) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| Error::Io {
            operation: IoOperation::CreateDir,
            path: parent.to_path_buf(),
            source,
        })?;
    }

    fs::write(path, "# Wayle configuration file\n").map_err(|source| Error::Io {
        operation: IoOperation::WriteFile,
        path: path.to_path_buf(),
        source,
    })?;

    Ok(())
}
