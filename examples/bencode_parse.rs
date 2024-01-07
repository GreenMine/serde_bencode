fn main() {
    let content = std::fs::read("./examples/example2.torrent").unwrap();
    let bencode: serde_bencode::Value = serde_bencode::from_binary(&content).unwrap();
    println!("Bencode data: {bencode:#?}");
}
