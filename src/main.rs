mod rclone;
// use serde_json;

fn main() {
    rclone::defaults::start();
    let method = String::from("operations/list");
    let input = String::from(r#"
        {
            "fs": "lp-lucaperic:",
            "remote": "knowledge-garden"
        }"#
    );
    let result = match rclone::defaults::run(method, input) {
        Ok(str) => str,
        Err(_) => panic!("Help!"),
    };
    println!("{}", result);
}
