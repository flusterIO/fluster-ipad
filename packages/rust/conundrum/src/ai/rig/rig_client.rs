use crate::ai::rig::{rig_client_local::RigClientLocal, rig_client_remote::RigClientRemote};

pub enum RigClient {
    Local(RigClientLocal),
    Remote(RigClientRemote),
}
