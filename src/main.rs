#![windows_subsystem = "windows"]

use macroquad::prelude::*;
use std::fs;

// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "BluamekBoard :)".to_owned(),
        window_width: 600,
        window_height: 300,
        ..Default::default()
    }
}

// Global key press detection - works even when window is unfocused
#[cfg(windows)]
fn is_key_held(vk: u16) -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
    unsafe { GetAsyncKeyState(vk as i32) & (0x8000u16 as i16) != 0 }
}

fn parse_color(name: &str) -> Color {
    match name {
        "DARKGRAY"  => DARKGRAY,
        "LIGHTGRAY" => LIGHTGRAY,
        "RED"       => RED,
        "GREEN"     => GREEN,
        "BLUE"      => BLUE,
        "YELLOW"    => YELLOW,
        "ORANGE"    => ORANGE,
        "PURPLE"    => PURPLE,
        "PINK"      => PINK,
        "BLACK"     => BLACK,
        _           => WHITE,
    }
}

/// Returns (macroquad KeyCode, Win32 VK code) for a given key name.
/// Key names follow the keycodes crate convention (docs.rs/keycodes).
/// The VK codes are the raw u32 values from that crate, cast to u16.
fn parse_keycode(name: &str) -> Option<(KeyCode, u16)> {
    match name {
        // Alphabet
        "KEY_A" => Some((KeyCode::A, keycodes::KEY_A as u16)),
        "KEY_B" => Some((KeyCode::B, keycodes::KEY_B as u16)),
        "KEY_C" => Some((KeyCode::C, keycodes::KEY_C as u16)),
        "KEY_D" => Some((KeyCode::D, keycodes::KEY_D as u16)),
        "KEY_E" => Some((KeyCode::E, keycodes::KEY_E as u16)),
        "KEY_F" => Some((KeyCode::F, keycodes::KEY_F as u16)),
        "KEY_G" => Some((KeyCode::G, keycodes::KEY_G as u16)),
        "KEY_H" => Some((KeyCode::H, keycodes::KEY_H as u16)),
        "KEY_I" => Some((KeyCode::I, keycodes::KEY_I as u16)),
        "KEY_J" => Some((KeyCode::J, keycodes::KEY_J as u16)),
        "KEY_K" => Some((KeyCode::K, keycodes::KEY_K as u16)),
        "KEY_L" => Some((KeyCode::L, keycodes::KEY_L as u16)),
        "KEY_M" => Some((KeyCode::M, keycodes::KEY_M as u16)),
        "KEY_N" => Some((KeyCode::N, keycodes::KEY_N as u16)),
        "KEY_O" => Some((KeyCode::O, keycodes::KEY_O as u16)),
        "KEY_P" => Some((KeyCode::P, keycodes::KEY_P as u16)),
        "KEY_Q" => Some((KeyCode::Q, keycodes::KEY_Q as u16)),
        "KEY_R" => Some((KeyCode::R, keycodes::KEY_R as u16)),
        "KEY_S" => Some((KeyCode::S, keycodes::KEY_S as u16)),
        "KEY_T" => Some((KeyCode::T, keycodes::KEY_T as u16)),
        "KEY_U" => Some((KeyCode::U, keycodes::KEY_U as u16)),
        "KEY_V" => Some((KeyCode::V, keycodes::KEY_V as u16)),
        "KEY_W" => Some((KeyCode::W, keycodes::KEY_W as u16)),
        "KEY_X" => Some((KeyCode::X, keycodes::KEY_X as u16)),
        "KEY_Y" => Some((KeyCode::Y, keycodes::KEY_Y as u16)),
        "KEY_Z" => Some((KeyCode::Z, keycodes::KEY_Z as u16)),

        // Top-row digits
        "KEY_0" => Some((KeyCode::Key0, keycodes::KEY_0 as u16)),
        "KEY_1" => Some((KeyCode::Key1, keycodes::KEY_1 as u16)),
        "KEY_2" => Some((KeyCode::Key2, keycodes::KEY_2 as u16)),
        "KEY_3" => Some((KeyCode::Key3, keycodes::KEY_3 as u16)),
        "KEY_4" => Some((KeyCode::Key4, keycodes::KEY_4 as u16)),
        "KEY_5" => Some((KeyCode::Key5, keycodes::KEY_5 as u16)),
        "KEY_6" => Some((KeyCode::Key6, keycodes::KEY_6 as u16)),
        "KEY_7" => Some((KeyCode::Key7, keycodes::KEY_7 as u16)),
        "KEY_8" => Some((KeyCode::Key8, keycodes::KEY_8 as u16)),
        "KEY_9" => Some((KeyCode::Key9, keycodes::KEY_9 as u16)),

        // Numpad
        "KEY_NUMPAD0" => Some((KeyCode::Kp0,        keycodes::KEY_NUMPAD0 as u16)),
        "KEY_NUMPAD1" => Some((KeyCode::Kp1,        keycodes::KEY_NUMPAD1 as u16)),
        "KEY_NUMPAD2" => Some((KeyCode::Kp2,        keycodes::KEY_NUMPAD2 as u16)),
        "KEY_NUMPAD3" => Some((KeyCode::Kp3,        keycodes::KEY_NUMPAD3 as u16)),
        "KEY_NUMPAD4" => Some((KeyCode::Kp4,        keycodes::KEY_NUMPAD4 as u16)),
        "KEY_NUMPAD5" => Some((KeyCode::Kp5,        keycodes::KEY_NUMPAD5 as u16)),
        "KEY_NUMPAD6" => Some((KeyCode::Kp6,        keycodes::KEY_NUMPAD6 as u16)),
        "KEY_NUMPAD7" => Some((KeyCode::Kp7,        keycodes::KEY_NUMPAD7 as u16)),
        "KEY_NUMPAD8" => Some((KeyCode::Kp8,        keycodes::KEY_NUMPAD8 as u16)),
        "KEY_NUMPAD9" => Some((KeyCode::Kp9,        keycodes::KEY_NUMPAD9 as u16)),
        "KEY_ADD"      => Some((KeyCode::KpAdd,      keycodes::KEY_ADD      as u16)),
        "KEY_SUBTRACT" => Some((KeyCode::KpSubtract, keycodes::KEY_SUBTRACT as u16)),
        "KEY_MULTIPLY" => Some((KeyCode::KpMultiply, keycodes::KEY_MULTIPLY as u16)),
        "KEY_DIVIDE"   => Some((KeyCode::KpDivide,   keycodes::KEY_DIVIDE   as u16)),
        "KEY_DECIMAL"  => Some((KeyCode::KpDecimal,  keycodes::KEY_DECIMAL  as u16)),
        "KEY_RETURN"   => Some((KeyCode::KpEnter,    keycodes::KEY_RETURN   as u16)),

        // Function keys
        "KEY_F1"  => Some((KeyCode::F1,  keycodes::KEY_F1  as u16)),
        "KEY_F2"  => Some((KeyCode::F2,  keycodes::KEY_F2  as u16)),
        "KEY_F3"  => Some((KeyCode::F3,  keycodes::KEY_F3  as u16)),
        "KEY_F4"  => Some((KeyCode::F4,  keycodes::KEY_F4  as u16)),
        "KEY_F5"  => Some((KeyCode::F5,  keycodes::KEY_F5  as u16)),
        "KEY_F6"  => Some((KeyCode::F6,  keycodes::KEY_F6  as u16)),
        "KEY_F7"  => Some((KeyCode::F7,  keycodes::KEY_F7  as u16)),
        "KEY_F8"  => Some((KeyCode::F8,  keycodes::KEY_F8  as u16)),
        "KEY_F9"  => Some((KeyCode::F9,  keycodes::KEY_F9  as u16)),
        "KEY_F10" => Some((KeyCode::F10, keycodes::KEY_F10 as u16)),
        "KEY_F11" => Some((KeyCode::F11, keycodes::KEY_F11 as u16)),
        "KEY_F12" => Some((KeyCode::F12, keycodes::KEY_F12 as u16)),

        // Modifiers
        "KEY_SHIFT"        => Some((KeyCode::LeftShift,   keycodes::KEY_SHIFT   as u16)),
        "KEY_CONTROL"      => Some((KeyCode::LeftControl, keycodes::KEY_CONTROL as u16)),
        "KEY_ALT"          => Some((KeyCode::LeftAlt,     keycodes::KEY_ALT     as u16)),
        "KEY_META"|"KEY_WIN" => Some((KeyCode::LeftSuper, keycodes::KEY_META    as u16)),

        // Navigation
        "KEY_UP"        => Some((KeyCode::Up,       keycodes::KEY_UP        as u16)),
        "KEY_DOWN"      => Some((KeyCode::Down,     keycodes::KEY_DOWN      as u16)),
        "KEY_LEFT"      => Some((KeyCode::Left,     keycodes::KEY_LEFT      as u16)),
        "KEY_RIGHT"     => Some((KeyCode::Right,    keycodes::KEY_RIGHT     as u16)),
        "KEY_HOME"      => Some((KeyCode::Home,     keycodes::KEY_HOME      as u16)),
        "KEY_END"       => Some((KeyCode::End,      keycodes::KEY_END       as u16)),
        "KEY_PAGE_UP"   => Some((KeyCode::PageUp,   keycodes::KEY_PAGE_UP   as u16)),
        "KEY_PAGE_DOWN" => Some((KeyCode::PageDown, keycodes::KEY_PAGE_DOWN as u16)),
        "KEY_INSERT"    => Some((KeyCode::Insert,   keycodes::KEY_INSERT    as u16)),
        "KEY_DELETE"    => Some((KeyCode::Delete,   keycodes::KEY_DELETE    as u16)),

        // Common keys
        "KEY_ENTER"       => Some((KeyCode::Enter,       keycodes::KEY_ENTER       as u16)),
        "KEY_ESCAPE"      => Some((KeyCode::Escape,      keycodes::KEY_ESCAPE      as u16)),
        "KEY_BACK_SPACE"  => Some((KeyCode::Backspace,   keycodes::KEY_BACK_SPACE  as u16)),
        "KEY_TAB"         => Some((KeyCode::Tab,         keycodes::KEY_TAB         as u16)),
        "KEY_SPACE"       => Some((KeyCode::Space,       keycodes::KEY_SPACE       as u16)),
        "KEY_CAPS_LOCK"   => Some((KeyCode::CapsLock,    keycodes::KEY_CAPS_LOCK   as u16)),
        "KEY_NUM_LOCK"    => Some((KeyCode::NumLock,     keycodes::KEY_NUM_LOCK    as u16)),
        "KEY_SCROLL_LOCK" => Some((KeyCode::ScrollLock,  keycodes::KEY_SCROLL_LOCK as u16)),
        "KEY_PAUSE"       => Some((KeyCode::Pause,       keycodes::KEY_PAUSE       as u16)),
        "KEY_PRINTSCREEN" => Some((KeyCode::PrintScreen, keycodes::KEY_PRINTSCREEN as u16)),

        // Punctuation / symbols
        "KEY_MINUS"|"KEY_HYPHEN_MINUS" => Some((KeyCode::Minus,        keycodes::KEY_HYPHEN_MINUS  as u16)),
        "KEY_EQUALS"                   => Some((KeyCode::Equal,        keycodes::KEY_EQUALS        as u16)),
        "KEY_OPEN_BRACKET"             => Some((KeyCode::LeftBracket,  keycodes::KEY_OPEN_BRACKET  as u16)),
        "KEY_CLOSE_BRACKET"            => Some((KeyCode::RightBracket, keycodes::KEY_CLOSE_BRACKET as u16)),
        "KEY_BACK_SLASH"               => Some((KeyCode::Backslash,    keycodes::KEY_BACK_SLASH    as u16)),
        "KEY_SEMICOLON"                => Some((KeyCode::Semicolon,    keycodes::KEY_SEMICOLON     as u16)),
        "KEY_QUOTE"                    => Some((KeyCode::Apostrophe,   keycodes::KEY_QUOTE         as u16)),
        "KEY_BACK_QUOTE"               => Some((KeyCode::GraveAccent,  keycodes::KEY_BACK_QUOTE    as u16)),
        "KEY_COMMA"                    => Some((KeyCode::Comma,        keycodes::KEY_COMMA         as u16)),
        "KEY_PERIOD"                   => Some((KeyCode::Period,       keycodes::KEY_PERIOD        as u16)),
        "KEY_SLASH"                    => Some((KeyCode::Slash,        keycodes::KEY_SLASH         as u16)),

        _ => {
            eprintln!("[warn] Unknown keycode name in config: '{}'", name);
            None
        }
    }
}

