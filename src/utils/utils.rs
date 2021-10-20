pub mod file_system {
    use std::env;
    use std::fs;
    use std::io;
    use std::io::{Error, ErrorKind};
    pub fn read_dir_to_string(dir_path: String) -> Result<Vec<String>, io::Error> {
        let mut files: Vec<String> = Vec::new();
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let file_name = match entry.file_name().into_string() {
                Err(_) => continue,
                Ok(str) => str,
            };
            files.push(file_name);
        }
        Ok(files)
    }

    pub fn generate_log_path(logs_dir: &str, n: usize) -> String {
        let filename = format!("log_{}.dat", n);
        generate_path(&[logs_dir, "/", &filename])
    }

    pub fn generate_salt_path(id: &str) -> Option<String> {
        let filename = format!("{}.txt", id);
        if let Ok(dir) = get_env_var(&"DATABASE_KEYS_DIR") {
            return Some(generate_path(&[&dir, "/", id, "/", &filename]));
        }
        None
    }

    pub fn generate_pass_path(id: &str) -> Option<String> {
        if let Ok(dir) = get_env_var(&"DATABASE_KEYS_DIR") {
            return Some(generate_path(&[&dir, "/", id, "/", "pass.txt"]));
        }
        None
    }

    pub fn generate_key_dir(id: &str) -> Result<(), Error> {
        if let Ok(dir) = get_env_var(&"DATABASE_KEYS_DIR") {
            let path = generate_path(&[&dir, "/", id, "/"]);
            fs::create_dir_all(path)?;

            return Ok(());
        }

        Err(Error::new(
            ErrorKind::NotFound,
            "DATABASE_KEYS_DIR env variable Not Found",
        ))
    }

    pub fn generate_path(path: &[&str]) -> String {
        let mut complete_path = String::new();
        for part in path {
            complete_path.push_str(&part);
        }
        complete_path
    }
    pub fn format_txt_filename(filename: &str) -> &str {
        &filename[0..filename.len() - 4]
    }
    pub fn get_env_var(var_name: &str) -> Result<String, Error> {
        match env::var(var_name) {
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                var_name.to_owned() + " env variable Not Found",
            )),
            Ok(key_dir) => Ok(key_dir),
        }
    }
}
