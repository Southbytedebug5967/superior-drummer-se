//! Windows registry read helpers for Superior Drummer Se.

pub const APP_KEY: &str = "Software\\Superior\\Superior Drummer Se";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Superior Drummer Se";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Superior Drummer Se") },
        RegValue { name: "Version",     value: String::from("v2.1.16") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Superior Drummer Se") },
        RegValue { name: "Publisher",       value: String::from("Superior") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Superior Drummer Se\\uninstall.exe\"") },
    ]
}
