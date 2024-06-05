use base::{MutationContract, QueryContract};
use command::{ListTemperatures, LogTemperature};

mod base;
pub mod command;
pub mod prelude;
mod test;

pub trait Client: QueryContract<ListTemperatures> + MutationContract<LogTemperature> {}
