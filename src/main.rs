use std::io;
use chrono::{DateTime, offset::{FixedOffset}};

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut sum: i64 = 0;
    const HOURLY_PAY: i64 = /* secret value */;

    for result in rdr.records() {
        let record = result.unwrap();
        let start_time: DateTime<FixedOffset>
            = DateTime::parse_from_rfc3339(&record[0]).unwrap();
        let end_time: DateTime<FixedOffset>
            = DateTime::parse_from_rfc3339(&record[1]).unwrap();
        let duration = end_time - start_time;
        sum += duration.num_seconds();
    }

    println!("Total time: {}:{}:{}",
             (sum / 60) / 60,
             (sum / 60) % 60,
             sum % 60); 
    println!("Your salary: {} yen",
             sum * HOURLY_PAY / 3600);
}

