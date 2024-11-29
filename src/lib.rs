pub mod entities;
pub mod clients;

pub mod k8s {
    
    #[cfg(feature = "kube")]
    pub use kube;

    #[cfg(feature = "k8s-openapi")]
    pub use k8s_openapi;

    #[cfg(feature = "kube")]
    pub use log;

}