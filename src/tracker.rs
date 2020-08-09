use super::torrent;

use bytes::Bytes;
use serde::{self, Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize)]
struct Query {
    #[serde(rename = "info_hash")]
    info_hash: String,
    #[serde(rename = "peer_id")]
    peer_id: String,
    #[serde(rename = "uploaded")]
    uploaded: usize,
    #[serde(rename = "downloaded")]
    downloaded: usize,
    #[serde(rename = "left")]
    left: usize,
    #[serde(rename = "event")]
    event: Event,
    #[serde(rename = "port")]
    port: i64,
}

#[derive(Debug, Serialize)]
enum Event {
    #[serde(rename="started")]
    Started,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Failure(Failure),
    Success(Success),
}

#[derive(Debug, Deserialize)]
pub struct Failure {
    #[serde(rename = "failure reason")]
    reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Success {
    #[serde(rename = "warning message")]
    warning: Option<String>,
    #[serde(rename = "interval")]
    interval: i64,
    #[serde(rename = "min interval")]
    min_interval: Option<i64>,
    #[serde(rename = "tracker id")]
    tracker_id: Option<String>,
    #[serde(rename = "complete")]
    complete: Option<i64>,
    #[serde(rename = "incomplete")]
    incomplete: Option<i64>,
    #[serde(rename = "peers")]
    peers: Peers,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Peers {
    Classic(Vec<Peer>),
    Compact(Bytes),
}

#[derive(Debug, Deserialize)]
pub struct Peer {
    #[serde(rename = "peer id")]
    peer_id: Bytes,
    #[serde(rename = "ip")]
    ip: String,
    #[serde(rename = "port")]
    port: i64,
}

pub async fn query(peer_id: String, torrent_file: &torrent::TorrentFile) -> Result<Response, Box<dyn Error>> {
    let info_hash = torrent_file.info_hash()?;
    
    println!("info_hash {}", hex::encode(info_hash.clone()));
    let params = Query {
        info_hash: unsafe { String::from_utf8_unchecked(info_hash.to_vec()) },
        peer_id: peer_id,
        uploaded: 0,
        downloaded: 0,
        left: torrent_file.info.total_length(),
        event: Event::Started,
        port: 6888,
    };
    let url = format!(
        "{}?{}",
        torrent_file.announce,
        serde_qs::to_string(&params)?
    );
    println!("{}", url);
    let resp = reqwest::get(&url).await?;
    let resp = resp.bytes().await?;
    let resp: Response = serde_bencode::from_bytes(resp.as_ref())?;
    Result::Ok(resp)
}
