use dsb_rs::data::objects::RequestObject;
use dsb_rs::data::compression::{compress_and_encode, decode_and_decompress};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let test = RequestObject::new("", "");
    let hi = RequestObject::stringify(test);

    let comp = compress_and_encode(&hi);
    println!("{}", comp);

    let uncomp = decode_and_decompress(&comp);
    println!("{}", uncomp)
}
