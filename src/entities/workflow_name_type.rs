use std::str::FromStr;

const MAESTRO_JOB_NAME: &str = "maestro-";

#[derive(Clone, Debug)]
pub enum WorkflowNameType {
    DefinedName(String),
    GenerateName(String)
}

impl Default for WorkflowNameType {
    fn default() -> Self {
        WorkflowNameType::GenerateName(MAESTRO_JOB_NAME.to_owned())
    }
}


impl FromStr for WorkflowNameType {
    type Err =  anyhow::Error;

    fn from_str(source: &str) -> anyhow::Result<Self> {
        let name = WorkflowNameType::DefinedName(source.to_owned());
        Ok(name)
    }
}