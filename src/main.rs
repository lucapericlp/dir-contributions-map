mod rclone;

fn main() {
    rclone::defaults::start();
    let tmp_dir_path = rclone::defaults::copy().unwrap();
}
