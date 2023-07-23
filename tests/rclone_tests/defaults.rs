use dircontribmap;

fn test_lib_init(){
    let result = dircontribmap::rclone::defaults::start();
    assert_eq!(result, true)
}
