mod app;
mod appui;
mod music;
mod file;
mod helper;
use color_eyre::Result;
use app::App;



fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}


