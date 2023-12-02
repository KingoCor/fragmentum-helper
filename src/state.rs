use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::structures::Structure;
use crate::resources::Payment;
use crate::aspects::{Aspect, AspectClass};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub aspects: HashMap<AspectClass, Aspect>,
    pub resources: Payment,
    pub structures: Vec::<Structure>,
}
