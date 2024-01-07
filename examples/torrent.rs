use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Torrent {
    announce: Option<String>,
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<String>>,
    info: TorrentInfo,
    #[serde(rename = "creation date")]
    creation_date: Option<u32>,
    comment: Option<String>,

    #[serde(rename = "created by")]
    created_by: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TorrentInfo {
    #[serde(rename = "piece length")]
    piece_length: usize,
    pieces: String,
    #[serde(default)]
    private: u8,
    name: String,

    files: Vec<TorrentFile>,
}

#[derive(Deserialize, Debug)]
struct TorrentFile {
    length: usize,
    path: Vec<String>,
}

fn main() {
    let content = std::fs::read("./examples/example.torrent").unwrap();

    let torrent: Torrent = serde_bencode::from_binary(&content).unwrap();

    println!("Torrent info: {torrent:#?}");
}
