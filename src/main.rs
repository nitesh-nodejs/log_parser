use std::fs;

#[derive(Debug)]
struct Urls {
    url: String,
}
#[derive(Debug)]
struct LogEntry<'a> {
    level: &'a str,
    user_id: &'a str,
    details: &'a str,
    date: &'a str,
    time: &'a str,
    line: &'a str,
}

fn log_formatter<'a>(logs: &'a str) -> Option<LogEntry<'a>> {
    // let log  = String::from("Error: -m User 1029 payment declined on /products/laptop, -D 2026-11-17&10:31:54 -Ln 5");

    let (level, rest) = logs.split_once(" -m User ")?;
    let (user_id, rest) = rest.split_once(' ')?;
    let (details, rest) = rest.split_once(" -D ")?;
    let (date, rest) = rest.split_once('&')?;
    let (time, line) = rest.split_once("-Ln ")?;

    Some(LogEntry {
        level,
        user_id,
        details,
        date,
        time,
        line,
    })
}

fn file_reader(path: Urls) -> Option<Vec<String>> {
    let contents = fs::read_to_string(path.url);

    let mut logs: Vec<String> = Vec::new();
    match contents {
        Ok(bytes) => {
            let content = bytes.as_bytes();

            // println!("bytes ==> {:?}", bytes);
            let mut start_from: usize = 0;
            for (i, &byte) in content.iter().enumerate() {
                if byte == b'\n' {
                    let upto: usize = i;
                    // println!("start_from, upto {:?}: {:?}\n", start_from, upto);

                    let slices = &bytes[start_from..upto].trim();
                    logs.push(slices.to_string());

                    start_from = upto + 1;
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    Some(logs)
}

fn log_parser() {
    // let mut result: Vec<String> = Vec::new();
    let urls = Urls {
        url: String::from("E:\\rust_project\\log_parser\\logs.txt"),
    };
    if let Some(content) = file_reader(urls) {
        for (i, line) in content.into_iter().enumerate() {
            let parts: Vec<&str> = line.splitn(4, " ").collect();
            match parts.as_slice() {
                [date, time, key, msg, ..] => {
                    if *key == "Error"
                        && let Some(entry) = log_formatter(&format!(
                            "{}: -m {}, -D {}&{}; -Ln {}\n",
                            key, msg, date, time, i
                        ))
                    {
                        let timestamp = format!("{} {}", entry.date, entry.time);
                        let level = entry.level.to_uppercase();

                        // Capitalize first letter of details
                        let mut chars = entry.details.chars();
                        let details = match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        };

                        let border = "+---------------------+-------+---------+--------------------------------------+------+";

                        println!("{}", border);
                        println!(
                            "| {:<19} | {:<5} | {:<7} | {:<36} | {:<4} |",
                            "Timestamp", "Level", "User ID", "Details", "Line"
                        );
                        println!("{}", border);
                        println!(
                            "| {:<19} | {:<5} | {:<7} | {:<36} | {:<4} |",
                            timestamp, level, entry.user_id, details, entry.line
                        );
                        println!("{}", border);
                    }
                }
                _ => continue,
            }
        }
    }
    // return result;
}
fn main() {
    log_parser();
}
