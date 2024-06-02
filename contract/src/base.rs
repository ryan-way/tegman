use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::util::Result;

#[derive(Serialize, Deserialize)]
pub(crate) struct Query;
#[derive(Serialize, Deserialize)]
pub(crate) struct Mutation;

pub(crate) trait Operation {}
impl Operation for Query {}
impl Operation for Mutation {}

#[derive(Serialize, Deserialize)]
pub(crate) struct Payload<O: Operation, C> {
    operation: O,
    command: C,
}

pub(crate) trait Sender {
    fn send(data: &str) -> Result<String>;
    fn serialize<T: Serialize>(data: &T) -> Result<String>;
    fn deserialize<T: DeserializeOwned>(data: &str) -> Result<T>;
}

pub(crate) trait Contract<O, C, Res>: Sender
where
    O: Operation + Serialize + DeserializeOwned,
    C: Serialize + DeserializeOwned,
    Res: Serialize + DeserializeOwned,
{
}

pub(crate) trait OperationContract<O, C, Res>: Contract<O, C, Res>
where
    O: Operation + Serialize + DeserializeOwned,
    C: Serialize + DeserializeOwned,
    Res: Serialize + DeserializeOwned,
{
    fn operation(&self, operation: O, command: C) -> Result<Res> {
        let request = Payload { operation, command };

        let request_data = Self::serialize(&request)?;
        let respone_data = Self::send(&request_data)?;
        let response: Res = Self::deserialize(&respone_data)?;

        Ok(response)
    }
}

impl<T, O, C, Res> Contract<O, C, Res> for T
where
    T: OperationContract<O, C, Res>,
    O: Operation + Serialize + DeserializeOwned,
    C: Serialize + DeserializeOwned,
    Res: Serialize + DeserializeOwned,
{
}

pub trait QueryContract<C>: OperationContract<Query, C, Self::Res>
where
    C: Serialize + DeserializeOwned,
{
    type Res: Serialize + DeserializeOwned;
    fn query(&self, command: C) -> Result<Self::Res> {
        self.operation(Query, command)
    }
}

impl<T, C> OperationContract<Query, C, T::Res> for T
where
    T: QueryContract<C>,
    C: Serialize + DeserializeOwned,
{
}

pub trait MutationContract<C>: OperationContract<Mutation, C, Self::Res>
where
    C: Serialize + DeserializeOwned,
{
    type Res: Serialize + DeserializeOwned;
    fn mutation(&self, command: C) -> Result<Self::Res> {
        self.operation(Mutation, command)
    }
}

impl<T, C> OperationContract<Mutation, C, T::Res> for T
where
    T: MutationContract<C>,
    C: Serialize + DeserializeOwned,
{
}
