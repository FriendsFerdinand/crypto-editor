pub mod logs_api {
    use crate::encryption::crypto::*;
    use crate::utils::utils::file_system;
    use std::env;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::{Error, ErrorKind};

    #[derive(Debug)]
    pub struct Log {
        pub content: String,
    }

    pub fn get_key_ids() -> Result<Vec<String>, Error> {
        match env::var("DATABASE_IDS_DIR") {
            Ok(ids_dir) => {
                let ids = file_system::read_dir_to_string(ids_dir)?;
                Ok(ids)
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                "DATABASE_IDS_DIR env variable Not Found",
            )),
        }
    }

    pub fn has_logs(id: &str) -> bool {
        if let Ok(_) = get_years(id) {
            return true;
        }

        false
    }

    pub fn get_years(id: &str) -> Result<Vec<String>, Error> {
        match env::var("DATABASE_IDS_DIR") {
            Ok(db_dir) => {
                let mut dates = file_system::read_dir_to_string(db_dir + "/" + id)?;
                dates.sort_by(|a, b| {
                    a.parse::<i32>()
                        .unwrap()
                        .partial_cmp(&b.parse::<i32>().unwrap())
                        .unwrap()
                });
                Ok(dates)
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                "DATABASE_IDS_DIR env variable Not Found",
            )),
        }
    }

    pub fn get_months(id: &str, year: &str) -> Result<Vec<String>, Error> {
        match env::var("DATABASE_IDS_DIR") {
            Ok(db_dir) => {
                let mut dates = file_system::read_dir_to_string(db_dir + "/" + id + "/" + year)?;
                dates.sort_by(|a, b| {
                    a.parse::<i32>()
                        .unwrap()
                        .partial_cmp(&b.parse::<i32>().unwrap())
                        .unwrap()
                });
                Ok(dates)
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                "DATABASE_IDS_DIR env variable Not Found",
            )),
        }
    }

    pub fn get_days(id: &str, year: &str, month: &str) -> Result<Vec<String>, Error> {
        match env::var("DATABASE_IDS_DIR") {
            Ok(db_dir) => {
                let mut dates =
                    file_system::read_dir_to_string(db_dir + "/" + id + "/" + year + "/" + month)?;
                dates.sort_by(|a, b| {
                    a.parse::<i32>()
                        .unwrap()
                        .partial_cmp(&b.parse::<i32>().unwrap())
                        .unwrap()
                });
                Ok(dates)
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                "DATABASE_IDS_DIR env variable Not Found",
            )),
        }
    }

    pub fn get_day_logs(
        id: &str,
        year: &str,
        month: &str,
        day: &str,
    ) -> Result<Vec<String>, Error> {
        let dates = file_system::read_dir_to_string(get_user_logs_dir(
            &format!("{}_{}_{}", day, month, year),
            id,
        )?)?;
        Ok(dates)
    }

    pub fn get_user_logs_dir(date: &str, user: &str) -> Result<String, Error> {
        let db_dir = file_system::get_env_var("DATABASE_IDS_DIR")?;
        let split_date: Vec<&str> = date.split('_').collect();
        if split_date.len() == 3 {
            let d = split_date[0];
            let m = split_date[1];
            let y = split_date[2];

            let file_path =
                file_system::generate_path(&[&db_dir, "/", &user, "/", &y, "/", m, "/", d]);
            Ok(file_path)
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid date"))
        }
    }

    pub fn write_log(file_path: &str, ciphertext: &[u8]) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        file.write(ciphertext)?;

        Ok(())
    }

    pub fn insert_log(
        date: &str,
        user: &str,
        message: &str,
        password: &str,
    ) -> Result<usize, Error> {
        let db_dir = get_user_logs_dir(&date, &user)?;
        fs::create_dir_all(db_dir.clone())?;
        let previous_logs = file_system::read_dir_to_string(db_dir.clone())?;
        let file_path = file_system::generate_log_path(&db_dir, previous_logs.len());
        let ciphertext = match crypto::encrypt_str(user, message, password) {
            Err(why) => Err(Error::new(ErrorKind::Other, why)),
            Ok(text) => Ok(text),
        }?;

        let ciphertext = unsafe { String::from_utf8_unchecked(ciphertext) };

        match write_log(&file_path, ciphertext.as_bytes()) {
            Ok(_) => Ok(previous_logs.len()),
            Err(err) => Err(err),
        }
    }

    pub fn overwrite_log(
        date: &str,
        user: &str,
        message: &str,
        password: &str,
        log_pos: usize,
    ) -> Result<(), Error> {
        let db_dir = get_user_logs_dir(&date, &user)?;
        fs::create_dir_all(db_dir.clone())?;
        let file_path = file_system::generate_log_path(&db_dir, log_pos);
        let ciphertext = match crypto::encrypt_str(user, message, password) {
            Err(why) => Err(Error::new(ErrorKind::Other, why)),
            Ok(text) => Ok(text),
        }?;

        let ciphertext = unsafe { String::from_utf8_unchecked(ciphertext) };

        write_log(&file_path, ciphertext.as_bytes())
    }

    pub fn get_date_history_logs(
        date: &str,
        user: &str,
        password: &str,
    ) -> Result<Vec<Log>, Error> {
        let db_dir = get_user_logs_dir(&date, &user)?;
        let previous_logs = file_system::read_dir_to_string(db_dir.clone())?;
        let mut contents = Vec::new();
        for i in 0..previous_logs.len() {
            let log_path = file_system::generate_log_path(&db_dir, i);
            match String::from_utf8(decrypt_log(user, password, &log_path)?) {
                Ok(decrypted_content) => contents.push(Log {
                    content: decrypted_content,
                }),
                Err(_) => println!("Unable to decrypt message"),
            };
        }

        Ok(contents)
    }

    pub fn get_date_log(
        date: &str,
        user: &str,
        password: &str,
        pos: usize,
    ) -> Result<Vec<u8>, Error> {
        let db_dir = get_user_logs_dir(&date, &user)?;
        let log_path = file_system::generate_log_path(&db_dir, pos);
        decrypt_log(user, password, &log_path)
    }

    pub fn decrypt_log(user: &str, password: &str, file_path: &str) -> Result<Vec<u8>, Error> {
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut content = vec![];

        file.read_to_end(&mut content)?;
        let decrypted_message;
        unsafe {
            decrypted_message =
                crypto::decrypt_str(user, &String::from_utf8_unchecked(content), password)
                    .expect("Error in decryption");
        };

        Ok(decrypted_message)
    }
}
