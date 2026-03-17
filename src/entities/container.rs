pub struct ComputeResource(pub String);

impl ComputeResource {
    pub fn cpu(v: impl Into<String>) -> Self {
        Self(v.into())
    }
    pub fn memory(v: impl Into<String>) -> Self {
        Self(v.into())
    }
}

impl AsRef<str> for ComputeResource {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
