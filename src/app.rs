use std::sync::Arc;

pub struct App {
    pub name: &'static str,
    pub version: &'static str,
    pub token: &'static str,
    pub notify: Notify,
}

pub struct Notify {
    pub icon: &'static str,
    pub timeout: i32,
}

pub fn app() -> Arc<App> {
    Arc::new(App {
        name: "rapier",
        version: "0.1.0",
        token: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        notify: Notify {
            icon: "thunderbird-bin", // iconname
            timeout: 20000, // milliseconds
        },
    })
}
