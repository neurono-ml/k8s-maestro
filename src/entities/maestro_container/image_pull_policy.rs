#[derive(Debug, Clone)]
pub enum ImagePullPolicy {
    Always,
    IfNotPresent,
    Never
}

impl Into<String> for ImagePullPolicy {
    fn into(self) -> String {
        match self {
            Self::Always => "Always".to_owned(),
            Self::IfNotPresent => "IfNotPresent".to_owned(),
            Self::Never => "Never".to_owned(),
        }
    }
}

impl TryFrom<String> for ImagePullPolicy {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.to_lowercase().eq("always") {
            Ok(Self::Always)
        } else if value.to_lowercase().eq("ifnotpresent") {
            Ok(Self::IfNotPresent)
        } else if value.to_lowercase().eq("never") {
            Ok(Self::Never)
        } else {
            anyhow::bail!("Can't convert string `{}` to ImagePullPolicy", value)
        }
    }
}

impl Default for ImagePullPolicy {
    fn default() -> Self {
        ImagePullPolicy::IfNotPresent
    }
}