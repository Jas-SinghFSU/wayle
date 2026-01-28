use bytesize::ByteSize;
use wayle_sysinfo::types::DiskData;

pub(super) fn format_label(format: &str, disk: &DiskData) -> String {
    format
        .replace("{percent}", &format!("{:02.0}", disk.usage_percent))
        .replace("{used_tib}", &tib(disk.used_bytes))
        .replace("{used_gib}", &gib(disk.used_bytes))
        .replace("{used_mib}", &mib(disk.used_bytes))
        .replace("{used_auto}", &auto(disk.used_bytes))
        .replace("{total_tib}", &tib(disk.total_bytes))
        .replace("{total_gib}", &gib(disk.total_bytes))
        .replace("{total_mib}", &mib(disk.total_bytes))
        .replace("{total_auto}", &auto(disk.total_bytes))
        .replace("{free_tib}", &tib(disk.available_bytes))
        .replace("{free_gib}", &gib(disk.available_bytes))
        .replace("{free_mib}", &mib(disk.available_bytes))
        .replace("{free_auto}", &auto(disk.available_bytes))
        .replace("{filesystem}", &disk.filesystem)
}

fn tib(bytes: u64) -> String {
    format!("{:.2}", ByteSize::b(bytes).as_tib())
}

fn gib(bytes: u64) -> String {
    format!("{:.1}", ByteSize::b(bytes).as_gib())
}

fn mib(bytes: u64) -> String {
    format!("{:.0}", ByteSize::b(bytes).as_mib())
}

fn auto(bytes: u64) -> String {
    ByteSize::b(bytes).to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    const TIB: u64 = 1024 * 1024 * 1024 * 1024;
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;

    fn disk_data(used: u64, total: u64, available: u64, usage_percent: f32, fs: &str) -> DiskData {
        DiskData {
            mount_point: PathBuf::from("/"),
            filesystem: fs.to_string(),
            total_bytes: total,
            used_bytes: used,
            available_bytes: available,
            usage_percent,
        }
    }

    #[test]
    fn format_label_replaces_percent_placeholder() {
        let disk = disk_data(500 * GIB, 1000 * GIB, 500 * GIB, 50.0, "ext4");
        let result = format_label("{percent}%", &disk);
        assert_eq!(result, "50%");
    }

    #[test]
    fn format_label_percent_pads_single_digits() {
        let disk = disk_data(50 * GIB, 1000 * GIB, 950 * GIB, 5.0, "ext4");
        let result = format_label("{percent}", &disk);
        assert_eq!(result, "05");
    }

    #[test]
    fn format_label_replaces_used_gib_placeholder() {
        let disk = disk_data(256 * GIB, 512 * GIB, 256 * GIB, 50.0, "ext4");
        let result = format_label("{used_gib}G", &disk);
        assert_eq!(result, "256.0G");
    }

    #[test]
    fn format_label_replaces_used_mib_placeholder() {
        let disk = disk_data(1536 * MIB, 4096 * MIB, 2560 * MIB, 37.5, "ext4");
        let result = format_label("{used_mib}M", &disk);
        assert_eq!(result, "1536M");
    }

    #[test]
    fn format_label_replaces_used_auto_placeholder_gib() {
        let disk = disk_data(100 * GIB, 500 * GIB, 400 * GIB, 20.0, "ext4");
        let result = format_label("{used_auto}", &disk);
        assert_eq!(result, "100.0 GiB");
    }

    #[test]
    fn format_label_replaces_used_auto_placeholder_mib() {
        let disk = disk_data(512 * MIB, 2 * GIB, GIB + 512 * MIB, 25.0, "ext4");
        let result = format_label("{used_auto}", &disk);
        assert_eq!(result, "512.0 MiB");
    }

    #[test]
    fn format_label_replaces_total_gib_placeholder() {
        let disk = disk_data(256 * GIB, 512 * GIB, 256 * GIB, 50.0, "ext4");
        let result = format_label("{total_gib}G", &disk);
        assert_eq!(result, "512.0G");
    }

    #[test]
    fn format_label_replaces_total_mib_placeholder() {
        let disk = disk_data(1024 * MIB, 4096 * MIB, 3072 * MIB, 25.0, "ext4");
        let result = format_label("{total_mib}M", &disk);
        assert_eq!(result, "4096M");
    }

    #[test]
    fn format_label_replaces_free_gib_placeholder() {
        let disk = disk_data(300 * GIB, 500 * GIB, 200 * GIB, 60.0, "ext4");
        let result = format_label("{free_gib}G free", &disk);
        assert_eq!(result, "200.0G free");
    }

    #[test]
    fn format_label_replaces_free_mib_placeholder() {
        let disk = disk_data(2048 * MIB, 4096 * MIB, 2048 * MIB, 50.0, "ext4");
        let result = format_label("{free_mib}M", &disk);
        assert_eq!(result, "2048M");
    }

    #[test]
    fn format_label_replaces_filesystem_placeholder() {
        let disk = disk_data(100 * GIB, 500 * GIB, 400 * GIB, 20.0, "btrfs");
        let result = format_label("[{filesystem}]", &disk);
        assert_eq!(result, "[btrfs]");
    }

    #[test]
    fn format_label_with_multiple_placeholders() {
        let disk = disk_data(250 * GIB, 500 * GIB, 250 * GIB, 50.0, "ext4");
        let result = format_label("{used_gib}/{total_gib}G ({percent}%)", &disk);
        assert_eq!(result, "250.0/500.0G (50%)");
    }

    #[test]
    fn format_label_with_zero_bytes() {
        let disk = disk_data(0, 500 * GIB, 500 * GIB, 0.0, "ext4");
        let result = format_label("{used_gib}", &disk);
        assert_eq!(result, "0.0");
    }

    #[test]
    fn format_label_with_no_placeholders_returns_unchanged() {
        let disk = disk_data(100 * GIB, 500 * GIB, 400 * GIB, 20.0, "ext4");
        let result = format_label("Disk", &disk);
        assert_eq!(result, "Disk");
    }

    #[test]
    fn format_label_replaces_tib_placeholders() {
        let disk = disk_data(2 * TIB, 4 * TIB, 2 * TIB, 50.0, "ext4");
        let result = format_label("{used_tib}/{total_tib} TiB", &disk);
        assert_eq!(result, "2.00/4.00 TiB");
    }
}
