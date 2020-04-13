use chrono::{DateTime, Duration, Local, TimeZone};

fn main() {
    let message = r" 
V V III PPP
V V  I  P P
V V  I  PPP
V V  I  P  
 V  III P  
"
    .trim();

    let git_format = "%c %z";
    let mut dayps: Vec<i64> = Vec::new();
    let start_day: DateTime<Local> = Local
        .datetime_from_str("2020/04/12 12:00:00", "%Y/%m/%d %H:%M:%S")
        .unwrap();

    for (wi, line) in message.split("\n").enumerate() {
        for (di, _c) in line.chars().enumerate().filter(|(_di, c)| *c != ' ') {
            // println!("{} {}", di, c);
            dayps.push((di * 7 + wi + 1) as i64)
        }
    }
    dayps.sort();
    for p in dayps {
        let d = start_day + Duration::days(p);
        println!("touch {}", d.format("%F"));
        println!("git add --all");
        println!(
            "git commit --date=\"{}\" -m \"yes\"",
            (d).format(git_format)
        );
    }
}
