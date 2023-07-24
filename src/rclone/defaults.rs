use librclone;

pub fn start() -> bool{
    librclone::initialize();
    true
}

#[test]
fn test_lib_init(){
    let result = start();
    assert_eq!(result, true)
}
