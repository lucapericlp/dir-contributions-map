mod rclone;
mod state;

fn main() {
    let state_file = state::StateFile::Local("./state_file.json".to_string());
    if state_file.exists() {
        let state = state_file.build_state();
    } else {
        let state: state::State = Default::default();
    }

    let source = "lp-lucaperic:knowledge-garden";
    rclone::defaults::start();
    let copied_source_dir = rclone::defaults::copy(source, None).unwrap();
}
