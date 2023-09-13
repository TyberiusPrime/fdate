use anyhow::{Context, Result};
use chrono::Datelike;
use tuikit::prelude::*;

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

fn main() -> Result<()> {
    let mut title = "".to_string();
    let mut highlights = Vec::new();
    let mut search = None;
    let mut max_results = 5;
    let mut sort_search = false;
    let mut output_filename = None;
    for arg in std::env::args().skip(1) {
        if arg.starts_with("--title=") {
            title = title.strip_prefix("--title=").unwrap().to_string();
            title.push_str(": ");
        } else if arg.starts_with("--highlight=") {
            for adate in arg.strip_prefix("--higlight=").unwrap().split(',') {
                highlights.push(
                    chrono::NaiveDate::parse_from_str(adate, "%Y-%m-%d")
                        .with_context(|| format!("Failed to parsed date '{}'", adate))?,
                );
            }
        } else if arg.starts_with("--search=") {
            search = Some(arg.strip_prefix("--search=").unwrap().to_string());
        } else if arg.starts_with("--sort-search") {
            sort_search = true;
        } else if arg.starts_with("--output_filename=") {
            output_filename = Some(arg.strip_prefix("--output_filename=").unwrap().to_string());
        } else if arg.starts_with("--max-results=") {
            max_results = arg
                .strip_prefix("--max-results=")
                .unwrap()
                .parse::<usize>()
                .with_context(|| format!("Failed to parse max_results '{}'", arg))?;
        }
    }
    let term: Term<()> = Term::with_height(TermHeight::Fixed(10 + max_results)).unwrap();
    let mut cursor_column: usize = 0;

    let mut date: chrono::NaiveDate = chrono::Local::now().naive_local().date();

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();

        let (width, _height) = term.term_size().unwrap();
        match ev {
            Event::Key(Key::Enter) => {
                println!("{}", date.format("%Y-%m-%d"));
                term.clear()?;
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
                cursor_column = 0.max(cursor_column - 1);
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

        let str_date = format!("{}{}", title, date.format("%Y-%m-%d %a KW %V"))
            .trim_start()
            .to_string();
        let centered_date = format!("{:^width$}", str_date, width = used_col);
        let cursor_offset = centered_date.len() / 2 - str_date.len() / 2 + title.len() - 1;
        let _ = term.print(used_row, 0, &centered_date);
        let _ = term.set_cursor(used_row, cursor_column + cursor_offset);

        let used_row = used_row + 1;

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
