#[macro_use]
extern crate log;
// extern crate simplelog;
use simplelog::*;
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
use std::fs::File;
pub use terminal::Terminal;

fn main() {
    let _ = CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("log.log").unwrap(),
        ),
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ]);
    Editor::default().run();
}
