use std::{
    cell::RefCell,
    collections::hash_map::{Entry, HashMap},
    time::Instant,
};
use chrono_english::{parse_date_string,Dialect};
use chrono::prelude::*;

thread_local!( static INSTANTS: RefCell<HashMap<String, Instant>> = RefCell::new(HashMap::new()) );

byond_fn!(fn time_microseconds(instant_id) {
    INSTANTS.with(|instants| {
        let mut map = instants.borrow_mut();
        let instant = match map.entry(instant_id.into()) {
            Entry::Occupied(elem) => elem.into_mut(),
            Entry::Vacant(elem) => elem.insert(Instant::now()),
        };
        Some(instant.elapsed().as_micros().to_string())
    })
});

byond_fn!(fn time_milliseconds(instant_id) {
    INSTANTS.with(|instants| {
        let mut map = instants.borrow_mut();
        let instant = match map.entry(instant_id.into()) {
            Entry::Occupied(elem) => elem.into_mut(),
            Entry::Vacant(elem) => elem.insert(Instant::now()),
        };
        Some(instant.elapsed().as_millis().to_string())
    })
});

byond_fn!(fn time_reset(instant_id) {
    INSTANTS.with(|instants| {
        let mut map = instants.borrow_mut();
        map.insert(instant_id.into(), Instant::now());
        Some("")
    })
});

byond_fn!(
    fn unix_timestamp() {
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64()
                .to_string(),
        )
    }
);

byond_fn! { seconds_until(time) {
    let date_time = parse_date_string(time, Local::now(), Dialect::Us);
    let start_time = Local::now();
    let duration = date_time.unwrap() - start_time;
    Some(duration.num_seconds().to_string())
} }