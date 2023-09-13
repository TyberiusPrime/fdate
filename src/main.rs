use anyhow::{Context, Result};
use chrono::Datelike;
use tuikit::prelude::*;

const MAX_SEARCH_RESULT_DEFAULT: usize = 5;

pub fn parse_to_arguments_replace(
    cmd: &str,
    replacement: Option<&str>,
    add_if_not_found: bool,
) -> Vec<String> {
    let mut parsed = shell_words::split(cmd).expect("failed to parse search_cmd");
    if let Some(replacement) = replacement {
        let hit = parsed
            .iter()
            .enumerate()
            .find(|(_, x)| x == &"{}")
            .map(|x| x.0);
        match hit {
            Some(x) => {
                parsed[x] = replacement.to_string();
            }
            None => {
                if add_if_not_found {
                    parsed.push(replacement.to_string());
                }
            }
        }
    }
    parsed
}

pub fn print_help() {
    println!("fdate - show an interactive calendar on the console");
    println!("Keyboard input:");
    println!("\t left/right: move one day back/forward");
    println!("\t up/down: move one week back/forward");
    println!("\t page up/page down: move one month back/forward");
    println!("\t home/end: move one year back/forward");
    println!("\t .: goto today");
    println!("\t >: goto tomorrow");
    println!("\t <: goto yesterday");
    println!("\t ,: goto to default date (default default is today)");
    println!(
        "\t m/t/w/h/f/s/u - go to next monday/tuesday/wednesday/thursday/friday/saturday/sunday"
    );
    println!(
        "\t M/T/W/H/F/S/U - go to last monday/tuesday/wednesday/thursday/friday/saturday/sunday"
    );
    println!("\t digits - enter date. No '-' necessary.");
    println!("\t tab - jump two next section of date (so year/month/day)");
    println!("\t backspace - go back one character in entered date");
    println!("\t Enter - leave, print chosen date, exit code 0");
    println!("\t Escape - leave, exit code 1");
    println!("");
    println!("CLI options");
    println!("\t -h | --help - print this help");
    println!("\t YYYY--mm-dd - default / start date");
    println!("\t --title=<whatever> - show this as title (before chosen date)");
    println!("\t --highlight=<iso-date> - Highlight this date (can be passed multiple times)");
    println!("\t --search=<external command> - Whenever the date is changed, call this command with the date as argument. Use '{{}}' as placeholder for the date. The results are shown below the date selection, up to --max-results lines");
    println!("\t --max-results=<number> - Maximum number of lines to show for --search. Default: {MAX_SEARCH_RESULT_DEFAULT}");
    println!("\t --output-filename<filename> - write chosen date to this file as well as outputing it on stdout");
}

