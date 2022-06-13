use std::env;
use std::error::Error;
use work_time_counter::{validate_input, parse_args_to_time, count_work_time};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if !validate_input(args.clone()) {
        println!("[Error] Invalid input parameters!");
        return Ok(());
    }

    let time_list = parse_args_to_time(args);

    if time_list.is_none() {
        println!("[Error] NativeTime parsing error -> check if you passing arguments in \"hh:mm\" format!");
        return Ok(());
    }

    let duration = count_work_time(time_list.unwrap());
    let minutes = duration.num_minutes() - duration.num_hours() * 60;
    println!("Time at work: {}:{:0>2}", duration.num_hours(), minutes);
    Ok(())
}

