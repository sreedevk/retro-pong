mod app;
mod frame;
mod game;
mod graphics;
mod sprite;

use anyhow::Result;
use app::App;

/* ░ ▒ ▓ █ ▀ ▄ ▌ ▐ ■ ▪ */

fn main() -> Result<()> {
    let mut app = App::new();
    app.init()
}
