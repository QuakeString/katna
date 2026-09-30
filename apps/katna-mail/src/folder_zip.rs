// SPDX-License-Identifier: GPL-3.0-or-later

//! Folders attached to a message go as one zip file of the folder, named
//! after it: "Send with Katna Mail" on a folder, or a folder dropped on
//! the message. No GPUI here.

use std::fs::File;
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

/// Zips left in the cache longer than this are deleted (those too large
/// for the message go to Drive from there).
const KEEP: Duration = Duration::from_secs(24 * 60 * 60);

/// Packs folder `dir`, with everything in it, into `<folder name>.zip` in a
/// new folder under `cache`, and returns the zip's path. Links to folders
/// are not followed, so a link back up cannot loop.
pub fn zip_folder(dir: &Path, cache: &Path) -> io::Result<PathBuf> {
    forget_old(cache);
    let name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "folder".to_owned());
    let stamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let out_dir = cache.join(stamp.to_string());
    std::fs::create_dir_all(&out_dir)?;
    let out = out_dir.join(format!("{name}.zip"));
    let mut zip = ZipWriter::new(BufWriter::new(File::create(&out)?));
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .large_file(true);
    add_dir(&mut zip, dir, &name, options)?;
    zip.finish()?.into_inner().map_err(|err| err.into_error())?;
    Ok(out)
}

/// Adds `dir` to `zip` as `prefix/…`.
fn add_dir(
    zip: &mut ZipWriter<BufWriter<File>>,
    dir: &Path,
    prefix: &str,
    options: SimpleFileOptions,
) -> io::Result<()> {
    zip.add_directory(format!("{prefix}/"), options)?;
    let mut entries: Vec<_> = std::fs::read_dir(dir)?.filter_map(Result::ok).collect();
    // The same folder always gives the same zip.
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let name = format!("{prefix}/{}", entry.file_name().to_string_lossy());
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        if kind.is_dir() {
            add_dir(zip, &path, &name, options)?;
        } else if path.is_file() {
            // A file, or a link to one: its content goes in.
            zip.start_file(name, options)?;
            io::copy(&mut File::open(&path)?, zip)?;
        }
    }
    Ok(())
}

/// Deletes the zips made more than a day ago.
fn forget_old(cache: &Path) {
    let Ok(entries) = std::fs::read_dir(cache) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let old = entry
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|modified| modified.elapsed().ok())
            .is_some_and(|age| age > KEEP);
        if old {
            let _ = std::fs::remove_dir_all(entry.path());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn zips_the_folder_under_its_own_name() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("Garden plans");
        std::fs::create_dir_all(dir.join("photos")).unwrap();
        std::fs::write(dir.join("quote.txt"), "12 roses").unwrap();
        std::fs::write(dir.join("photos/shed.jpg"), [1u8, 2, 3]).unwrap();
        let zipped = zip_folder(&dir, &tmp.path().join("cache")).unwrap();
        assert_eq!(zipped.file_name().unwrap(), "Garden plans.zip");

        let mut archive = zip::ZipArchive::new(File::open(&zipped).unwrap()).unwrap();
        let mut names: Vec<_> = archive.file_names().map(str::to_owned).collect();
        names.sort();
        assert_eq!(
            names,
            [
                "Garden plans/",
                "Garden plans/photos/",
                "Garden plans/photos/shed.jpg",
                "Garden plans/quote.txt",
            ]
        );
        let mut text = String::new();
        archive
            .by_name("Garden plans/quote.txt")
            .unwrap()
            .read_to_string(&mut text)
            .unwrap();
        assert_eq!(text, "12 roses");
    }

    #[test]
    fn two_zips_of_one_folder_do_not_collide() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("Receipts");
        std::fs::create_dir_all(&dir).unwrap();
        let cache = tmp.path().join("cache");
        let first = zip_folder(&dir, &cache).unwrap();
        let second = zip_folder(&dir, &cache).unwrap();
        assert_ne!(first, second);
        assert!(first.is_file() && second.is_file());
    }
}
