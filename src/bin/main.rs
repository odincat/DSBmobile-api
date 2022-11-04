use dsb_rs::RequestObject;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let test = RequestObject::new("", "");
    let hi = RequestObject::stringify(test);
    println!("{}", hi)
}
