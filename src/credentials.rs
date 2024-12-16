use crate::oauth2;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Default, Clone)]
struct Credentials {
    data: CredentialsData,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct CredentialsData {
    google: Option<oauth2::OAuth2Token>,
}
#[derive(thiserror::Error, Debug)]
enum CredentialsError {
    #[error("file access error: {0}")]
    Io(io::Error),
    #[error("yaml reading/writing error: {0}")]
    YamlParse(serde_yml::Error),
}

impl From<serde_yml::Error> for CredentialsError {
    fn from(value: serde_yml::Error) -> Self {
        Self::YamlParse(value)
    }
}
impl From<io::Error> for CredentialsError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Credentials {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), CredentialsError> {
        serde_yml::to_writer(
            io::BufWriter::new(fs::File::create(path.as_ref())?),
            &self.data,
        )?;
        Ok(())
    }
    pub fn load(path: impl AsRef<Path>) -> Result<Self, CredentialsError> {
        Ok(Self {
            data: serde_yml::from_reader(io::BufReader::new(fs::File::open(path.as_ref())?))?,
        })
    }
    pub fn load_or_create(path: impl AsRef<Path>) -> Result<Self, CredentialsError> {
        if fs::exists(path.as_ref())? {
            Self::load(path)
        } else {
            Ok(Self::new())
        }
    }
    pub fn google(&mut self) -> &mut Option<oauth2::OAuth2Token> {
        &mut self.data.google
    }
}
