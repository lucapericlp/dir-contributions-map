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

pub fn list() -> Result<String, String>{
    let method = String::from("operations/list");
    let input = serde_json::json!(
        {
            "fs": "lp-lucaperic:",
            "remote": "knowledge-garden"
        }
    );
    run(&method, &input.to_string())
}

pub fn copy() -> Result<String, String>{
    let method = String::from("sync/copy");
    let tmp_dir = tempfile::tempdir().unwrap(); // idgaf unwrap
    let tmp_dir_path = tmp_dir.path().to_str().unwrap();
    let input = serde_json::json!(
        {
            "srcFs": "lp-lucaperic:knowledge-garden",
            "dstFs": tmp_dir_path,
            "_filter": {
                "IncludeRule": ["**/*.md", "*.md"],
            }
        }
    );
    run(&method, &input.to_string())?;
    println!("Copied successfully into {}!", tmp_dir_path);
    Ok(tmp_dir_path.to_string())
}

#[test]
fn test_lib_init(){
    let result = start();
    assert_eq!(result, true)
}
