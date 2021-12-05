use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::{LogPacker};
use std::thread::sleep;
use std::time::Duration;

fn main(){
    fast_log::init_split_log(
        "target/logs/",
        LogSize::MB(1),
        RollingType::All,
        log::Level::Info,
        None,
        Box::new(LogPacker{}),
        true,
    );
    for _ in 0..20000 {
        log::info!("Commencing yak shaving");
    }
    may::coroutine::sleep(Duration::from_secs(3));
    println!("you can see log files in path: {}","target/logs/")
}