use std::io;

fn main() {
    loop {
        let mut date = String::new();
        io::stdin().read_line(&mut date)
            .expect("Failed to read input.");

        let mut date_iter = date.split_whitespace();
        let year: u16 = date_iter.next().unwrap().parse::<u16>().unwrap();
        let month: u16 = date_iter.next().unwrap().parse::<u16>().unwrap();
        let day_num: u16 = date_iter.next().unwrap().parse::<u16>().unwrap();

        println!("{}", get_day(year, month, day_num));
    }
}

/// ```fn get_day(year: u16, month: u16, day: u16) -> String```
///
/// Returns the day of the week as a String, when a date is provided
/// Example: get_day(1996, 10, 21) == "Monday"
fn get_day(year: u16, month: u16, day: u16) -> String {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let days_in_week = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let mut processed_year = year;
    if month < 3 {
        processed_year -= 1;
    }
    let day_in_week: u16 = (processed_year + processed_year/4 - processed_year/100 + processed_year/400 + t[(month-1) as usize] + day) % 7;
    String::from(days_in_week[day_in_week as usize])
}
