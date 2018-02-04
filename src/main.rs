
use std::io::{BufWriter, BufReader};
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() {

    let filename = "/usr/bin/skypeforlinux";
    let filename_test = "/mnt/dev/test.txt";
    let founded = "nohup \"$SKYPE_PATH\" --executed-from=\"$(pwd)\" --pid=$$ \"$@\" > \"$SKYPE_LOGS/skype-startup.log\" 2>&1 &";
    let replaced: String = "PULSE_LATENCY_MSEC=100 nohup \"$SKYPE_PATH\" --executed-from=\"$(pwd)\" --pid=$$ \"$@\" > \"$SKYPE_LOGS/skype-startup.log\" 2>&1 &".to_string();
    let new_line: String = "\n".to_string();

    let mut was_found:bool = false;
    let mut vec: Vec<String> = Vec::new();

    {
        let mut options = OpenOptions::new();
        let file_source = options.read(true).open(filename).expect("Cannot open this file!");
        let mut reader = BufReader::new(file_source);

        for line in reader.lines() {
            let mut buffer: String = String::new();
            buffer = line.expect("Cannot read line from buffer!");
            println!("{}", buffer);
            vec.push(buffer);
        }
    }

    println!("Lines readed from file: {}", vec.len());

    {
        let mut options = OpenOptions::new();
        let file_dest = options.write(true).open(filename).expect("Cannot write to this file!");
        let mut writer = BufWriter::new(file_dest);

        for item in vec {
            if item.eq(&founded) {
                writer.write(replaced.as_bytes());
                writer.write(new_line.as_bytes());
                was_found = true;
            } else {
                writer.write(item.as_bytes());
                writer.write(new_line.as_bytes());
            }
        }
    }

    if was_found {
        println!("File was written successfully!");
    } else {
        println!("File has correct data");
    }
}
