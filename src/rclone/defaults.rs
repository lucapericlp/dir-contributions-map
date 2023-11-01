use librclone;
use serde_json;
use tempfile;

pub fn start() -> bool{
    librclone::initialize();
    println!("rclone initialized successfully!");
    true
}

pub fn run(method: &str, input: &str) -> Result<String, String>{
    librclone::rpc(method, input)
}

pub fn copy(src: &str, dst: &str) -> Result<String, String>{
    let method = String::from("sync/copy");
    let input = serde_json::json!(
        {
            "srcFs": src,
            "dstFs": dst,
            "_filter": {
                "IncludeRule": ["**/*.md", "*.md"],
            }
        }
    );
    run(&method, &input.to_string())?;
    println!("Copied successfully into {}!", dst);
    Ok(dst.to_string())
}

#[test]
fn test_lib_init(){
    let result = start();
    assert_eq!(result, true)
}
