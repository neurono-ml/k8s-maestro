use k8s_openapi::api::batch::v1::Job;

use crate::entities::job::JobBuilder;

pub trait BuildJob {
    fn try_build(self) -> anyhow::Result<Job>;
}

impl BuildJob for JobBuilder {
    fn try_build(self) -> anyhow::Result<Job> {
        self.build()
    }
}