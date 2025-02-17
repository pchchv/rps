use minijinja::Value;
use std::process::Command;
use serde_derive::Serialize;

#[derive(Serialize)]
struct Output {
    code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>
}

pub fn command(command: String) -> Value {
    let result = match Command::new("sh").arg("-c").arg(&command).output() {
        Ok(result) => result,
        Err(err) => {
            return Value::from_serialize(Output {
                code: 999999,
                stdout: "Fail to execute command!".to_string().as_bytes().to_vec(),
                stderr: err.to_string().as_bytes().to_vec()
            });
        }
    };

    Value::from_serialize(Output {
        code: result.status.code().unwrap_or(0),
        stdout: result.stdout,
        stderr: result.stderr
    })
}