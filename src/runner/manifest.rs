use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Manifest {
    #[serde(rename = "kono")]
    Kono(KonoManifest),
}

#[derive(Deserialize, Debug)]
pub struct KonoManifest {
    pub prefix: String,
    pub runner: String,
}
