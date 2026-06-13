use notify_rust::Notification;

pub fn send_notification(title: &str, message: &str) {
    let _ = Notification::new()
        .summary(title)
        .body(message)
        .appname("pomoru")
        .timeout(5000)
        .show();
}