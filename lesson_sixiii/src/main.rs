enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize {
    fn file(size: u64) -> Self {
        let filesize = match size {
            0..=999 => FileSize::Bytes(size),
            1000..=999_999 => FileSize::Kilobytes(size / 1000),
            1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
            _ => FileSize::Gigabytes(size / 1_000_000_000),
        };
        filesize
    }

    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{} KB", *kb as f64 / 1000.0),
            FileSize::Megabytes(mb) => format!("{} MB", *mb as f64 / 1000.0),
            FileSize::Gigabytes(gb) => format!("{} GB", *gb as f64 / 1000.0),
        }
    }
}

fn main() {
    let file = FileSize::file(8788896655989);

    println!("{}", file.format_size())
}
