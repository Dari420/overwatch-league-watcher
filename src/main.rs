use webbrowser;
use enigo::*;
use std::thread::sleep_ms;
use chrono::{DateTime, Utc};
use std::io::stdin;
use std::time::Duration;

fn main() {
    let mut size = String::new();
    println!("Dari's Overwatch League watcher. Special thanks to https://gigabra.in/ for providing a platform that actually gives tokens instead of this inconsistent mess of a \"system\" that is overwatchleague.com\nSource code at https://github.com/Dari420/overwatch-league-watcher");
    loop {
        let current_time: DateTime<Utc> = Utc::now();
        println!("UTC now is: {}", current_time.format("%a %b %e %T %Y"));
        let formatted_time: String = format!("{}", current_time.format("%a %b %e %T %Y"));
        let mut date: String = String::from(&formatted_time);
        let mut time: String = String::from(&formatted_time);
        date.truncate(3);
        time.truncate(16);
        time = time.chars().rev().collect::<String>();
        time.truncate(5);
        time = time.chars().rev().collect::<String>();
        if date == String::from("Sat") && time == String::from("08:05") || time == String::from("18:35") {
            open_gigabrain_overwatch();
        }
        else if date == String::from("Sun") && time == String::from("08:05") || time == String::from("18:35") {
            open_gigabrain_overwatch();
        }
        sleep_ms(60000);
    }
}

fn open_gigabrain_overwatch() {
    let mut enigo = Enigo::new();
    println!("Opening overwatch league.");
    if webbrowser::open("https://gigabra.in").is_ok() {
        /*//Clicks on every possible login location starting from most likely to least likely
        enigo.mouse_move_to(1300, 260);
        sleep_ms(2000);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
        enigo.mouse_move_to(1300, 150);
        sleep_ms(200);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
        //waits for the disclaimer page to allow the button to be clicked, then clicks it
        enigo.mouse_move_to(700, 600);
        sleep_ms(6500);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
        //clicks allow button
        enigo.mouse_move_to(600, 700);
        sleep_ms(1500);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);*/
        //plays video
        enigo.mouse_move_to(500, 500);
        sleep_ms(3000);
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
    }
}
