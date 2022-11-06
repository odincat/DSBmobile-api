use std::io::prelude::*;
use flate2::write::{GzEncoder, GzDecoder};
use flate2::Compression;

pub fn compress_and_encode (input: &str) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes()).unwrap();
    let compressed_bytes = encoder.finish().unwrap();

    return base64::encode(compressed_bytes);
}

pub fn decode_and_decompress (input: &str) -> String {
    let mut decoder = GzDecoder::new(Vec::new());
    decoder.write_all(&base64::decode(input).unwrap()).unwrap();
    let decompressed_bytes = decoder.finish().unwrap();

    return String::from_utf8(decompressed_bytes).unwrap();
}
