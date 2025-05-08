use std::str::FromStr;

const MAESTRO_JOB_NAME: &str = "maestro-";

#[derive(Clone, Debug)]
pub enum JobNameType {
    DefinedName(String),
    GenerateName(String)
}

impl Default for JobNameType {
    fn default() -> Self {
        JobNameType::GenerateName(MAESTRO_JOB_NAME.to_owned())
    }
}


impl FromStr for JobNameType {
    type Err =  anyhow::Error;

    fn from_str(source: &str) -> anyhow::Result<Self> {
        let name = JobNameType::DefinedName(source.to_owned());
        Ok(name)
    }
}