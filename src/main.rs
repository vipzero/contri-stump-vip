use chrono::{DateTime, Duration, Local, TimeZone};

fn main() {
    let vertical_shift = 0;
    let year_dir = "2021";
    let start_date = "2021/01/31 12:00:00";
    //     let message = r"
    // V V III PPP
    // V V  I  P P
    // V V  I  PPP
    // V V  I  P
    //  V  III P
    // "
    let message = r"
_X       X 
X X     X X
            
   W   W   
  W  W  W  
   WW WW   
"
    .trim();

    let git_format = "%c %z";
    let mut dayps: Vec<i64> = Vec::new();
    let start_day: DateTime<Local> = Local
        .datetime_from_str(start_date, "%Y/%m/%d %H:%M:%S")
        .unwrap();

    for (wi, line) in message.split("\n").enumerate() {
        for (di, _c) in line
            .chars()
            .enumerate()
            .filter(|(_di, c)| *c != ' ' && *c != '_')
        {
            // println!("{} {}", di, c);
            dayps.push((di * 7 + wi + vertical_shift) as i64)
        }
    }
    dayps.sort();
    println!("mkdir -p {}", year_dir);
    for p in dayps {
        let d = start_day + Duration::days(p);
        println!("touch {}/{}", year_dir, d.format("%F"));
        println!("git add --all");
        println!(
            "git commit --date=\"{}\" -m \"yes\"",
            (d).format(git_format)
        );
    }
}
