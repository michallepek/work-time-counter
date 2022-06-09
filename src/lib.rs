use std::ops::Add;
use std::str::FromStr;
use chrono::naive::NaiveTime;
use chrono::{Duration, ParseResult};

pub fn validate_input(args: Vec<String>) -> bool {
    let len = args.len();
    let has_more_than_two_args = len >= 2;
    let args_number_is_even = len % 2 == 0;
    has_more_than_two_args && args_number_is_even
}

pub fn parse_args_to_time(args: Vec<String>) -> Option<Vec<NaiveTime>> {
    let mut time_list: Vec<NaiveTime> = Vec::new();

    for time_str in args.iter() {
        let parse_res: ParseResult<NaiveTime> = NaiveTime::from_str(format!("{}:00", time_str).as_str());

        if parse_res.is_err() {
            return None;
        }

        time_list.push(parse_res.unwrap());
    }

    return Some(time_list);
}

pub fn count_work_time(time_list: Vec<NaiveTime>) -> Duration {
    let mut result: Duration = Duration::seconds(0);
    let mut diffs_to_sum: Vec<Duration> = Vec::new();
    let mut first_element: NaiveTime = NaiveTime::from_hms(0, 0, 0);
    let mut is_pair = false;

    // Collect time diffs
    for next_element in time_list.iter() {
        if !is_pair {
            first_element = next_element.to_owned();
        } else {
            diffs_to_sum.push(*next_element - first_element.to_owned());
        }

        is_pair = !is_pair;
    }

    for next_diff in diffs_to_sum.iter() {
        result = result.add(next_diff.to_owned());
    }

    result
}
