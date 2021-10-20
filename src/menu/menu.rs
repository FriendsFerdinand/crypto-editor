pub mod menu {
    use std::io::{self, Write};
    use termion::clear;
    use termion::color;
    use termion::input::TermRead;

    use super::super::super::database::database_handler::*;
    use super::super::super::database::keychain::*;
    use super::super::super::CryptoEditor;

    use super::super::super::utils::list_tools;

    use chrono::Local;

    pub fn read_input() -> Option<String> {
        let mut input = String::new();

        if let Ok(_) = io::stdin().read_line(&mut input) {
            println!("{}", clear::All);
            print!("{}", termion::cursor::Goto(1, 1));
            return Some(input[0..input.len() - 1].to_string());
        }

        None
    }

    pub fn read_password() -> Option<String> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        stdout.write_all(b"Password: ").unwrap();
        stdout.flush().unwrap();

        let pass = stdin.read_passwd(&mut stdout).unwrap();
        println!();
        pass
    }

    pub fn read_with_prompt(prompt: &str) -> Option<String> {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        read_input()
    }

    pub fn run() {
        let mut password: String = String::new();
        let mut input: String = String::new();

        println!("{}", clear::All);
        print!("{}", termion::cursor::Goto(1, 1));

        println!("{}Welcome. Choose an option.", color::Fg(color::Green));
        print!("{}", color::Fg(color::Reset));
        loop {
            print!("{}", color::Fg(color::Blue));
            println!("{}) {}", '1', "Access logs");
            println!("{}) {}", '2', "Create user");
            println!("{}) {}", '3', "Quit");
            print!("{}", color::Fg(color::Reset));

            match read_input() {
                Some(c) => input = c,
                None => println!("Didn't work!"),
            }
            if input == "1" {
                if let Some(ids) = key_chain::get_key_ids() {
                    print!("{}", color::Fg(color::Green));
                    println!("Following users are active: ");
                    print!("{}", color::Fg(color::Reset));
                    display_options(&ids);
                    match read_input() {
                        Some(c) => input = c,
                        None => println!("Didn't work!"),
                    }
                    match process_option(input.clone(), ids) {
                        Some(chosen_id) => {
                            access_logs(chosen_id.1.as_ref());
                            break;
                        }
                        None => {
                            println!("Invalid input. Choose one of the options.");
                        }
                    }
                }
                println!("There are no users.");
            } else if input == "2" {
                print!("{}", color::Fg(color::Green));
                println!("Creating a new user");
                print!("{}", color::Fg(color::Reset));
                match read_with_prompt("Username") {
                    Some(c) => input = c,
                    None => println!("Didn't work!"),
                }

                match read_password() {
                    Some(c) => password = c,
                    None => println!("Didn't work!"),
                }

                print!("{}", color::Fg(color::Green));
                println!("Insert password again.");
                print!("{}", color::Fg(color::Reset));
                let mut temp_pass = String::new();
                match read_password() {
                    Some(c) => temp_pass = c,
                    None => println!("Didn't work!"),
                }

                if temp_pass.contains(&password) {
                    println!("Creating user...");
                    match key_chain::create_user(&input, &password) {
                        Ok(_) => {
                            println!("{}", clear::All);
                            print!("{}", termion::cursor::Goto(1, 1));
                            access_logs(&input)
                        }
                        Err(_) => println!("Error creating user!"),
                    }
                }
            } else {
                println!("Goodbye!");
                break;
            }
        }

        print!("{}", color::Fg(color::Reset));
    }

    fn process_option(option: String, options: Vec<String>) -> Option<(usize, String)> {
        if let Ok(idx) = option.parse::<u32>() {
            if idx <= options.len() as u32 && idx != 0 {
                return Some(((idx as usize) - 1, options[(idx as usize) - 1].clone()));
            }
            return None;
        }
        None
    }

    fn access_logs(id: &str) {
        let mut input: String = String::new();
        let now = Local::now().format("%d_%m_%Y");

        print!("{}", color::Fg(color::Green));
        println!("Welcome {}!", id);
        print!("{}", color::Fg(color::Reset));

        loop {
            print!("{}", color::Fg(color::Green));
            println!("Choose an option");
            print!("{}", color::Fg(color::Reset));
            print!("{}", color::Fg(color::Blue));
            println!("{}.{} (For {})", '1', "Write new log", now);
            println!("{}.{}", '2', "Browse all logs");
            print!("{}", color::Fg(color::Reset));

            match read_input() {
                Some(c) => input = c,
                None => println!("Didn't work!"),
            }

            io::stdout().flush().unwrap();
            if input == "2" {
                browse_logs(id);
            } else {
                create_log(id, &now.to_string());
            }
        }
    }

    pub fn create_log(id: &str, now: &str) {
        loop {
            match read_password() {
                Some(c) => {
                    if let Ok(_) = key_chain::valid_auth(id, &c) {
                        CryptoEditor::create_log(now, id, &c);
                        break;
                    }
                }
                None => {
                    println!("Didn't work!");
                    break;
                }
            }
        }
    }

    pub fn browse_logs(id: &str) {
        let mut input: String = String::new();
        let year: String;
        let month: String;
        let day: String;
        let log: (usize, String);

        print!("{}", color::Fg(color::Green));
        println!("Browsing {}'s logs", id);
        print!("{}", color::Fg(color::Reset));

        if logs_api::has_logs(id) {
            loop {
                display_years(id);
                match read_input() {
                    Some(c) => input = c,
                    None => println!("Didn't work!"),
                }

                if let Ok(years) = logs_api::get_years(id) {
                    match process_option(input.clone(), years) {
                        Some(y) => {
                            year = y.1;
                            print!("{}", color::Fg(color::Green));
                            println!("Selected year: {}", year);
                            print!("{}", color::Fg(color::Reset));
                            break;
                        }
                        None => {
                            println!("Invalid input. Choose one of the options.");
                        }
                    }
                }
            }

            loop {
                display_months(id, &year);
                match read_input() {
                    Some(c) => input = c,
                    None => println!("Didn't work!"),
                }

                match process_option(
                    input.clone(),
                    logs_api::get_months(id, &year).expect("Unable to get years"),
                ) {
                    Some(m) => {
                        month = m.1;
                        print!("{}", color::Fg(color::Green));
                        println!("Selected month: {}", month);
                        print!("{}", color::Fg(color::Reset));
                        break;
                    }
                    None => {
                        println!("Invalid input. Choose one of the options.");
                    }
                }
            }
            loop {
                display_days(id, &year, &month);
                match read_input() {
                    Some(c) => input = c,
                    None => println!("Didn't work!"),
                }

                match process_option(
                    input.clone(),
                    logs_api::get_days(id, &year, &month).expect("Unable to get years"),
                ) {
                    Some(d) => {
                        day = d.1;
                        print!("{}", color::Fg(color::Green));
                        println!("Selected day: {}", day);
                        print!("{}", color::Fg(color::Reset));
                        break;
                    }
                    None => {
                        println!("Invalid input. Choose one of the options.");
                    }
                }
            }
            loop {
                display_logs(id, &year, &month, &day);
                match read_input() {
                    Some(c) => input = c,
                    None => println!("Didn't work!"),
                }

                match process_option(
                    input.clone(),
                    logs_api::get_day_logs(id, &year, &month, &day).expect("Unable to get logs"),
                ) {
                    Some(l) => {
                        log = l;
                        print!("{}", color::Fg(color::Green));
                        println!("Selected log: {}", log.1);
                        print!("{}", color::Fg(color::Reset));
                        break;
                    }
                    None => {
                        println!("Invalid input. Choose one of the options.");
                    }
                }
            }

            print!("{}", color::Fg(color::Green));
            println!("Would you like to edit or read {}?", log.1);
            print!("{}", color::Fg(color::Reset));
            let log_options = vec![String::from("Edit"), String::from("Read")];
            display_options(&log_options);

            match read_input() {
                Some(c) => input = c,
                None => println!("Didn't work!"),
            }

            match process_option(input.clone(), log_options) {
                Some(action) => loop {
                    match read_password() {
                        Some(c) => {
                            if let Ok(_) = key_chain::valid_auth(id, &c) {
                                match action.1.as_ref() {
                                    "Edit" => {
                                        CryptoEditor::edit_log(
                                            &format!("{}_{}_{}", day, month, year),
                                            id,
                                            &c,
                                            log.0,
                                        );
                                    }
                                    "Read" => {
                                        CryptoEditor::edit_log(
                                            &format!("{}_{}_{}", day, month, year),
                                            id,
                                            &c,
                                            log.0,
                                        );
                                    }
                                    _ => (),
                                }
                                break;
                            }
                            print!("{}", color::Fg(color::Green));
                            println!("Incorrect password!");
                            print!("{}", color::Fg(color::Reset));
                        }
                        None => {
                            println!("Didn't work!");
                            break;
                        }
                    }
                },
                None => {
                    println!("Invalid input. Choose one of the options.");
                }
            }
        } else {
            print!("{}", color::Fg(color::Red));
            println!("User does not have any logs!");
            print!("{}", color::Fg(color::Reset));
        }
    }

    fn display_options(options: &Vec<String>) {
        print!("{}", color::Fg(color::Blue));
        list_tools::utils::display_options(options.clone());
        print!("{}", color::Fg(color::Reset));
    }

    fn display_logs(id: &str, year: &str, month: &str, day: &str) {
        print!("{}", color::Fg(color::Blue));
        let logs = logs_api::get_day_logs(id, year, month, day).expect("Unable to get logs");
        list_tools::utils::display_options(logs);
        print!("{}", color::Fg(color::Reset));
    }
    fn display_days(id: &str, year: &str, month: &str) {
        print!("{}", color::Fg(color::Blue));
        let logs = logs_api::get_days(id, year, month).expect("Unable to get days");
        list_tools::utils::display_options(logs);
        print!("{}", color::Fg(color::Reset));
    }

    fn display_months(id: &str, year: &str) {
        print!("{}", color::Fg(color::Blue));
        let logs = logs_api::get_months(id, year).expect("Unable to get months");
        list_tools::utils::display_options(logs);
        print!("{}", color::Fg(color::Reset));
    }

    fn display_years(id: &str) {
        print!("{}", color::Fg(color::Blue));
        let logs = logs_api::get_years(id).expect("Unable to get years");
        list_tools::utils::display_options(logs);
        print!("{}", color::Fg(color::Reset));
    }
}
