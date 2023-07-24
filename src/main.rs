mod rclone;

fn main() {
    rclone::defaults::start();
    let result = rclone::defaults::list().unwrap();
    // let result = rclone::defaults::copy().unwrap();
    println!("{}", result);
}
