
use std::process::*;
use crate::wlr::*;

pub struct Bar {
    command: &str
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct MenuEntry {
    title: &str,
    command: &str
}


#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct KeyBinding {
    description: &'static str,
    key: &'static str,
    mod_key: &'static str,
    command: &'static str
}

pub struct Configuration
{
    key_bindings: Vec<KeyBinding>,
    menu: Vec<MenuEntry>,
    bar: Bar
}

impl Configuration {
    pub fn from_file(file: &str) {
        let data = read_file(file);

        // FIXME Deserialize with Serde
    }

    fn read_file(path: &str) -> String {
        let file = File::open(path).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        return contents;
    }

    pub fn new() -> Configuration {
        Configuration{
            key_bindings: vec![
                // Start a Terminal
                KeyBinding { key: "F1", mod_key: "Logo", command: "termite" },
                // Run the launcher
                KeyBinding { key: "F2", mod_key: "Logo", command: "dmenu_run" },
                // Close/Kill the focued Window
                KeyBinding { key: "Q", mod_key: "Logo", command: "kill <focused_window>" },
            ]
        }
    }

    pub fn matches_modifiers(&self, modifiers: u32) -> bool {
        let mod_keys: Vec<&'static str> = self.key_bindings.iter()
                                                           .map(|f| f.mod_key)
                                                           .collect();

        for mod_key in mod_keys {
            match mod_key {
                "Logo" => return modifiers & (wlr_keyboard_modifier_WLR_MODIFIER_LOGO) != 0,
                "Alt" => return modifiers & (wlr_keyboard_modifier_WLR_MODIFIER_ALT) != 0,
                &_ => continue
            }
        }
        false
    }

    pub fn handle_keybinding(&self, sym: xkb_keysym_t) -> bool {
        #[allow(non_upper_case_globals)]
        match sym {
            XKB_KEY_F1 => return self.execute_binding("F1"),
            XKB_KEY_F2 => return self.execute_binding("F2"),
            XKB_KEY_Q => return self.execute_binding("Q"),
            _ => return false
        }
    }

    fn execute_binding(&self, key: &str) -> bool {
        let binding = self.key_bindings.iter().find(|x| x.key == key);

        match binding {
            Some(b) => self.execute_command(b.command),
            _ => false
        }
    }

    fn execute_command(&self, command: &str) -> bool {
        let handle = Command::new(command)
            .spawn()
            .expect("Command failed to start");
        true
    }
}