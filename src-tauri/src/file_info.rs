use std::{
    fs::{read_to_string, write},
    io,
};

use platform_dirs::UserDirs;

pub struct GitConfig {
    git_file_path: String,
    ssh_file_path: String,
    npm_file_path: String,
}

impl GitConfig {
    pub fn new(handle: tauri::AppHandle) -> Self {
        let mut desktop_dir = String::from("");
        let mut git_file_path = String::from("");
        let mut ssh_file_path = String::from("");
        let mut npm_file_path = String::from("");

        if cfg!(target_os = "windows") {
            desktop_dir = UserDirs::new()
                .unwrap()
                .desktop_dir
                .to_str()
                .unwrap()
                .replace("\\Desktop", "");
            git_file_path = format!("{desktop_dir}\\.gitconfig");
            npm_file_path = format!("{desktop_dir}\\.npmrc");
        } else if cfg!(target_os = "macos") {
            desktop_dir = UserDirs::new()
                .unwrap()
                .desktop_dir
                .to_str()
                .unwrap()
                .replace("/Desktop", "");
            git_file_path = format!("{desktop_dir}/.gitconfig");
            npm_file_path = format!("{desktop_dir}/.npmrc");
        }
        let ssh = handle
            .path_resolver()
            .resolve_resource("../resources/ssh-forward.config")
            .expect("failed to resolve resource");
        ssh_file_path = format!("{}", ssh.to_string_lossy());

        GitConfig {
            git_file_path,
            ssh_file_path,
            npm_file_path,
        }
    }

    pub fn read_file(self, file_type: u8) -> String {
        let mut file_path = String::from("");

        match file_type {
            1 => file_path = self.git_file_path,
            2 => file_path = self.npm_file_path,
            _ => file_path = self.ssh_file_path,
        }
        read_to_string(file_path).unwrap()
    }

    pub fn write_file(self, ctx: &str, file_type: u8) -> io::Result<()> {
        let mut file_path = String::from("");

        match file_type {
            1 => file_path = self.git_file_path,
            2 => file_path = self.npm_file_path,
            _ => file_path = self.ssh_file_path,
        }
        write(file_path, ctx)
    }
}
