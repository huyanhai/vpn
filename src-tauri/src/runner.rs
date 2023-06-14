pub mod Runner {
    use std::{
        process::{Command, Stdio},
        thread, time,
    };
    pub enum VpnStatus {
        OPEN,
        CLOSE,
    }
    pub static mut CLOSE_STATUS: VpnStatus = VpnStatus::CLOSE;

    pub struct Running {
        file_path: String,
    }

    impl Running {
        pub fn new(handle: tauri::AppHandle) -> Running {
            let mut file_path = String::from("");
            let resource_path = handle
                .path_resolver()
                .resolve_resource("../resources/SSHProxy-1.5.jar")
                .expect("failed to resolve resource");
            if cfg!(target_os = "windows") {
                file_path = format!("{}", resource_path.to_string_lossy().replace("\\\\?\\", ""));
            } else {
                file_path = format!("{}", resource_path.to_string_lossy());
            }

            Running { file_path }
        }
        pub fn run(self) {
            thread::spawn(|| {
                let mut child = Command::new("java")
                    .arg("-jar")
                    .arg(self.file_path)
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();

                unsafe {
                    CLOSE_STATUS = VpnStatus::OPEN;
                }

                loop {
                    thread::sleep(time::Duration::from_millis(500));
                    unsafe {
                        match CLOSE_STATUS {
                            VpnStatus::CLOSE => {
                                let _ = child.kill();
                                CLOSE_STATUS = VpnStatus::CLOSE;
                                println!("关闭进程id:{:?}", child.id());
                                break;
                            }
                            VpnStatus::OPEN => (),
                        }
                    };
                }
            });
        }
    }
}
