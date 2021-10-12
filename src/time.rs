use chrono_english::{parse_date_string,Dialect};
use chrono::prelude::*;

byond_fn! { seconds_until(time) {
    let date_time = parse_date_string(time, Local::now(), Dialect::Us);
    let start_time = Local::now();
    let duration = date_time.unwrap() - start_time;
    Some(duration.num_seconds().to_string())
} }