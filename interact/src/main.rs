extern crate curl;

use std::io::{stdout, Write};
use std::io::Read;

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut data = "{\"jsonrpc\":\"2.0\",\"method\":\"web3_clientVersion\",\"params\":[],\"id\":1}".as_bytes();

    let mut easy = Easy::new();
    easy.url("http://localhost:8545").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
}
