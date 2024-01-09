use serde_bencode::Value;

fn main() {
    let content = std::fs::read("./examples/ubuntu.torrent").unwrap();
    let bencode: Value = serde_bencode::from_binary(&content).unwrap();

    println!("Bencode: {bencode:#?}");
}
