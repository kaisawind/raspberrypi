use pi_rfid::Mfrc522;

#[test]
pub fn new_mfrc522() {
    let mut mfrc = Mfrc522::new().unwrap();
    match mfrc.read_id() {
        Ok(_) => println!("read id ok"),
        Err(err) => println!("read id err {}", err),
    };
    println!("mfrc522");
}
