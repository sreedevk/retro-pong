mod app;
mod frame;
mod game;
mod graphics;

use anyhow::Result;
use app::App;

/* ░ ▒ ▓ █ ▀ ▄ ▌ ▐ ■ ▪ */

fn main() -> Result<()> {
    let mut app = App::new();
    app.init()
}
