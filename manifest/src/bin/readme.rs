// standard library
use std::fs::read_to_string;
use std::path::PathBuf;

// our code
use manifest::ModpackManifest;

// other peoples code
use argh::FromArgs;
use serde_json::Value as JsonValue;

/// the cli arguments for the readme generator
#[derive(Debug, FromArgs)]
struct ReadmeCli {
    /// path to the manifest that is used to make the readme
    #[argh(positional)]
    input_manifest: PathBuf,
}

fn main() {
    let cli: ReadmeCli = argh::from_env();

    if cli.input_manifest.exists() && cli.input_manifest.is_file() {
        let json_str = read_to_string(&cli.input_manifest).unwrap();
        dbg!(serde_json::from_str::<ModpackManifest>(&json_str).unwrap());
    } else {
        panic!("The path provided does not exist or is not a file");
    }
}
