use std::env;
use std::fs;
use std::io;

pub fn install() -> io::Result<()> {
    let bin_dir = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "home directory not found"))?
        .join(".local/bin");

    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir)?;
    }

    // Try multiple locations to find shell scripts
    let source_dirs = vec![
        // 1. Current working directory (when running from source)
        env::current_dir()?.join("shell"),
        // 2. Relative to executable (when installed)
        env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .map(|p| p.join("shell"))
            .unwrap_or_default(),
    ];

    let mut source_dir = None;
    for dir in &source_dirs {
        if dir.exists() && dir.is_dir() {
            source_dir = Some(dir.clone());
            break;
        }
    }

    let source_dir = source_dir
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "shell directory not found"))?;

    let wrappers = ["frec_editor.sh", "frec_editor_fzf.sh"];

    for wrapper in wrappers {
        let src = source_dir.join(wrapper);
        let dst = bin_dir.join(wrapper.trim_end_matches(".sh"));

        if src.exists() {
            fs::copy(&src, &dst)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&dst)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&dst, perms)?;
            }
            println!("Installed: {}", dst.display());
        } else {
            eprintln!("Warning: {} not found, skipping", src.display());
        }
    }

    println!("\nAdd ~/.local/bin to your PATH if not already added:");
    println!("  export PATH=\"$HOME/.local/bin:$PATH\"");
    println!("\nThen add to your shell config:");
    println!("  eval \"$(frec init zsh)\"  # or bash");

    Ok(())
}
