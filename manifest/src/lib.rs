//! utility functions and types to process manifest schema

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// top level object of a json modpack manifest
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModpackManifest {
    pub manifest: ManifestMetadata,
    pub modpack: ModpackSpecification,
}

/// metadata concerning what format to use when parsing a manifest
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestMetadata {
    pub version: String,
}

/// this contains the data that will be turned into the modpack
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModpackSpecification {
    pub name: String,
    pub version: String,
    pub codename: Option<String>,
    pub minecraft_version: String,
    pub mod_loaders: Vec<ModLoader>,
    pub curseforge_mods: Vec<CurseForgeMod>,
    pub modrinth_mods: Vec<ModrinthMod>,
    pub external_files: Vec<ExternalFile>,
}

/// modloaders supported by this launcher
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "name")]
pub enum ModLoader {
    Forge { version: String },
    Fabric { version: String },
}

///
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CurseForgeMod {
    pub slug: String,
}

///
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModrinthMod {
    pub slug: String,
}

///
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExternalFile {
    pub download_url: String,
    pub target_file: String,
}
