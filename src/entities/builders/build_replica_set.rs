use k8s_openapi::api::apps::v1::ReplicaSet;

use crate::entities::job::JobBuilder;

pub trait BuildReplicaSet {
    fn try_build(self) -> anyhow::Result<ReplicaSet>;
}

impl BuildReplicaSet for JobBuilder {
    fn try_build(self) -> anyhow::Result<ReplicaSet> {
        let replica_set = ReplicaSet {
            ..ReplicaSet::default()
        };

        Ok(replica_set)
    }
}