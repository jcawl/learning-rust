use chrono;

fn main() {
    //get full utc time
    let utc_full_date = chrono::offset::Utc::now().to_string();
    println!("\n{}\n", utc_full_date);

    //split full date string to isolate data then parse it as an int
    let month = &utc_full_date[5..7].parse::<u32>().unwrap();
    let day = &utc_full_date[8..10].parse::<u32>().unwrap();

    //see if it is summer or winter
    let is_summer = is_daylight_savings(month, day);

    //isolate time data
    let hour = &utc_full_date[11..13].parse::<u32>().unwrap();
    let min = &utc_full_date[14..16];
    let sec = &utc_full_date[17..19];

    //countries who do not use daylight savings
    let mos_hour = get_hour(&hour,23);
    let japan_hour = get_hour(&hour, 29);
    let bangkok_hour = get_hour(&hour,27);
    let beijing_hour = get_hour(&hour, 28);
    let ny_hour;
    let chi_hour;
    let den_hour;
    let la_hour;
    let haw_hour;
    let uk_hour;
    let par_hour;


    if is_summer {
        ny_hour = get_hour(&hour, 16);
        chi_hour = get_hour(&hour, 15);
        den_hour = get_hour(&hour, 14);
        la_hour = get_hour(&hour, 13);
        haw_hour = get_hour(&hour, 10);
        uk_hour = get_hour(&hour, 21);
        par_hour = get_hour(&hour,22);
    } else {
        ny_hour = get_hour(&hour, 15);
        chi_hour = get_hour(&hour, 14);
        den_hour = get_hour(&hour, 13);
        la_hour = get_hour(&hour, 12);
        haw_hour = get_hour(&hour, 9);
        uk_hour = get_hour(&hour, 20);
        par_hour = get_hour(&hour,21);
    }

    //final output
    format_output(day, month, "New York".to_string(), ny_hour, min, sec);
    format_output(day, month, "Chicago".to_string(), chi_hour, min, sec);
    format_output(day, month,"Denver".to_string(), den_hour, min, sec);
    format_output(day, month,"LA".to_string(), la_hour, min, sec);
    format_output(day, month,"Hawaii".to_string(), haw_hour, min, sec);
    format_output(day, month,"UK".to_string(), uk_hour, min, sec);
    format_output(day, month,"Paris".to_string(), par_hour, min, sec);
    format_output(day, month, "Moscow".to_string(), mos_hour, min, sec);
    format_output(day, month,"Bangkok".to_string(), bangkok_hour, min, sec);
    format_output(day, month,"Beijing".to_string(), beijing_hour, min, sec);
    format_output(day, month, "Japan".to_string(), japan_hour, min, sec);
    println!{""};
}

fn format_output(day: &u32, month: &u32, area: String, hour: u32, min: &str, sec: &str) {
    let am_pm = if hour > 11 && hour < 24 { "PM".to_string() } else if hour == 24 { "AM".to_string() } else { "AM".to_string() };
    let mut day_of_month = day.clone();
    let twelve_hour = if hour > 12 && hour < 24  { hour - 12 } else if hour > 24 { day_of_month += 1; hour - 24} else { hour };
    let time_string = format!("({}/{}) - {}: {}:{}:{} {}",  month, day_of_month, area, twelve_hour, min, sec, am_pm);
    println!("{}", time_string);
}

fn is_daylight_savings(month: &u32, day: &u32) -> bool {
    let mon = month.clone();
    let da = day.clone();
    if (mon >= 3) && (mon <= 11) {
        if mon == 3 && da < 14 {
            false
        } else if mon == 11 && da > 7 {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn get_hour(hour: &u32, change: u32) -> u32 {
    //to a avoid equations with a signed and unsigned value
    //subtract change by 20 and proceess value
    if change < 20 {
        hour - (20 - change)
    } else {
        hour + (change - 20)
    }
}
