use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct InstanceId(pub i32);
