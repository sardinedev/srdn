use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    env,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

fn true_by_default() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CssModulesOption {
  Bool(bool),
  Config(CssModulesConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CssModulesConfig {
  pub pattern: Option<String>,
  #[serde(default = "true_by_default")]
  pub dashed_idents: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SrdnSettings {
    #[serde(rename = "cssModules")]
    pub css_modules: Option<CssModulesOption>,
    #[serde(default)]
    pub minify: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Exports {
    #[serde(default)]
    default: Option<String>,
    #[serde(default)]
    require: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub browserslist: Option<Vec<String>>,

    #[serde(default)]
    pub source: String,

    #[serde(default)]
    pub main: Option<String>,

    #[serde(default)]
    pub srdn: SrdnSettings,

    #[serde(default)]
    pub exports: Option<Exports>,
}

pub fn read_package() -> Option<Settings> {
    let settings_path = find_settings();
    if let Some(file_path) = settings_path {
        let settings = read_to_string(file_path).unwrap();
        return Some(
            serde_json::from_str::<Settings>(&settings)
                .expect("failed to read package.json"),
        );
    }
    None
}

fn find_settings() -> Option<PathBuf> {
    let current_dir = env::current_dir().unwrap();
    let ancestors = current_dir.ancestors();
    for ancestor in ancestors {
        let dir = ancestor;
        if let Ok(entry) = read_dir(dir) {
            for file in entry {
                let file_path = file.unwrap();
                let filename = file_path.file_name();
                if filename == "package.json" {
                    return Some(file_path.path());
                }
                // Feels a bit risky. Files should be caught before hidden folders but.. :|
                if filename == ".git" {
                    return None;
                }
            }
        }
    }
    None
}
