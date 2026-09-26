// SPDX-License-Identifier: GPL-3.0-or-later

//! XDG base directories and the files Katna keeps in them
//! (`docs/ARCHITECTURE.md` §5.1).

use std::ffi::OsString;
use std::fs::DirBuilder;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// Name of the Katna subdirectory in each XDG base directory.
const APP_DIR: &str = "katna";

/// Where Katna keeps its configuration and data.
///
/// Build it with [`Paths::from_env`] in programs and [`Paths::with_root`] in
/// tests. Nothing is created until [`Paths::create_dirs`] is called.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paths {
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
}

impl Paths {
    /// Resolves the directories from `$XDG_CONFIG_HOME`, `$XDG_DATA_HOME`,
    /// `$XDG_CACHE_HOME` and `$HOME`.
    pub fn from_env() -> Result<Self> {
        Self::from_lookup(|name| std::env::var_os(name))
    }

    /// Resolves the directories with `lookup` in place of the process
    /// environment.
    ///
    /// As the XDG Base Directory spec requires, relative or empty XDG values
    /// are ignored and the `$HOME` default is used instead.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<OsString>) -> Result<Self> {
        let absolute = |name: &str| {
            lookup(name)
                .map(PathBuf::from)
                .filter(|path| path.is_absolute())
        };
        let home = absolute("HOME");
        let base = |var: &str, default: &str| {
            absolute(var)
                .or_else(|| home.as_ref().map(|home| home.join(default)))
                .map(|dir| dir.join(APP_DIR))
                .ok_or(Error::NoHomeDir)
        };
        Ok(Self {
            config_dir: base("XDG_CONFIG_HOME", ".config")?,
            data_dir: base("XDG_DATA_HOME", ".local/share")?,
            cache_dir: base("XDG_CACHE_HOME", ".cache")?,
        })
    }

    /// Puts every directory under `root` (`root/config`, `root/data`,
    /// `root/cache`). For tests and portable setups.
    pub fn with_root(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref();
        Self {
            config_dir: root.join("config"),
            data_dir: root.join("data"),
            cache_dir: root.join("cache"),
        }
    }

    /// Creates the configuration, data and cache directories.
    ///
    /// New directories get mode `0700`: they hold mail and settings that
    /// other users must not read. Existing directories are left as they are.
    pub fn create_dirs(&self) -> Result<()> {
        for dir in [
            &self.config_dir,
            &self.data_dir,
            &self.cache_dir,
            &self.attachments_dir(),
        ] {
            DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(dir)
                .map_err(|source| Error::io(dir, source))?;
        }
        Ok(())
    }

    /// `$XDG_CONFIG_HOME/katna/`
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// `$XDG_DATA_HOME/katna/`
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// `$XDG_CACHE_HOME/katna/` — only for data that is cheap to rebuild.
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Settings file: `$XDG_CONFIG_HOME/katna/config.toml`.
    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Mail database: `$XDG_DATA_HOME/katna/mail.db`.
    pub fn mail_db(&self) -> PathBuf {
        self.data_dir.join("mail.db")
    }

    /// Shared database (accounts, contacts, organizations):
    /// `$XDG_DATA_HOME/katna/pim.db`.
    pub fn pim_db(&self) -> PathBuf {
        self.data_dir.join("pim.db")
    }

    /// Calendar database: `$XDG_DATA_HOME/katna/calendar.db`.
    pub fn calendar_db(&self) -> PathBuf {
        self.data_dir.join("calendar.db")
    }

    /// Raw messages and small blobs: `$XDG_DATA_HOME/katna/blobs.db`.
    pub fn blobs_db(&self) -> PathBuf {
        self.data_dir.join("blobs.db")
    }

    /// Large blobs stored as files: `$XDG_DATA_HOME/katna/attachments/`.
    pub fn attachments_dir(&self) -> PathBuf {
        self.data_dir.join("attachments")
    }

    /// Search index: `$XDG_DATA_HOME/katna/index/`.
    pub fn index_dir(&self) -> PathBuf {
        self.data_dir.join("index")
    }

    /// Deletes everything Katna keeps: the data directory (mail, contacts,
    /// calendars, attachments, the search index), the cache and the
    /// settings file. Other files in the configuration directory are left
    /// alone; the directory goes only if nothing else is in it. Only the
    /// daemon calls this, with every writer stopped.
    pub fn delete_all_data(&self) -> Result<()> {
        let gone = |path: &Path, result: std::io::Result<()>| match result {
            Err(err) if err.kind() != std::io::ErrorKind::NotFound => Err(Error::io(path, err)),
            _ => Ok(()),
        };
        for dir in [&self.data_dir, &self.cache_dir] {
            gone(dir, std::fs::remove_dir_all(dir))?;
        }
        let config = self.config_file();
        gone(&config, std::fs::remove_file(&config))?;
        // Fails when the user keeps other files there.
        let _ = std::fs::remove_dir(&self.config_dir);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    fn lookup(vars: &[(&str, &str)]) -> impl Fn(&str) -> Option<OsString> {
        let vars: HashMap<String, OsString> = vars
            .iter()
            .map(|(k, v)| (k.to_string(), OsString::from(v)))
            .collect();
        move |name| vars.get(name).cloned()
    }

    #[test]
    fn defaults_to_home() {
        let paths = Paths::from_lookup(lookup(&[("HOME", "/home/ada")])).unwrap();
        assert_eq!(paths.config_dir(), Path::new("/home/ada/.config/katna"));
        assert_eq!(paths.data_dir(), Path::new("/home/ada/.local/share/katna"));
        assert_eq!(paths.cache_dir(), Path::new("/home/ada/.cache/katna"));
        assert_eq!(
            paths.config_file(),
            Path::new("/home/ada/.config/katna/config.toml")
        );
        assert_eq!(
            paths.mail_db(),
            Path::new("/home/ada/.local/share/katna/mail.db")
        );
    }

    #[test]
    fn xdg_variables_win_over_home() {
        let paths = Paths::from_lookup(lookup(&[
            ("HOME", "/home/ada"),
            ("XDG_CONFIG_HOME", "/cfg"),
            ("XDG_DATA_HOME", "/data"),
            ("XDG_CACHE_HOME", "/cache"),
        ]))
        .unwrap();
        assert_eq!(paths.config_dir(), Path::new("/cfg/katna"));
        assert_eq!(paths.blobs_db(), Path::new("/data/katna/blobs.db"));
        assert_eq!(paths.index_dir(), Path::new("/data/katna/index"));
        assert_eq!(paths.cache_dir(), Path::new("/cache/katna"));
    }

    #[test]
    fn relative_and_empty_xdg_values_are_ignored() {
        let paths = Paths::from_lookup(lookup(&[
            ("HOME", "/home/ada"),
            ("XDG_CONFIG_HOME", "relative/dir"),
            ("XDG_DATA_HOME", ""),
        ]))
        .unwrap();
        assert_eq!(paths.config_dir(), Path::new("/home/ada/.config/katna"));
        assert_eq!(paths.data_dir(), Path::new("/home/ada/.local/share/katna"));
    }

    #[test]
    fn needs_a_home_directory() {
        assert!(matches!(
            Paths::from_lookup(lookup(&[])),
            Err(Error::NoHomeDir)
        ));
        assert!(matches!(
            Paths::from_lookup(lookup(&[("HOME", "not/absolute")])),
            Err(Error::NoHomeDir)
        ));
        // Complete XDG settings do not need $HOME.
        assert!(
            Paths::from_lookup(lookup(&[
                ("XDG_CONFIG_HOME", "/c"),
                ("XDG_DATA_HOME", "/d"),
                ("XDG_CACHE_HOME", "/k"),
            ]))
            .is_ok()
        );
    }

    #[test]
    fn creates_private_directories() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        paths.create_dirs().unwrap();
        paths.create_dirs().unwrap(); // idempotent
        for dir in [paths.config_dir(), paths.data_dir(), paths.cache_dir()] {
            let mode = std::fs::metadata(dir).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o700, "{}", dir.display());
        }
        assert!(paths.attachments_dir().is_dir());
    }

    #[test]
    fn deletes_all_data() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        paths.create_dirs().unwrap();
        std::fs::write(paths.mail_db(), "mail").unwrap();
        std::fs::create_dir_all(paths.index_dir()).unwrap();
        std::fs::write(paths.index_dir().join("meta.json"), "{}").unwrap();
        std::fs::write(paths.cache_dir().join("pictures"), "").unwrap();
        std::fs::write(paths.config_file(), "[mail]\n").unwrap();
        paths.delete_all_data().unwrap();
        assert!(!paths.data_dir().exists());
        assert!(!paths.cache_dir().exists());
        assert!(!paths.config_dir().exists());
        // Nothing left to delete is fine.
        paths.delete_all_data().unwrap();

        // A file of the user's keeps the configuration directory.
        paths.create_dirs().unwrap();
        std::fs::write(paths.config_file(), "").unwrap();
        std::fs::write(paths.config_dir().join("notes.txt"), "mine").unwrap();
        paths.delete_all_data().unwrap();
        assert!(!paths.config_file().exists());
        assert!(paths.config_dir().join("notes.txt").exists());
    }
}
