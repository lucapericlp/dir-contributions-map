use librclone;

pub fn start() -> bool{
    librclone::initialize();
    true
}

pub fn run(method: String, input: String) -> Result<String, String>{
    librclone::rpc(method, input)
}

#[test]
fn test_lib_init(){
    let result = start();
    assert_eq!(result, true)
}
