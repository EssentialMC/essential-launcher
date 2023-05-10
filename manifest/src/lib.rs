use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let json_str = read_to_string("./example-modpack.json").unwrap();
        dbg!(serde_json::from_str::<ModpackManifest>(&json_str).unwrap());
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ModpackManifest {
    manifest: ManifestMetadata,
    modpack: ModpackSpecification,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ManifestMetadata {
    version: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ModpackSpecification {
    name: String,
    version: String,
    codename: Option<String>,
    minecraft_version: String,
    mod_loaders: Vec<ModLoader>,
    curseforge_mods: Vec<CurseForgeMod>,
    modrinth_mods: Vec<ModrinthMod>,
    external_files: Vec<ExternalFile>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ModLoader {
    name: String,
    version: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct CurseForgeMod {
    slug: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ModrinthMod {
    slug: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ExternalFile {
    download_url: String,
    target_file: String,
}
