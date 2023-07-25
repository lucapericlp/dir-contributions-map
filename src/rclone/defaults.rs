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

fn tmp_dir() -> String {
    let tmp_dir = tempfile::tempdir().unwrap();
    tmp_dir.path().to_str().unwrap().to_string()
}

pub fn copy(src: &str, dst: Option<&str>) -> Result<String, String>{
    let method = String::from("sync/copy");
    let tmp = tmp_dir();
    let dst_path = dst.unwrap_or(&tmp);
    let input = serde_json::json!(
        {
            "srcFs": src,
            "dstFs": dst_path,
            "_filter": {
                "IncludeRule": ["**/*.md", "*.md"],
            }
        }
    );
    run(&method, &input.to_string())?;
    println!("Copied successfully into {}!", dst_path);
    Ok(dst_path.to_string())
}

#[test]
fn test_lib_init(){
    let result = start();
    assert_eq!(result, true)
}
