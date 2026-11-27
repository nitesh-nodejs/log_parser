use std::fs;

#[derive(Debug)]
struct Urls {
    url: String,
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
    return Some(logs);
}

fn log_parser() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let urls = Urls {
        url: String::from("E:\\rust_project\\log_parser\\logs.txt"),
    };
    if let Some(content) = file_reader(urls) {
        for (i, line) in content.into_iter().enumerate() {
            let parts: Vec<&str> = line.splitn(4, " ").collect();
            match parts.as_slice() {
                [date, time, key, msg, ..] => {
                    if *key == "Error" {
                        result.push(format!("{}: -m {}, On {}&{}; Ln: {}\n", key, msg, date, time, i));
                    };
                },
                _ => continue,
            }
        }
    }
    return result;
}
fn main() {
    let a = log_parser();
    println!("{:?}", a);
}
