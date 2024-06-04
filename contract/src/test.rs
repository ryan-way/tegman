use crate::prelude::*;

pub struct TestClient {}

impl Client for TestClient {}
impl LogTemperatureMutation for TestClient {
    fn mutation(&self, command: LogTemperature) -> Result<Self::Res> {
        todo!()
    }
}
impl ListTemperaturesQuery for TestClient {
    fn query(&self, command: ListTemperatures) -> Result<Self::Res> {
        todo!()
    }
}

fn test() {
    let test = TestClient {};
    let response = test.query(ListTemperatures);
    let response = test.mutation(LogTemperature { temperature: 32.0 });
}
