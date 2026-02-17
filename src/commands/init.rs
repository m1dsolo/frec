pub fn init_shell(shell: &str) -> String {
    match shell {
        "zsh" => include_str!("../../shell/zsh.sh").to_string(),
        "bash" => include_str!("../../shell/bash.sh").to_string(),
        _ => panic!("unsupported shell: {}", shell),
    }
}
