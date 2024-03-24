use crate::utils::to_vec::string_to_vec;
use tui_input::Input;

pub enum CurrentScreen {
    Main,
    Editing,
    Setting,
    Exiting,
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct AppTui {
    pub input_uri: Input,
    pub input_mode: InputMode,
    pub curr_screen: CurrentScreen,
    pub saved_input: Vec<String>,
}

impl AppTui {
    pub fn new() -> Self {
        Self {
            input_uri: Input::default(),
            input_mode: InputMode::Normal,
            curr_screen: CurrentScreen::Main,
            saved_input: Vec::new(),
        }
    }

    pub fn save_input(&mut self) {
        let input_value = self.input_uri.value();

        if input_value.contains(',') {
            let mut vec_str = string_to_vec(input_value);
            self.saved_input.append(&mut vec_str);
        } else {
            self.saved_input.push(input_value.into());
        }

        self.input_uri.reset();
    }

    pub fn print_vec(&self) -> eyre::Result<()> {
        let output = serde_json::to_string_pretty(&self.saved_input)?;
        println!("{}", output);
        Ok(())
    }
}
