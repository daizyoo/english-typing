mod game;
mod gemini;
mod level;
mod meaning;

use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use game::Game;
use getch_rs::Getch;
use getch_rs::Key;
use rand::{thread_rng, Rng};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().unwrap();
    clear();
    let level = input_msg("level 1~10: ", 0);
    let meaning_level = input_msg("meaning level 1~5 (default: 1): ", 1);
    if meaning_level > 5 {
        panic!("meaning level is not found");
    }
    let end = input_msg("number of questions: ", 2);

    let mut game = Game::new(level, meaning_level, end);
    game.change_section();
    cursor_change();

    loop {
        game.draw();
        match Getch::new().getch() {
            Ok(Key::Char('\r')) => continue, // 改行は無視
            Ok(Key::Char(c)) => game.input(c),
            Ok(Key::Delete | Key::Backspace) => game.delete(),
            Ok(Key::Ctrl('c')) => break, //終了
            _ => continue,
        }
        game.change_section();
        if game.is_input_end() {
            if game.is_correct() {
                game.correct_count()
            }
            game.next_question()
        }
        if game.is_game_end() {
            break;
        }
    }
    game.quit();
    game.show_meanings().await;
    restore_cursor();
}

/// カーソルをブロック(点滅)にする
fn restore_cursor() {
    print!("\x1B[1 q");
}

/// カーソルを棒にする
fn cursor_change() {
    print!("\x1B[5 q");
}

/// カーソルを移動
fn move_cursor(x: u16, y: u16) {
    execute!(stdout(), MoveTo(x, y)).unwrap();
}

fn meaning_print(text: &str) {
    for c in text.chars() {
        print(&c.to_string(), Color::Green);
        sleep(Duration::from_secs_f32(thread_rng().gen_range(0.03..0.15)));
    }
}

/// 文字列を色付きで出力
fn print(str: &str, color: Color) {
    if str == " " && color == Color::Red {
        execute!(stdout(), SetBackgroundColor(color), Print(str), ResetColor).unwrap();
    } else {
        execute!(stdout(), SetForegroundColor(color), Print(str), ResetColor).unwrap();
    }
}

/// 画面をクリア
fn clear() {
    print!("\x1B[2J\x1B[H");
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn input_msg<T: std::str::FromStr>(str: &str, y: u16) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    println!("{}", str);
    move_cursor(str.len() as u16 + 1, y);
    input().trim().parse().expect("parse error")
}
