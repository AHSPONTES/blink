use sysinfo::get_current_pid;
use syslog::{Facility, Formatter3164};

fn get_process_name() -> String {
    let this_process = std::env::current_exe().unwrap();
    let this_file = this_process.file_name().unwrap();

    String::from(this_file.to_str().unwrap())
}

fn logger(message: &str) {
    let this_pid = get_current_pid().unwrap();
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: get_process_name(),
        pid: this_pid,
    };

    match syslog::unix(formatter) {
        Err(e) => {
            println!("Unable to connect to syslog: {:?}", e);
        }
        Ok(mut writer) => {
            writer.err(message).expect("could not write error message");
            println!("{}", message);
        }
    }
}

fn main() {
    logger("This is a log message");
}