struct KeyBox {
    vk_code:    u16,     // Win32 VK code for GetAsyncKeyState (unfocused detection)
    key:        KeyCode, // macroquad KeyCode (fallback / future Linux support)
    x:          f32,
    y:          f32,
    w:          f32,
    h:          f32,
    color_up:   Color,
    color_down: Color,
    label:      String,
}

#[macroquad::main(window_conf)]
async fn main() {
    // Parse board.conf
    let board_config = fs::read_to_string("board.conf")
        .expect("Could not read board.conf");

    let mut keys: Vec<KeyBox> = Vec::new();

    for (line_no, raw_line) in board_config.lines().enumerate() {
        let line = raw_line.trim();

        // Skip comments and blank lines
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 8 {
            eprintln!("[warn] Line {}: expected 8 fields, got {} — skipping", line_no + 1, parts.len());
            continue;
        }

        let key_name   = parts[0].trim();
        let x: f32     = parts[1].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad x", line_no+1); 0.0 });
        let y: f32     = parts[2].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad y", line_no+1); 0.0 });
        let w: f32     = parts[3].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad w", line_no+1); 50.0 });
        let h: f32     = parts[4].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad h", line_no+1); 50.0 });
        let color_up   = parse_color(parts[5].trim());
        let color_down = parse_color(parts[6].trim());
        let label      = parts[7].trim().trim_matches('"').to_string();

        let Some((keycode, vk_code)) = parse_keycode(key_name) else {
            eprintln!("[warn] Line {}: skipping unknown key '{}'", line_no + 1, key_name);
            continue;
        };

        keys.push(KeyBox { vk_code, key: keycode, x, y, w, h, color_up, color_down, label });
    }

    println!("[info] Loaded {} key(s) from board.conf", keys.len());

    // Render loop
    loop {
        clear_background(BLACK);

        for k in &keys {
            #[cfg(windows)]
            let pressed = is_key_held(k.vk_code);
            #[cfg(not(windows))]
            let pressed = is_key_down(k.key);

            let color = if pressed { k.color_down } else { k.color_up };

            draw_rectangle(k.x, k.y, k.w, k.h, color);

            // Thin border so keys are always visible
            draw_rectangle_lines(k.x, k.y, k.w, k.h, 1.5, DARKGRAY);

            // Center the label inside the key box
            let font_size = 18.0;
            let text_w = measure_text(&k.label, None, font_size as u16, 1.0).width;
            draw_text(
                &k.label,
                k.x + (k.w - text_w) / 2.0,
                k.y + k.h / 2.0 + font_size / 3.0,
                font_size,
                WHITE,
            );
        }

        next_frame().await;
    }
}
