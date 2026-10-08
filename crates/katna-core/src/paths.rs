// SPDX-License-Identifier: GPL-3.0-or-later

//! XDG base directories (on Windows, the AppData folders) and the files
//! Katna keeps in them (`docs/ARCHITECTURE.md` §5.1).

use std::ffi::OsString;
use std::fs::DirBuilder;
#[cfg(unix)]
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// Name of the Katna subdirectory in each XDG base directory.
const APP_DIR: &str = "katna";

/// Name of the Katna folder in `%APPDATA%` and `%LOCALAPPDATA%` on Windows.
const WINDOWS_APP_DIR: &str = "Katna";

/// Where Katna keeps its configuration and data.
///
/// Build it with [`Paths::from_env`] in programs and [`Paths::with_root`] in
/// tests. Nothing is created until [`Paths::create_dirs`] is called.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paths {
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    state_dir: PathBuf,
}

impl Paths {
    /// Resolves the directories from `$XDG_CONFIG_HOME`, `$XDG_DATA_HOME`,
    /// `$XDG_CACHE_HOME`, `$XDG_STATE_HOME` and `$HOME`; on Windows from
    /// `%APPDATA%` and `%LOCALAPPDATA%` ([`Paths::from_windows_lookup`]).
    pub fn from_env() -> Result<Self> {
        if cfg!(windows) {
            Self::from_windows_lookup(|name| std::env::var_os(name))
        } else {
            Self::from_lookup(|name| std::env::var_os(name))
        }
    }

    /// The Windows layout: settings in `%APPDATA%\Katna` (they roam with
    /// the user's profile), mail and everything else in
    /// `%LOCALAPPDATA%\Katna\{Data,Cache,State}`, which stays on this
    /// computer because it can be large.
    pub fn from_windows_lookup(lookup: impl Fn(&str) -> Option<OsString>) -> Result<Self> {
        let absolute = |name: &str| {
            lookup(name)
                .map(PathBuf::from)
                .filter(|path| path.is_absolute())
        };
        let local = absolute("LOCALAPPDATA")
            .ok_or(Error::NoHomeDir)?
            .join(WINDOWS_APP_DIR);
        let config_dir = absolute("APPDATA")
            .map(|dir| dir.join(WINDOWS_APP_DIR))
            .unwrap_or_else(|| local.join("Config"));
        Ok(Self {
            config_dir,
            data_dir: local.join("Data"),
            cache_dir: local.join("Cache"),
            state_dir: local.join("State"),
        })
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
        let cache_dir = base("XDG_CACHE_HOME", ".cache")?;
        Ok(Self {
            config_dir: base("XDG_CONFIG_HOME", ".config")?,
            data_dir: base("XDG_DATA_HOME", ".local/share")?,
            // Only crash reports live here; without $HOME or
            // $XDG_STATE_HOME they go under the cache.
            state_dir: base("XDG_STATE_HOME", ".local/state")
                .unwrap_or_else(|_| cache_dir.join("state")),
            cache_dir,
        })
    }

    /// Puts every directory under `root` (`root/config`, `root/data`,
    /// `root/cache`, `root/state`). For tests and portable setups.
    pub fn with_root(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref();
        Self {
            config_dir: root.join("config"),
            data_dir: root.join("data"),
            cache_dir: root.join("cache"),
            state_dir: root.join("state"),
        }
    }

