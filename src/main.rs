use mac_notification_sys::*;
use sysinfo::{System, SystemExt};

fn main() {
    // use notify_rust::Notification;
    let mut sys = System::new_all();
    sys.refresh_all();
    let threshold = 0.50;

    loop {
        sys.refresh_all();
        let total_memory: f64 = sys.total_memory() as f64;
        let used_memory: f64 = sys.used_memory() as f64;
        let used_memory_percentage = used_memory / total_memory;

        if used_memory_percentage > threshold {
            println!(
                "Current used memory percentage: {:?}",
                used_memory_percentage
            );
            send_notification(
                "Exceeding Memory",
                Some("WARNING"),
                &format!(
                    "Looks like memory has exceeded than the given threshold: {}",
                    threshold
                ),
                Some(Notification::new().sound("Blow")),
            )
            .unwrap();
        }

        std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
