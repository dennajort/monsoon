mod tracker;
mod torrent;

use std::error::Error;
use clap::Clap;

#[derive(Clap)]
#[clap()]
struct Opts {
    torrent_path: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let opts = Opts::parse();
    let torrent_file = torrent::TorrentFile::from_file(opts.torrent_path).await?;
    let peer_id = generate_peer_id();

    let resp = tracker::query(peer_id, &torrent_file).await?;
    
    println!("{:?}", resp);
    Result::Ok(())
}

fn generate_peer_id() -> String {
    format!("-MS0000-{:012}", rand::random::<u64>() % 1000000000000)
}