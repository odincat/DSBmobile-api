use std::fs::read_to_string;

use dsb_rs::data::parse::UntisParser;

#[tokio::main]
async fn main() {
    let debug_plan_path = "";
    let plan_raw = read_to_string(debug_plan_path).unwrap();

    let result = UntisParser { document: plan_raw }.execute().await;
    println!("{:?}", result.current.week_type);
}
