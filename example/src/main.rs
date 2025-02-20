use fast_log::config::Config;

fn main() {
    fast_log::init(Config::new().console()).unwrap();
    log::info!("Commencing yak shaving{}", 0);
    fast_log::print("Commencing print\n".into()).expect("fast log not init");
    log::logger().flush();
}
