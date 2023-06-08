use std::fs::File;
use std::io::{BufReader, Write};
use tokio::time::{sleep};
use std::time::Duration;
use notify_rust::{Notification, Timeout};
use rodio;
use rodio::{Decoder, OutputStream, Source};

struct Status {
    title: String,
    body: String,
    icon: String,
    sound: String,
}


#[tokio::main]
async fn main() {
    let work = Status {
        title: "Work".to_string(),
        body: "Its time to work".to_string(),
        icon: "face-cool".to_string(),
        sound: "work.wav".to_string(),
    };
    let break_time = Status {
        title: "Break".to_string(),
        body: "Its time to take some rest".to_string(),
        icon: "face-smile-big".to_string(),
        sound: "break.wav".to_string(),
    };
    loop {
        timer(25, &work).await;
        timer(5, &break_time).await;
    }
}


async fn timer(minutes: i32, status: &Status) {
    send_notification(&status);
    sound_alert(&status);
    let mut total_time_ticker = 1;
    let mut seconds_ticker = 0;
    let mut minutes_ticker = 0;
    while total_time_ticker < minutes * 10 {
        sleep(Duration::from_secs(1)).await;
        seconds_ticker = seconds_ticker + 1;
        if seconds_ticker == 60 {
            seconds_ticker = 0;
            minutes_ticker = minutes_ticker + 1;
        }
        print!("\r>> {}  --- {}:{} ---  ", &status.title,
               if minutes_ticker < 10 {
                   String::from("0") + &minutes_ticker.to_string()
               } else {
                   minutes_ticker.to_string()
               },
               if seconds_ticker < 10 {
                   String::from("0") + &seconds_ticker.to_string()
               } else {
                   seconds_ticker.to_string()
               });
        std::io::stdout().flush().expect("error occurred");
        total_time_ticker = total_time_ticker + 1;
    }
}

fn send_notification(status: &Status) {
    Notification::new()
        .summary(&status.title)
        .body(&status.body)
        .icon(&status.icon)
        .timeout(Timeout::Milliseconds(6000))
        .show().unwrap();
}

fn sound_alert(status: &Status) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(&status.sound).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).expect("error occurred");
    std::thread::sleep(std::time::Duration::from_secs(1));
}