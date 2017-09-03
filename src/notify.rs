use notify_rust::{Notification, NotificationHandle, NotificationHint, Error};

use std::sync::Arc;

use app;

pub fn new(
    app: &Arc<app::App>,
    summary: String,
    body: String,
) -> Result<NotificationHandle, Error> {
    Notification::new()
        .summary(summary.as_ref())
        .body(body.as_ref())
        .icon(app.notify.icon)
        .appname(app.name)
        .action("read", "📭")
        .action("open", "🌎")
        .hint(NotificationHint::Category("email".to_owned()))
        .hint(NotificationHint::Resident(true))
        .timeout(app.notify.timeout)
        .show()
    // .show_debug()
}
