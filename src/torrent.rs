use bytes::Bytes;
use serde::{self, Deserialize, Serialize};
use serde_bencode;
use std::error::Error;
use tokio::fs;

// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
#[derive(Debug, Deserialize)]
pub struct TorrentFile {
    #[serde(rename = "info")]
    pub info: Info,
    #[serde(rename = "announce")]
    pub announce: String,
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
    #[serde(rename = "encoding")]
    pub encoding: Option<String>,
}

impl TorrentFile {
    pub async fn from_file(path: String) -> Result<TorrentFile, Box<dyn Error>> {
        let torrent_file = fs::read(path).await?;
        Result::Ok(serde_bencode::from_bytes(torrent_file.as_slice())?)
    }

    pub fn info_hash(&self) -> Result<Bytes, Box<dyn Error>> {
        use ring::digest;

        let info = serde_bencode::to_bytes(&self.info)?;
        let info = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, &info);
        Result::Ok(Bytes::copy_from_slice(info.as_ref()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Info {
    Single(InfoSingle),
    Multiple(InfoMultiple),
}

impl Info {
    pub fn piece_length(&self) -> usize {
        match self {
            Info::Single(i) => i.piece_length,
            Info::Multiple(i) => i.piece_length,
        }
    }

    pub fn piece_count(&self) -> usize {
        match self {
            Info::Single(i) => i.pieces.len() / 20,
            Info::Multiple(i) => i.pieces.len() / 20,
        }
    }

    pub fn total_length(&self) -> usize {
        match self {
            Info::Single(i) => i.length,
            Info::Multiple(i) =>  i.files.iter().fold(0, |acc, f| acc + f.length),
        }
    }
}

// order of field is important for bencode
#[derive(Debug, Deserialize, Serialize)]
pub struct InfoSingle {
    #[serde(rename = "piece length")]
    piece_length: usize,
    #[serde(rename = "pieces")]
    pieces: Bytes,
    #[serde(rename = "private")]
    private: Option<u8>,
    #[serde(rename = "length")]
    length: usize,
    #[serde(rename = "name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoMultiple {
    #[serde(rename = "piece length")]
    piece_length: usize,
    #[serde(rename = "pieces")]
    pieces: Bytes,
    #[serde(rename = "private")]
    private: Option<u8>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "files")]
    files: Vec<InfoFile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoFile {
    #[serde(rename = "length")]
    length: usize,
    #[serde(rename = "path")]
    path: Vec<String>,
}