    /// Creates the configuration, data, cache and state directories.
    ///
    /// New directories get mode `0700`: they hold mail, settings and crash
    /// reports that other users must not read. Existing directories are
    /// left as they are. On Windows the user's profile folders are already
    /// private.
    pub fn create_dirs(&self) -> Result<()> {
        for dir in [
            &self.config_dir,
            &self.data_dir,
            &self.cache_dir,
            &self.state_dir,
            &self.attachments_dir(),
        ] {
            let mut builder = DirBuilder::new();
            builder.recursive(true);
            #[cfg(unix)]
            builder.mode(0o700);
            builder
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

    /// `$XDG_STATE_HOME/katna/` — kept across runs but not worth a backup.
    pub fn state_dir(&self) -> &Path {
        &self.state_dir
    }

    /// How the mail window was when it closed:
    /// `$XDG_STATE_HOME/katna/mail-window.toml`.
    pub fn mail_window_file(&self) -> PathBuf {
        self.state_dir.join("mail-window.toml")
    }

    /// The daemon's start record and self-check:
    /// `$XDG_STATE_HOME/katna/health.toml` (`docs/ARCHITECTURE.md` §21.2).
    pub fn health_file(&self) -> PathBuf {
        self.state_dir.join("health.toml")
    }

    /// Crash reports, one text file per crash:
    /// `$XDG_STATE_HOME/katna/crashes/` (`docs/ARCHITECTURE.md` §19.2).
    pub fn crash_dir(&self) -> PathBuf {
        self.state_dir.join("crashes")
    }

    /// Settings file: `$XDG_CONFIG_HOME/katna/config.toml`.
    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Senders whose remote images are shown, one address per line:
    /// `$XDG_CONFIG_HOME/katna/trusted-senders`.
    pub fn trusted_senders_file(&self) -> PathBuf {
        self.config_dir.join("trusted-senders")
    }

    /// Pictures the user picked for their own accounts, one file per
    /// account ID: `$XDG_DATA_HOME/katna/account-pictures/`.
    pub fn account_pictures_dir(&self) -> PathBuf {
        self.data_dir.join("account-pictures")
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
    /// calendars, attachments, the search index), the cache, the state
    /// directory (crash reports) and the settings file. Other files in the configuration directory are left
    /// alone; the directory goes only if nothing else is in it. Only the
    /// daemon calls this, with every writer stopped.
    pub fn delete_all_data(&self) -> Result<()> {
        let gone = |path: &Path, result: std::io::Result<()>| match result {
            Err(err) if err.kind() != std::io::ErrorKind::NotFound => Err(Error::io(path, err)),
            _ => Ok(()),
        };
        for dir in [&self.data_dir, &self.cache_dir, &self.state_dir] {
            gone(dir, std::fs::remove_dir_all(dir))?;
        }
        for file in [self.config_file(), self.trusted_senders_file()] {
            gone(&file, std::fs::remove_file(&file))?;
        }
        // Fails when the user keeps other files there.
        let _ = std::fs::remove_dir(&self.config_dir);
        Ok(())
    }
}

/// Creates `dir` and any missing parents with mode `0700`, and makes `dir`
/// itself `0700` if it already existed: for directories of files other
/// users must not read, such as crash reports. On Windows the user's
/// profile folders are already private.
pub fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    builder.mode(0o700);
    builder.create(dir)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = std::fs::metadata(dir)?.permissions();
        if permissions.mode() & 0o077 != 0 {
            std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    fn lookup(vars: &[(&str, &str)]) -> impl Fn(&str) -> Option<OsString> {
        let vars: HashMap<String, OsString> = vars
            .iter()
            .map(|(k, v)| (k.to_string(), OsString::from(v)))
            .collect();
        move |name| vars.get(name).cloned()
    }

    // XDG paths start with `/`, which is not a whole path on Windows.
    #[cfg(unix)]
    #[test]
    fn defaults_to_home() {
        let paths = Paths::from_lookup(lookup(&[("HOME", "/home/ada")])).unwrap();
        assert_eq!(paths.config_dir(), Path::new("/home/ada/.config/katna"));
        assert_eq!(paths.data_dir(), Path::new("/home/ada/.local/share/katna"));
        assert_eq!(paths.cache_dir(), Path::new("/home/ada/.cache/katna"));
        assert_eq!(
            paths.crash_dir(),
            Path::new("/home/ada/.local/state/katna/crashes")
        );
        assert_eq!(
            paths.config_file(),
            Path::new("/home/ada/.config/katna/config.toml")
        );
        assert_eq!(
            paths.mail_db(),
            Path::new("/home/ada/.local/share/katna/mail.db")
        );
    }

    // XDG paths start with `/`, which is not a whole path on Windows.
    #[cfg(unix)]
    #[test]
    fn xdg_variables_win_over_home() {
        let paths = Paths::from_lookup(lookup(&[
            ("HOME", "/home/ada"),
            ("XDG_CONFIG_HOME", "/cfg"),
            ("XDG_DATA_HOME", "/data"),
            ("XDG_CACHE_HOME", "/cache"),
            ("XDG_STATE_HOME", "/state"),
        ]))
        .unwrap();
        assert_eq!(paths.state_dir(), Path::new("/state/katna"));
        assert_eq!(paths.config_dir(), Path::new("/cfg/katna"));
        assert_eq!(paths.blobs_db(), Path::new("/data/katna/blobs.db"));
        assert_eq!(paths.index_dir(), Path::new("/data/katna/index"));
        assert_eq!(paths.cache_dir(), Path::new("/cache/katna"));
    }

    // XDG paths start with `/`, which is not a whole path on Windows.
    #[cfg(unix)]
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

    // XDG paths start with `/`, which is not a whole path on Windows.
    #[cfg(unix)]
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
    fn windows_uses_appdata() {
        let root = if cfg!(windows) {
            r"C:\Users\ada"
        } else {
            "/Users/ada"
        };
        let roaming = Path::new(root).join("AppData").join("Roaming");
        let local = Path::new(root).join("AppData").join("Local");
        let paths = Paths::from_windows_lookup(lookup(&[
            ("APPDATA", roaming.to_str().unwrap()),
            ("LOCALAPPDATA", local.to_str().unwrap()),
            ("HOME", "/ignored"),
        ]))
        .unwrap();
        let katna = local.join("Katna");
        assert_eq!(
            paths.config_file(),
            roaming.join("Katna").join("config.toml")
        );
        assert_eq!(paths.mail_db(), katna.join("Data").join("mail.db"));
        assert_eq!(paths.cache_dir(), katna.join("Cache"));
        assert_eq!(paths.crash_dir(), katna.join("State").join("crashes"));
        assert!(matches!(
            Paths::from_windows_lookup(lookup(&[("APPDATA", root)])),
            Err(Error::NoHomeDir)
        ));
    }

    #[cfg(unix)]
    #[test]
    fn creates_private_directories() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        paths.create_dirs().unwrap();
        paths.create_dirs().unwrap(); // idempotent
        for dir in [
            paths.config_dir(),
            paths.data_dir(),
            paths.cache_dir(),
            paths.state_dir(),
        ] {
            let mode = std::fs::metadata(dir).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o700, "{}", dir.display());
        }
        assert!(paths.attachments_dir().is_dir());
    }

    #[cfg(unix)]
    #[test]
    fn private_directories_are_tightened() {
        let tmp = tempfile::tempdir().unwrap();
        let crashes = tmp.path().join("state").join("crashes");
        create_private_dir(&crashes).unwrap();
        for dir in [crashes.parent().unwrap(), &crashes] {
            let mode = std::fs::metadata(dir).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o700, "{}", dir.display());
        }
        // One made by an older version with the default umask.
        std::fs::set_permissions(&crashes, std::fs::Permissions::from_mode(0o755)).unwrap();
        create_private_dir(&crashes).unwrap();
        let mode = std::fs::metadata(&crashes).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o700);
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
        std::fs::write(paths.trusted_senders_file(), "a@example.com\n").unwrap();
        std::fs::create_dir_all(paths.crash_dir()).unwrap();
        paths.delete_all_data().unwrap();
        assert!(!paths.data_dir().exists());
        assert!(!paths.state_dir().exists());
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
