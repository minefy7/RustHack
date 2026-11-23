use macroquad::prelude::*;

#[macroquad::main("Hack")]
async fn main() {
    let mut input = String::new();
    let mut history = Vec::new();

    let mut hold_frames = 0;

    loop {
        while let Some(c) = get_char_pressed() {
            if !c.is_control() {
                input.push(c);
            }
        }
        if is_key_down(KeyCode::Backspace) {
            if hold_frames == 0 || hold_frames > 40 && hold_frames % 2 == 0 {
                input.pop();
            }
            hold_frames += 1;
        } else {
            hold_frames = 0;
        }
        if is_key_pressed(KeyCode::Enter) {
            let parts: Vec<&str> = input.split_whitespace().collect();
            history.push(input.clone());
            match parts[0] {
                "help" => history.push(String::from(
                    "Hello, im terminal. Commands: help, ip, whoami, echo",
                )),
                "ip" => history.push(String::from("192.168.67.67")),
                "whoami" => history.push(String::from("low-budget hacker.")),
                "echo" => {
                    if parts.len() > 1 {
                        history.push(parts[1..].join(" "));
                    } else {
                        history.push(String::from("Usage: echo <text>"));
                    }
                }
                _ => history.push(format!("Unknown command: {}", input)),
            }
            input.clear();
        }
        clear_background(BLACK);
        draw_text(
            &format!("> {input}|"),
            30.0,
            screen_height() - 30.0,
            32.0,
            GREEN,
        );
        let mut otstup = 36.0;
        for his in history.iter().rev() {
            draw_text(
                &format!("*> {his}"),
                30.0,
                screen_height() - 30.0 - otstup,
                32.0,
                GREEN,
            );
            otstup += 36.0;
        }
        next_frame().await
    }
}
