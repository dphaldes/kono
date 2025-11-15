use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub prefix: String,
    pub runner: String,
}
