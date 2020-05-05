extern crate dh11 as dht11;

fn main() {
    println!("Hello, world!");
    let dht11_pin = 17;
    let dht11 = dht11::DHT11::new(dht11_pin).unwrap();
    dht11.start().unwrap();
}