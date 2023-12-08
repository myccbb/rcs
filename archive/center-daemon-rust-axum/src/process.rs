// 23.03.28 just stop and restart pm2 app.
// get process info (pid, listen port) by ss -nplt
// step 1, record prev process info in process info table.
// step 2, start new process with new port.
// step 3, change nginx config to new port, and reload nginx.
// step 4, check new process is running.
// step 5, stop prev process and update process into table process pid and port.

pub enum Error {
    ProcessNotFound,
    Failed,
}

pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub listen_ip: String,
    pub listen_port: u16,
}

pub fn get_process_info(process_name: String) -> Result<ProcessInfo, Error> {
    let child = std::process::Command::new("cargo")
        .current_dir("/usr/bin/bash")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .args(&["build", "--release"])
        .spawn();
    if let Err(e) = child {
        tracing::error!("build center failed: {}", e);
        return Err(Error::ProcessNotFound);
    }
    Err(Error::ProcessNotFound)
}
