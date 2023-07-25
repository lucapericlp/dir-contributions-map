mod rclone;
mod state;
mod collect;

fn main() {
    let user_state_file = "./state_file.json";
    let source = "lp-lucaperic:knowledge-garden";

    let state_file = state::build_statefile(user_state_file.to_string());
    if state_file.exists() {
        let state = state_file.load_state();
    } else {
        state_file.touch();
        let state: state::State = Default::default();
    }

    rclone::defaults::start();
    let copied_source_dir = rclone::defaults::copy(source, None).unwrap();
    let day_stats = collect::get_recent_stats(&copied_source_dir);
}
