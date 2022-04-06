#[derive(Clone, Debug, knuffel::Decode)]
pub struct ModpackManifest {
    #[knuffel(child)]
    pub manifest: ManifestNode,
    #[knuffel(child)]
    pub modpack: ModpackNode,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct ManifestNode {
    #[knuffel(property)]
    pub version: f32,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct ModpackNode {
    #[knuffel(child, unwrap(argument))]
    pub name: String,
    #[knuffel(child, unwrap(argument))]
    pub version: String,
    #[knuffel(child, unwrap(argument))]
    pub minecraft: String,
    #[knuffel(child, unwrap(argument))]
    pub description: String,
    #[knuffel(children(name = "link"))]
    pub links: Vec<LinkNode>,
    #[knuffel(children(name = "author"))]
    pub authors: Vec<AuthorNode>,
    #[knuffel(children(name = "loader"))]
    pub loaders: Vec<LoaderNode>,
    #[knuffel(children(name = "libraries"))]
    pub libraries: Vec<LibraryNode>,
    #[knuffel(children(name = "feature"))]
    pub features: Vec<FeatureNode>,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct LinkNode {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(property)]
    pub url: String,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct AuthorNode {
    #[knuffel(argument)]
    pub handle: String,
    #[knuffel(property)]
    pub name: Option<String>,
    #[knuffel(child, unwrap(argument))]
    pub email: Option<String>,
    #[knuffel(children(name = "link"))]
    pub links: Vec<LinkNode>,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct LoaderNode {
    #[knuffel(property)]
    pub kind: ModLoader,
    #[knuffel(property)]
    pub version: String,
}

#[derive(Clone, Debug, knuffel::DecodeScalar)]
pub enum ModLoader {
    Fabric,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct LibraryNode {
    #[knuffel(property)]
    pub optional: bool,
    #[knuffel(children(name = "mod"))]
    pub mods: Vec<ModNode>,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct FeatureNode {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(children(name = "mod"))]
    pub mods: Vec<ModNode>,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct ModNode {
    #[knuffel(argument)]
    pub slug: String,
    #[knuffel(property)]
    pub side: ModSide,
    #[knuffel(property)]
    pub provider: Option<Provider>,
    #[knuffel(children(name = "provider"))]
    pub providers: Vec<ProviderNode>,
}

#[derive(Clone, Debug, knuffel::DecodeScalar)]
pub enum ModSide {
    Client,
    Server,
    Both,
}

#[derive(Clone, Debug, knuffel::Decode)]
pub struct ProviderNode {
    #[knuffel(argument)]
    pub provider: Provider,
    #[knuffel(property)]
    pub slug: String,
}

#[derive(Clone, Debug, knuffel::DecodeScalar)]
pub enum Provider {
    Curseforge,
    Modrinth,
    Both,
}

#[cfg(test)]
mod tests {
    use super::ModpackManifest;

    #[test]
    fn parse_sample_manifest() {
        let path =
            "/home/jacob/Documents/github.com/EssentialMC/fabric-essentials-modpack/manifest.kdl";
        let manifest = knuffel::parse::<ModpackManifest>(
            "fabric-essentials-modpack/manifest.kdl",
            &std::fs::read_to_string(path).unwrap(),
        );

        match manifest {
            Ok(manifest) => {
                println!("{:#?}", manifest);
                std::fs::write("parsed-manifest.ron", format!("{:#?}", manifest)).unwrap();
            }
            Err(error) => panic!("{:?}", miette::Report::new(error)),
        };
    }
}
