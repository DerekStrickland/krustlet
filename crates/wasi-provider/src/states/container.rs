use crate::ModuleRunContext;
use crate::ProviderState;
use kubelet::container::{Container, ContainerKey, Status};
use kubelet::pod::Pod;
use kubelet::state::{ResourceState, SharedState};

pub(crate) mod running;
pub(crate) mod terminated;
pub(crate) mod waiting;

pub(crate) struct ContainerState {
    pod: Pod,
    container_key: ContainerKey,
    run_context: SharedState<ModuleRunContext>,
}

impl ContainerState {
    pub fn new(
        pod: Pod,
        container_key: ContainerKey,
        run_context: SharedState<ModuleRunContext>,
    ) -> Self {
        ContainerState {
            pod,
            container_key,
            run_context,
        }
    }
}

#[async_trait::async_trait]
impl ResourceState for ContainerState {
    type Manifest = Container;
    type Status = Status;
    type SharedState = ProviderState;
    async fn async_drop(self, _shared_state: &mut Self::SharedState) {}
}
