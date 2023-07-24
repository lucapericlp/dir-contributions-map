mod rclone;

fn main() {
    rclone::defaults::start();
    let _ = rclone::defaults::copy().unwrap();
    println!("Completed!")
}
