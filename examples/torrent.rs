use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Torrent<'a> {
    announce: Option<String>,
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(borrow)]
    info: TorrentInfo<'a>,
    #[serde(rename = "creation date")]
    creation_date: Option<u32>,
    comment: Option<String>,

    #[serde(rename = "created by")]
    created_by: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TorrentInfo<'a> {
    #[serde(rename = "piece length")]
    piece_length: usize,
    #[serde(borrow)]
    pieces: &'a [u8],
    #[serde(default)]
    private: u8,
    name: String,

    files: Option<Vec<TorrentFile>>,
}

#[derive(Deserialize, Debug)]
struct TorrentFile {
    length: usize,
    path: Vec<String>,
}

fn main() {
    let content = std::fs::read("./examples/ubuntu.torrent").unwrap();

    let torrent: Torrent = serde_bencode::from_binary(&content).unwrap();

    println!("Torrent info: {torrent:?}");
}
