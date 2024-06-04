use base::{MutationContract, QueryContract};
use command::{ListTemperatures, LogTemperature};

mod base;
pub mod command;
pub mod prelude;
mod test;
mod util;

pub trait Client: QueryContract<ListTemperatures> + MutationContract<LogTemperature> {}
