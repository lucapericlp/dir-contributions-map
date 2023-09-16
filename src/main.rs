use chrono;

mod rclone;
mod state;
mod collect;

fn main() {
    let user_state_file = "./state_file.json";
    let source = "lp-lucaperic:knowledge-garden/node_modules/";
    // we assume we'll run this, at maximum, once a day on some cron job
    // make more resilient in future with some form of population identity
    // per DateMetadata
    let consideration_window = chrono::Duration::days(1);

    let state_file = state::determine_statefile(user_state_file.to_string());
    let mut entire_state: state::State;
    if state_file.exists() {
        entire_state = state_file.load_state();
    } else {
        state_file.touch().unwrap();
        entire_state = Default::default();
    }

    rclone::defaults::start();
    let rclone_dir = tempfile::tempdir().unwrap()
        .path().to_str().unwrap().to_string();
    rclone::defaults::copy(source, &rclone_dir).unwrap();
    // mutate entire state cos wgaf?
    let day_stats = collect::get_recent_stats(&rclone_dir, consideration_window);
    for (key, value) in day_stats.unwrap() {
        println!("{} has {:?}", key, value)
    }
}
