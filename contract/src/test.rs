use crate::prelude::*;

pub struct TestClient {}

impl Client for TestClient {}
impl LogTemperatureMutation for TestClient {
    type Err = ();
    async fn mutation(&self, command: LogTemperature) -> Result<Self::Res, ()> {
        todo!()
    }
}
impl ListTemperaturesQuery for TestClient {
    type Err = ();
    async fn query(&self, command: ListTemperatures) -> Result<Self::Res, ()> {
        todo!()
    }
}

async fn test() {
    let test = TestClient {};
    let response: Result<Vec<Temperature>, ()> = test.query(ListTemperatures).await;
    let response: Result<Temperature, ()> = test
        .mutation(LogTemperature {
            temperature: 32.0,
            host_name: "test host".to_owned(),
            humidity: 50.0,
        })
        .await;
}
