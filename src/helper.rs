use ratatui::widgets::TableState;

use crate::app::InputMode;

pub struct HelpTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
    pub last_mod : InputMode,
}

impl Default for HelpTable {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpTable{
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            items: vec![
                vec![">>>File Browser<<<".to_string(), "".to_string()],
                vec!["q | ESC".to_string(), "Quit".to_string()],
                vec!["l | Right".to_string(), "Switch To Playing List".to_string()],
                vec!["j | Down".to_string(), "Select Next Item".to_string()],
                vec!["k | Up".to_string(), "Select Previous Item".to_string()],
                vec!["g".to_string(), "Select First Item".to_string()],
                vec!["G".to_string(), "Select Last Item".to_string()],
                vec!["a | Enter".to_string(), "Add Music To Playing List".to_string()],
                vec!["A".to_string(), "Add All The Music In This Folder To Playing List".to_string()],
                vec!["o".to_string(), "Open Folder".to_string()],
                vec!["Backspace".to_string(), "Close Folder".to_string()],
                vec!["Tab".to_string(), "Helper".to_string()],
                vec!["".to_string(), "".to_string()],


                vec![">>>Playing List<<<".to_string(), "".to_string()],
                vec!["q | ESC".to_string(), "Quit".to_string()],
                vec!["h | Left".to_string(), "Switch To File Browser".to_string()],
                vec!["j | Down".to_string(), "Select Next Item".to_string()],
                vec!["k | Up".to_string(), "Select Previous Item".to_string()],
                vec!["g".to_string(), "Select First Item".to_string()],
                vec!["G".to_string(), "Select Last Item".to_string()],
                vec!["Enter".to_string(), "Play Current Music".to_string()],
                vec!["p".to_string(), "Play / Pause".to_string()],
                vec!["s".to_string(), "Stop Playing".to_string()],
                vec!["n".to_string(), "Play Next Music".to_string()],
                vec!["d".to_string(), "Remove from Playing List(slow)".to_string()],
                vec!["D".to_string(), "Remove from Playing List(fast, but may change order)".to_string()],
                vec!["m".to_string(), "Change Playing Mod (Auto|Repeat|Random|Manual)".to_string()],
                vec!["+".to_string(), "Volume Up".to_string()],
                vec!["-".to_string(), "Volume Down".to_string()],
                vec!["Tab".to_string(), "Helper".to_string()],
                vec!["".to_string(), "".to_string()],



                vec![">>>Helper<<<".to_string(), "".to_string()],
                vec!["j | Down".to_string(), "Select Next Item".to_string()],
                vec!["k | Up".to_string(), "Select Previous Item".to_string()],
                vec!["q | ESC | Tab".to_string(), "Quit Helper".to_string()],
            ],
            last_mod: InputMode::Filelist,
        }
    }
}
