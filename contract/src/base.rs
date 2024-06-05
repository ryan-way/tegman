use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Query;
#[derive(Serialize, Deserialize)]
pub(crate) struct Mutation;

pub(crate) trait Operation {}
impl Operation for Query {}
impl Operation for Mutation {}

#[derive(Serialize, Deserialize)]
pub struct Payload<O: Operation, C> {
    operation: O,
    command: C,
}

pub(crate) trait Contract<O, C>
where
    O: Operation + Serialize + DeserializeOwned,
    C: Serialize + DeserializeOwned,
{
}

pub trait QueryContract<C>
where
    C: Serialize + DeserializeOwned,
{
    type Res: Serialize + DeserializeOwned;
}

impl<T, C> Contract<Query, C> for T
where
    T: QueryContract<C>,
    C: Serialize + DeserializeOwned,
{
}

pub trait MutationContract<C>: Contract<Mutation, C>
where
    C: Serialize + DeserializeOwned,
{
    type Res: Serialize + DeserializeOwned;
}

impl<T, C> Contract<Mutation, C> for T
where
    T: MutationContract<C>,
    C: Serialize + DeserializeOwned,
{
}
