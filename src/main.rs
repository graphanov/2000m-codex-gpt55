mod sim;

use serde_json::{json, Value};
use sim::{Game, Input};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut game = Game::new(0);

    for line in stdin.lock().lines() {
        let response = match line {
            Ok(line) => handle_command(&mut game, line.trim()),
            Err(err) => json!({ "ok": false, "error": format!("stdin read error: {}", err) }),
        };

        if serde_json::to_writer(&mut stdout, &response).is_err() {
            break;
        }
        if writeln!(stdout).is_err() || stdout.flush().is_err() {
            break;
        }
    }
}

fn handle_command(game: &mut Game, line: &str) -> Value {
    let command: Value = match serde_json::from_str(line) {
        Ok(value) => value,
        Err(err) => return json!({ "ok": false, "error": format!("invalid JSON: {}", err) }),
    };

    let Some(cmd) = command.get("cmd").and_then(Value::as_str) else {
        return json!({ "ok": false, "error": "missing cmd" });
    };

    match cmd {
        "init" => {
            let seed = command.get("seed").and_then(Value::as_i64).unwrap_or(0);
            *game = Game::new(seed);
            json!({ "ok": true, "state": game.state() })
        }
        "reset" => {
            let seed = command.get("seed").and_then(Value::as_i64).unwrap_or(0);
            *game = Game::new(seed);
            json!({ "ok": true, "state": game.state() })
        }
        "state" => json!({ "ok": true, "state": game.state() }),
        "step" => {
            let input = parse_input(command.get("input"));
            game.step(input);
            json!({ "ok": true, "state": game.state() })
        }
        _ => json!({ "ok": false, "error": "unsupported command" }),
    }
}

fn parse_input(input: Option<&Value>) -> Input {
    let steer = input
        .and_then(|value| value.get("steer"))
        .and_then(Value::as_i64)
        .unwrap_or(0)
        .clamp(-1, 1) as i32;
    let boost = input
        .and_then(|value| value.get("boost"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let jump = input
        .and_then(|value| value.get("jump"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    Input { steer, boost, jump }
}
