mod rclone;
mod state;
mod collect;

fn main() {
    let user_state_file = "./state_file.json";
    let source = "lp-lucaperic:knowledge-garden/node_modules/";

    let state_file = state::determine_statefile(user_state_file.to_string());
    if state_file.exists() {
        let state = state_file.load_state();
    } else {
        state_file.touch().unwrap();
        let state: state::State = Default::default();
    }

    rclone::defaults::start();
    let tmp_dir = tempfile::tempdir().unwrap()
        .path().to_str().unwrap().to_string();
    rclone::defaults::copy(source, &tmp_dir).unwrap();
    let day_stats = collect::get_recent_stats(&tmp_dir);
}