fn is_string_iso_date(maybe_a_date: &str) -> bool {
    if maybe_a_date.len() != 10 {
        return false;
    }
    if maybe_a_date.chars().nth(4).unwrap() != '-' || maybe_a_date.chars().nth(7).unwrap() != '-' {
        return false;
    }
    for (i, c) in maybe_a_date.chars().enumerate() {
        if i == 4 || i == 7 {
            continue;
        }
        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

fn main() -> Result<()> {
    let mut title = "".to_string();
    let mut highlights = Vec::new();
    let mut search = None;
    let mut max_results = MAX_SEARCH_RESULT_DEFAULT;
    let mut sort_search = false;
    let mut output_filename = None;
    let mut debug = false;
    let mut start_date: chrono::NaiveDate = chrono::Local::now().naive_local().date();
    for arg in std::env::args().skip(1) {
        if arg == "--help" || arg == "-h" {
            print_help();
            std::process::exit(0);
        } else if arg.starts_with("--title=") {
            title = arg.strip_prefix("--title=").unwrap().to_string();
            title.push_str(": ");
        } else if arg.starts_with("--highlight=") {
            for adate in arg.strip_prefix("--highlight=").unwrap().split(',') {
                highlights.push(
                    chrono::NaiveDate::parse_from_str(adate, "%Y-%m-%d")
                        .with_context(|| format!("Failed to parsed date '{}'", adate))?,
                );
            }
        } else if arg.starts_with("--search=") {
            search = Some(arg.strip_prefix("--search=").unwrap().to_string());
        } else if arg.starts_with("--sort-search") {
            sort_search = true;
        } else if arg.starts_with("--output-filename=") {
            output_filename = Some(arg.strip_prefix("--output-filename=").unwrap().to_string());
        } else if arg.starts_with("--max-results=") {
            max_results = arg
                .strip_prefix("--max-results=")
                .unwrap()
                .parse::<usize>()
                .with_context(|| format!("Failed to parse max_results '{}'", arg))?;
        } else if arg == "--debug" {
            debug = true;
        } else if is_string_iso_date(&arg) {
            start_date = chrono::NaiveDate::parse_from_str(&arg, "%Y-%m-%d")
                .with_context(|| format!("Failed to parsed date '{}'", arg))?;
        } else {
            println!("Unknown argument '{}'", arg);
            std::process::exit(1);
        }
    }
    let term: Term<()> = Term::with_height(TermHeight::Fixed(10 + max_results)).unwrap();
    let mut cursor_column: usize = 0;
    let mut date = start_date;

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();

        let (width, _height) = term.term_size().unwrap();
        match ev {
            Event::Key(Key::Enter) => {
                term.clear()?;
                println!("{}", date.format("%Y-%m-%d"));
                if let Some(output_filename) = &output_filename {
                    std::fs::write(output_filename, date.format("%Y-%m-%d").to_string())?;
                }
                std::process::exit(0);
            }
            Event::Key(Key::Up) => {
                date -= chrono::Duration::days(7);
            }
            Event::Key(Key::Down) => {
                date += chrono::Duration::days(7);
            }
            Event::Key(Key::Left) => {
                date -= chrono::Duration::days(1);
            }
            Event::Key(Key::Right) => {
                date += chrono::Duration::days(1);
            }
            Event::Key(Key::PageUp) => {
                date = date - chrono::Months::new(1);
            }
            Event::Key(Key::PageDown) => {
                date = date + chrono::Months::new(1);
            }
            Event::Key(Key::Home) => {
                date = date + chrono::Months::new(12);
            }
            Event::Key(Key::End) => {
                date = date - chrono::Months::new(12);
            }
            Event::Key(Key::Backspace) => {
                cursor_column = if cursor_column > 0 {
                    cursor_column - 1
                } else {
                    cursor_column
                };
                if cursor_column == 4 || cursor_column == 7 {
                    cursor_column -= 1;
                }
            }
            Event::Key(Key::Tab) => {
                if cursor_column < 4 {
                    cursor_column = 5;
                } else if cursor_column < 7 {
                    cursor_column = 8;
                } else {
                    cursor_column = 0;
                }
            }
            Event::Key(Key::Char('.')) => {
                date = chrono::Local::now().naive_local().date();
            }
            Event::Key(Key::Char(',')) => {
                date = start_date;
            }

            Event::Key(Key::Char('<')) => {
                date = chrono::Local::now().naive_local().date() + chrono::Duration::days(-1);
            }
            Event::Key(Key::Char('>')) => {
                date = chrono::Local::now().naive_local().date() + chrono::Duration::days(1);
            }
            Event::Key(Key::Char('m')) => {
                //advance to next tuesday
                date += chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Mon {
                    date += chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('M')) => {
                //advance to next tuesday
                date -= chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Mon {
                    date -= chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('t')) => {
                //advance to next tuesday
                date += chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Tue {
                    date += chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('T')) => {
                //advance to next tuesday
                date -= chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Tue {
                    date -= chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('w')) => {
                //advance to next tuesday
                date += chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Wed {
                    date += chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('W')) => {
                //advance to next tuesday
                date -= chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Wed {
                    date -= chrono::Duration::days(1);
                }
            }

            Event::Key(Key::Char('h')) => {
                //advance to next tuesday
                date += chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Thu {
                    date += chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('H')) => {
                //advance to next tuesday
                date -= chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Thu {
                    date -= chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('f')) => {
                //advance to next tuesday
                date += chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Fri {
                    date += chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('F')) => {
                //advance to next tuesday
                date -= chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Fri {
                    date -= chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('s')) => {
                //advance to next tuesday
                date += chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Sat {
                    date += chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('S')) => {
                //advance to next tuesday
                date -= chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Sat {
                    date -= chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('u')) => {
                //advance to next tuesday
                date += chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Sun {
                    date += chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('U')) => {
                //advance to next tuesday
                date -= chrono::Duration::days(1);
                while date.weekday() != chrono::Weekday::Sun {
                    date -= chrono::Duration::days(1);
                }
            }
            Event::Key(Key::Char('0')) => {
                add_digit(&mut cursor_column, 0, &mut date);
            }
            Event::Key(Key::Char('1')) => {
                add_digit(&mut cursor_column, 1, &mut date);
            }
            Event::Key(Key::Char('2')) => {
                add_digit(&mut cursor_column, 2, &mut date);
            }
            Event::Key(Key::Char('3')) => {
                add_digit(&mut cursor_column, 3, &mut date);
            }
            Event::Key(Key::Char('4')) => {
                add_digit(&mut cursor_column, 4, &mut date);
            }
            Event::Key(Key::Char('5')) => {
                add_digit(&mut cursor_column, 5, &mut date);
            }
            Event::Key(Key::Char('6')) => {
                add_digit(&mut cursor_column, 6, &mut date);
            }
            Event::Key(Key::Char('7')) => {
                add_digit(&mut cursor_column, 7, &mut date);
            }
            Event::Key(Key::Char('8')) => {
                add_digit(&mut cursor_column, 8, &mut date);
            }
            Event::Key(Key::Char('9')) => {
                add_digit(&mut cursor_column, 9, &mut date);
            }
            //ctrl c
            Event::Key(Key::ESC) | Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                //exit with code 1
                let _ = term.present();
                std::process::exit(1);
            }
            _ => {}
        }

        let mut cal_col = 0;
        let cal_row = 0;
        let (_used_row, used_col) = write_cal(
            date - chrono::Months::new(1),
            date,
            &term,
            cal_col,
            cal_row,
            &highlights,
        );
        cal_col = used_col + 2;

        let (_used_row, used_col) = write_cal(date, date, &term, cal_col, cal_row, &highlights);
        cal_col = used_col + 2;
        let (used_row, used_col) = write_cal(
            date + chrono::Months::new(1),
            date,
            &term,
            cal_col,
            cal_row,
            &highlights,
        );
        let used_row = used_row.max(8);

        let str_date = format!("{}{}", title, date.format("%Y-%m-%d %a WN %V"))
            .trim_start()
            .to_string();
        let centered_date = format!("{:^width$}", str_date, width = used_col);
        let cursor_offset = centered_date.len() / 2 - str_date.len() / 2 + title.len() - 1;
        let _ = term.print(used_row, 0, &centered_date);
        let _ = term.set_cursor(used_row, cursor_column + cursor_offset + 1);

        let used_row = used_row + 1;

        if debug {
            match ev {
                Event::Key(x) => {
                    let _ = term.print(used_row, 0, &format!("Key pressed: {:?}", x));
                }
                _ => {}
            }
        }

        if let Some(search_cmd) = &search {
            let search_result = get_search_results(date, search_cmd, max_results)?;
            let mut lines = search_result.split('\n').collect::<Vec<_>>();
            if sort_search {
                lines.sort();
            }
            for (ii, line) in lines.iter().enumerate() {
                term.print(used_row + ii, 0, &line[..line.len().min(width)])?;
            }
        }
        let _ = term.present();
    }

    fn write_cal(
        date: chrono::NaiveDate,
        date_chosen: chrono::NaiveDate,
        term: &Term<()>,
        start_col: usize,
        start_row: usize,
        highlight: &[chrono::NaiveDate],
    ) -> (usize, usize) {
        let cal = calendarize::calendarize_with_offset(date, 1);
        let header = format!(
            "{} {} {} {} {} {} {}",
            "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
        );
        let year_month = format!("{} {}", date.format("%Y"), date.format("%B"),);
        let year_month = format!("{:^width$}", year_month, width = header.len());
        let attr_underline = Attr {
            effect: Effect::UNDERLINE,
            ..Attr::default()
        };
        let attr_underline_weekend = Attr {
            fg: Color::RED,
            effect: Effect::UNDERLINE,
            ..Attr::default()
        };
        let attr_today = Attr {
            fg: Color::RED,
            effect: Effect::UNDERLINE,
            ..Attr::default()
        };
        let attr_today_chosen = Attr {
            fg: Color::BLUE,
            effect: Effect::UNDERLINE | Effect::BOLD,
            ..Attr::default()
        };
        let attr_today_highlight = Attr {
            fg: Color::CYAN,
            effect: Effect::UNDERLINE | Effect::BOLD,
            ..Attr::default()
        };

        let attr_chosen = Attr {
            effect: Effect::BOLD,
            fg: Color::BLUE,
            ..Attr::default()
        };

        let attr_past = Attr {
            fg: Color::LIGHT_BLACK,
            ..Attr::default()
        };
        let attr_past_highlight = Attr {
            fg: Color::LIGHT_CYAN,
            ..Attr::default()
        };
        let attr_future = Attr { ..Attr::default() };
        let attr_future_highlight = Attr {
            fg: Color::CYAN,
            effect: Effect::BOLD,
            ..Attr::default()
        };
        let attr_weekend = Attr {
            fg: Color::LIGHT_RED,
            //effect: Effect::DIM,
            ..Attr::default()
        };

        term.print(start_row, start_col, &year_month).unwrap();
        term.print(start_row + 1, start_col, &header).unwrap();
        term.print_with_attr(
            start_row + 1,
            start_col + header.len() - 7,
            &header[header.len() - 7..],
            attr_weekend,
        )
        .unwrap();

        term.print_with_attr(start_row + 1, start_col, "M", attr_underline)
            .unwrap();
        term.print_with_attr(start_row + 1, start_col + 4, "T", attr_underline)
            .unwrap();
        term.print_with_attr(start_row + 1, start_col + 8, "W", attr_underline)
            .unwrap();
        term.print_with_attr(start_row + 1, start_col + 13, "h", attr_underline)
            .unwrap();
        term.print_with_attr(start_row + 1, start_col + 16, "F", attr_underline)
            .unwrap();
        term.print_with_attr(start_row + 1, start_col + 20, "S", attr_underline_weekend)
            .unwrap();
        term.print_with_attr(start_row + 1, start_col + 25, "u", attr_underline_weekend)
            .unwrap();

        let today = chrono::Local::now().naive_local().date();

        let mut max_col = start_col;
        for (row, days) in cal.iter().enumerate() {
            let mut col = start_col;
            for ii in days {
                if *ii > 0 {
                    let mod_date =
                        chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), *ii).unwrap();
                    let attr = if mod_date == date_chosen {
                        if today == date {
                            attr_today_chosen
                        } else {
                            attr_chosen
                        }
                    } else if mod_date == today {
                        if highlight.contains(&mod_date) {
                            attr_today_highlight
                        } else {
                            attr_today
                        }
                    } else if mod_date < today {
                        if highlight.contains(&mod_date) {
                            attr_past_highlight
                        } else {
                            attr_past
                        }
                    } else if highlight.contains(&mod_date) {
                        attr_future_highlight
                    } else {
                        attr_future
                    };
                    term.print_with_attr(start_row + 2 + row, col, &format!("{:>2}", ii), attr)
                        .unwrap();
                }
                col += 4;
                max_col = max_col.max(col)
            }
        }
        (start_row + 2 + cal.len(), max_col)
    }

    fn add_digit(col: &mut usize, digit: u8, date: &mut chrono::NaiveDate) {
        let str_date = format!("{}", date.format("%Y-%m-%d"));
        //replace char at col with digit
        let mut chars: Vec<char> = str_date.chars().collect();
        chars[*col] = std::char::from_digit(digit as u32, 10).unwrap();
        let new_str_date: String = chars.into_iter().collect();
        let mut new_date = chrono::NaiveDate::parse_from_str(&new_str_date, "%Y-%m-%d");

        if new_date.is_err() && ((*col == 5) || (*col == 8)) {
            let mut chars: Vec<char> = str_date.chars().collect();
            chars[*col] = std::char::from_digit(digit as u32, 10).unwrap();
            chars[*col + 1] = if digit == 0 { '1' } else { '0' };
            let new_str_date: String = chars.into_iter().collect();
            new_date = chrono::NaiveDate::parse_from_str(&new_str_date, "%Y-%m-%d");
        }

        if let Ok(new_date) = new_date {
            *date = new_date;
            *col += 1;
            if (*col == 4) || (*col == 7) {
                *col += 1;
            } else if *col == 4 + 3 + 3 {
                *col = 0;
            }
        }
    }
    Ok(())
}

fn get_search_results(
    date: chrono::NaiveDate,
    search_cmd: &str,
    max_lines: usize,
) -> Result<String> {
    let search_parsed = parse_to_arguments_replace(
        search_cmd,
        Some(&format!("{}", date.format("%Y-%m-%d"))),
        true,
    );
    let output = std::process::Command::new(&search_parsed[0])
        .args(&search_parsed[1..])
        .output()
        .context("search cmd failed")?;
    let results = String::from_utf8(output.stdout).context("invalid utf8")?;
    //cut results to max_lines
    let results = results
        .lines()
        .take(max_lines)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    Ok(results)
}
