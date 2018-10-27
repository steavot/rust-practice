// I want to find the structure that contains the
// latest timestring.  The structure may contain None.
extern crate chrono;
use chrono::DateTime;
#[derive(Clone)]
struct TimeString {
    timestring: Option<String>,
}

impl TimeString {
    fn new(maybe_time: Option<String>) -> TimeString {
        TimeString { timestring: maybe_time }
    }
}

fn later_timestring(a: String, b: String) -> String {
    let form = "%Y-%m-%dT%H:%M:%S.%.3f+%z";
    let time_a = DateTime::parse_from_str(&a, form);
    let time_b = DateTime::parse_from_str(&b, form);
    if time_a > time_b {
        a
    } else {
        b
    }
}

fn main() {
    let mut t = Vec::new();
    t.push(TimeString::new(None));
    t.push(TimeString::new(None));
    t.push(TimeString::new(Some("2015-12-24T16:51:14.000+01:00".to_string())));
    t.push(TimeString::new(Some("2015-12-24T16:52:14.000+01:00".to_string())));
    t.push(TimeString::new(Some("2015-12-23T16:51:14.000+01:00".to_string())));

    let latest_time: Option<String> = t.iter().cloned()
        .fold(None, |acc, x| match (acc, x.timestring) {
            (anything, None) => anything,
            (None, Some(thing)) => Some(thing.to_string()),
            //(Some(a), Some(b)) => Some(later_timestring(a, b))
            (Some(_), Some(b)) => Some(b.to_string()),
        }
    );

    println!("Latest Time: {}", latest_time.unwrap());
}
