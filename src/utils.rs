use std::{io::stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use rand::{thread_rng, Rng};

/// カーソルをブロック(点滅)にする
pub fn restore_cursor() {
    print!("\x1B[1 q");
}

/// カーソルを棒にする
pub fn cursor_change() {
    print!("\x1B[5 q");
}

/// カーソルを移動
pub fn move_cursor(x: u16, y: u16) {
    execute!(stdout(), MoveTo(x, y)).unwrap();
}

/// 文字列を色付きで出力
pub fn meaning_print(text: &str) {
    for c in text.chars() {
        print(&c.to_string(), Color::Green);
        sleep(Duration::from_secs_f32(thread_rng().gen_range(0.03..0.15)));
    }
}

/// 文字列を色付きで出力
pub fn print(str: &str, color: Color) {
    if str == " " && color == Color::Red {
        execute!(stdout(), SetBackgroundColor(color), Print(str), ResetColor).unwrap();
    } else {
        execute!(stdout(), SetForegroundColor(color), Print(str), ResetColor).unwrap();
    }
}

/// 画面をクリア
pub fn clear() {
    print!("\x1B[2J\x1B[H");
}

/// 入力
pub fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn input_msg(str: &str, y: u16) {
    print!("{}", str);
    move_cursor(str.len() as u16, y);
}
