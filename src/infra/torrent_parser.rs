use std::fs;

use serde_bencode::de;
use serde_bytes::ByteBuf;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Node(String, i64);

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}

impl Torrent {
    #[allow(dead_code)]
    pub fn read_from_file(path: &str) -> Result<Torrent, serde_bencode::Error> {
        let file = fs::read(path).unwrap();

        let torrent = de::from_bytes::<Self>(&file);

        return torrent;
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Torrent;

    #[test]
    fn should_read_torrent_from_file() {
        let file_path = Path::new("./tests/fixtures/archlinux-2024.01.01-x86_64.iso.torrent");
        let torrent = Torrent::read_from_file(&file_path.display().to_string());

        let torrent = torrent.unwrap();

        println!("{:?}", torrent);

        assert_eq!(
            torrent.info.name,
            "archlinux-2024.01.01-x86_64.iso".to_string()
        );
    }
}
