use std::{env, fs, path::PathBuf};

use crate::language::LanguageType;

/// A configuration struct for how [`Languages::get_statistics`] searches and
/// counts languages.
///
/// ```
/// use tokei::Config;
///
/// let config = Config {
///     treat_doc_strings_as_comments: Some(true),
///     ..Config::default()
/// };
/// ```
///
/// [`Languages::get_statistics`]: struct.Languages.html#method.get_statistics
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// Width of columns to be printed to the terminal. _This option is ignored
    /// in the library._ *Default:* Auto detected width of the terminal.
    pub columns: Option<usize>,
    /// Count hidden files and directories. *Default:* `false`.
    pub hidden: Option<bool>,
    /// Don't respect ignore files. *Default:* `false`.
    pub no_ignore: Option<bool>,
    /// Don't respect ignore files in parent directories. *Default:* `false`.
    pub no_ignore_parent: Option<bool>,
    /// Don't respect VCS ignore files. *Default:* `false`.
    pub no_ignore_vcs: Option<bool>,
    /// Whether to treat doc strings in languages as comments.  *Default:*
    /// `false`.
    pub treat_doc_strings_as_comments: Option<bool>,
    /// Filters languages searched to just those provided. E.g. A directory
    /// containing `C`, `Cpp`, and `Rust` with a `Config.types` of `[Cpp, Rust]`
    /// will count only `Cpp` and `Rust`. *Default:* `None`.
    pub types: Option<Vec<LanguageType>>,
    // /// A map of individual language configuration.
    // pub languages: Option<HashMap<LanguageType, LanguageConfig>>,
}

impl Config {
    /// Get either `tokei.toml` or `.tokeirc`. `tokei.toml` takes precedence
    /// over `.tokeirc` as the latter is a hidden file on Unix and not idiomatic
    /// on Windows.
    fn get_config(base: PathBuf) -> Option<Self> {
        fs::read_to_string(base.join("tokei.toml"))
            .ok()
            .or_else(|| fs::read_to_string(base.join(".tokeirc")).ok())
            .and_then(|s| toml::from_str(&s).ok())
    }

    /// Creates a `Config` from three configuration files if they are available.
    /// Files can have two different names `tokei.toml` and `.tokeirc`.
    /// Firstly it will attempt to find a config in the configuration directory
    /// (see below), secondly from the home directory, `$HOME/`,
    /// and thirdly from the current directory, `./`.
    /// The current directory's configuration will take priority over the configuratio
    /// directory.
    ///
    /// |Platform | Value | Example |
    /// | ------- | ----- | ------- |
    /// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share | /home/alice/.local/share |
    /// | macOS   | `$HOME`/Library/Application Support | /Users/Alice/Library/Application Support |
    /// | Windows | `{FOLDERID_RoamingAppData}` | C:\Users\Alice\AppData\Roaming |
    ///
    /// # Example
    /// ```toml
    /// columns = 80
    /// types = ["Python"]
    /// treat_doc_strings_as_comments = true
    // ///
    // /// [[languages.Python]]
    // /// extensions = ["py3"]
    /// ```
    pub fn from_config_files() -> Self {
        let conf_dir = dirs::config_dir()
            .and_then(Self::get_config)
            .unwrap_or_else(Self::default);

        let home_dir = dirs::home_dir()
            .and_then(Self::get_config)
            .unwrap_or_else(Self::default);

        let current_dir = env::current_dir()
            .ok()
            .and_then(Self::get_config)
            .unwrap_or_else(Self::default);

        Config {
            columns: current_dir
                .columns
                .or(home_dir.columns.or(conf_dir.columns)),
            //languages: current_dir.languages.or(conf_dir.languages),
            treat_doc_strings_as_comments: current_dir.treat_doc_strings_as_comments.or(home_dir
                .treat_doc_strings_as_comments
                .or(conf_dir.treat_doc_strings_as_comments)),
            types: current_dir.types.or(home_dir.types.or(conf_dir.types)),
            no_ignore: current_dir
                .no_ignore
                .or(home_dir.no_ignore.or(conf_dir.no_ignore)),
            no_ignore_parent: current_dir
                .no_ignore_parent
                .or(home_dir.no_ignore_parent.or(conf_dir.no_ignore_parent)),
            no_ignore_vcs: current_dir
                .no_ignore_vcs
                .or(home_dir.no_ignore_vcs.or(conf_dir.no_ignore_vcs)),
            ..Self::default()
        }
    }
}

/*
/// Configuration for a individual [`LanguageType`].
///
/// ```
/// use std::collections::HashMap;
/// use tokei::{Config, LanguageConfig, LanguageType};
///
/// let config = Config {
///     languages: {
///         let cpp_conf = LanguageConfig {
///             extensions: vec![String::from("c")],
///         };
///
///         let mut languages_config = HashMap::new();
///         languages_config.insert(LanguageType::Cpp, cpp_conf);
///
///         Some(languages_config)
///     },
///
///     ..Config::default()
/// };
///
/// ```
///
/// [`LanguageType`]: enum.LanguageType.html
#[derive(Debug, Default, Deserialize)]
pub struct LanguageConfig {
    /// Additional extensions for a language. Any extensions that overlap with
    /// already defined extensions from `tokei` will be ignored.
    pub extensions: Vec<String>,
}

impl LanguageConfig {
    /// Creates a new empty configuration. By default this will not change
    /// anything from the default.
    pub fn new() -> Self {
        Self::default()
    }

    /// Accepts a `Vec<String>` representing additional extensions for a
    /// language. Any extensions that overlap with already defined extensions
    /// from `tokei` will be ignored.
    pub fn extensions(&mut self, extensions: Vec<String>) {
        self.extensions = extensions;
    }
}
*/
