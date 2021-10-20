use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub mod crypto_editor {
    pub mod document;
    pub mod editor;
    pub mod row;
    pub mod terminal;
    pub use document::Document;
    pub use editor::Editor;
    pub use editor::EditorEvent;
    pub use editor::EditorMessage;
    pub use editor::Position;
    pub use row::Row;
    pub use terminal::Terminal;
}

pub use crypto_editor::editor::*;
pub use crypto_editor::*;

pub struct CryptoEditor;

pub use super::database::database_handler;
pub use super::database::keychain;
pub use database_handler::*;
pub use keychain::*;

impl CryptoEditor {
    pub fn edit_log(date: &str, user: &str, password: &str, log_pos: usize) {
        let (tx, rx): (Sender<EditorMessage>, Receiver<EditorMessage>) = mpsc::channel();
        let date_temp = String::from(date).chars().collect::<Vec<char>>().to_vec();
        let user_temp = String::from(user).chars().collect::<Vec<char>>().to_vec();
        let password_temp = String::from(password)
            .chars()
            .collect::<Vec<char>>()
            .to_vec();

        match logs_api::get_date_log(date, user, password, log_pos) {
            Ok(content) => {
                let child = thread::spawn(move || loop {
                    let msg = &rx.recv().unwrap();

                    match msg.event {
                        EditorEvent::Save => {
                            if let Err(_) = logs_api::overwrite_log(
                                &date_temp.iter().cloned().collect::<String>(),
                                &user_temp.iter().cloned().collect::<String>(),
                                &msg.message,
                                &password_temp.iter().cloned().collect::<String>(),
                                log_pos,
                            ) {
                                break;
                            }
                        }
                        EditorEvent::Exit => break,
                    }
                });

                Editor::open_log(content, date, tx).run();

                child.join().expect("oops! the child thread panicked");
            }
            Err(why) => {
                println!("Unable to open log!");
                println!("{}", why);
            }
        };
    }

    pub fn create_log(date: &str, user: &str, password: &str) {
        let (tx, rx): (Sender<EditorMessage>, Receiver<EditorMessage>) = mpsc::channel();
        let date_temp = String::from(date).chars().collect::<Vec<char>>().to_vec();
        let user_temp = String::from(user).chars().collect::<Vec<char>>().to_vec();
        let password_temp = String::from(password)
            .chars()
            .collect::<Vec<char>>()
            .to_vec();
        // EVALUATE IF LOG ALREADY EXISTS
        let child = thread::spawn(move || {
            let log_pos;
            let msg = &rx.recv().unwrap();

            match msg.event {
                EditorEvent::Save => {
                    log_pos = logs_api::insert_log(
                        &date_temp.iter().cloned().collect::<String>(),
                        &user_temp.iter().cloned().collect::<String>(),
                        &msg.message,
                        &password_temp.iter().cloned().collect::<String>(),
                    )
                    .expect("Unable to write a new log");

                    loop {
                        let msg = &rx.recv().unwrap();
                        match msg.event {
                            EditorEvent::Save => {
                                logs_api::overwrite_log(
                                    &date_temp.iter().cloned().collect::<String>(),
                                    &user_temp.iter().cloned().collect::<String>(),
                                    &msg.message,
                                    &password_temp.iter().cloned().collect::<String>(),
                                    log_pos,
                                )
                                .expect("Unable to write a new log");
                            }
                            EditorEvent::Exit => break,
                        }
                    }
                }
                EditorEvent::Exit => (),
            }
        });
        Editor::default(tx).run();
        child.join().expect("oops! the child thread panicked");
    }
}
