use std::str::FromStr;

pub enum LabelOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist
}

impl ToString for LabelOperator {
    fn to_string(&self) -> String {
        let string_value = 
            match self {
                LabelOperator::In => "In",
                LabelOperator::NotIn => "NotIn",
                LabelOperator::Exists => "Exists",
                LabelOperator::DoesNotExist => "DoesNotExist",
            };

        string_value.to_owned()
    }
}

impl FromStr for LabelOperator {
    type Err = anyhow::Error;

    fn from_str(operator_string: &str) -> anyhow::Result<Self> {
        let converted = 
            if operator_string.to_lowercase().eq(&LabelOperator::In.to_string().to_lowercase()) {
                LabelOperator::In
            } else if operator_string.to_lowercase().eq(&LabelOperator::NotIn.to_string().to_lowercase()) {
                LabelOperator::NotIn
            } else if operator_string.to_lowercase().eq(&LabelOperator::Exists.to_string().to_lowercase()) {
                LabelOperator::Exists
            } else if operator_string.to_lowercase().eq(&LabelOperator::DoesNotExist.to_string().to_lowercase()) {
                LabelOperator::DoesNotExist
            } else {
                anyhow::bail!("Can't parse value {operator_string} into a kubernetes label operator")
            };

        Ok(converted)
    }
}