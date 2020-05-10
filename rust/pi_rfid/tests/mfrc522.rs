use pi_rfid::Mfrc522;

#[test]
pub fn new_mfrc522() {
    let _ = Mfrc522::new();
    println!("mfrc522");
}
