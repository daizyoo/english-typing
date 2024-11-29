mod game;
mod gemini;
mod level;
mod meaning;
mod utils;

use getch_rs::{Getch, Key};

use game::Game;
use utils::{clear, cursor_change, input, input_msg, restore_cursor};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().unwrap();

    clear();
    cursor_change();

    input_msg("level 1~10: ", 0);
    let level = input().trim().parse::<usize>().unwrap_or(1);
    input_msg("meaning level 1~5: ", 1);
    let meaning_level = input().trim().parse::<usize>().unwrap_or(1);
    input_msg("number of questions: ", 2);
    let end = input().trim().parse::<u8>().unwrap_or(5);

    let mut game = Game::new(level, meaning_level, end);
    game.change_section();

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
