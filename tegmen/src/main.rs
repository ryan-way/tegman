extern crate dht22_pi;

use dht22_pi::read;

fn main() {
    let result = match read(2) {
        Ok(reading) => reading,
        Err(e) => {
            println!("ERROR: {:?}", e);
            return;
        }
    };
    println!("Temperature: {}", result.temperature);
    println!("Humidity: {}", result.humidity);
}
