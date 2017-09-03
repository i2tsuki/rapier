extern crate github_rs;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate notify_rust;

use std::process;
use std::rc::Rc;
use std::{thread, time};

use github_rs::client::Github;

mod app;
mod notification;
mod notify;
mod utils;

fn main() {
    let app = app::app();

    let client = Github::new(app.token).unwrap();
    loop {
        let notifications = client.get().notifications().execute();
        match notifications {
            Ok((_headers, _status, json)) => {
                if let Some(json) = json {
                    let notifications: Vec<notification::Notification> =
                        match serde_json::from_value(json) {
                            Ok(n) => (n),
                            Err(e) => {
                                println!("{}", e);
                                process::exit(1);
                            }
                        };
                    for notification in notifications {
                        let notification = Rc::new(notification);
                        println!("{}", notification.id);
                        let handle = match notify::new(
                            &app,
                            notification.repository.full_name.clone(),
                            notification.subject.title.clone(),
                        ) {
                            Ok(handle) => handle,
                            Err(e) => {
                                println!("{}", e);
                                process::exit(1);
                            }
                        };
                        handle.wait_for_action(|action| {
                            match action {
                                "read" => {
                                    // Todo:: Implementation of notification_as_read
                                    // notification_as_read();
                                }
                                "open" => {
                                    match utils::xdg_open(
                                        utils::human_url(notification.subject.url.clone()),
                                    ) {
                                        Ok(_) => (),
                                        Err(e) => {
                                            println!("{}", e);
                                            process::exit(1);
                                        }
                                    };
                                }
                                _ => println!("received: {}", action),
                            }
                        });
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
        thread::sleep(time::Duration::from_millis(app.interval));
    }
}
