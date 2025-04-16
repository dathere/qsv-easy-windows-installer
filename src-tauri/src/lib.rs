use tauri::Manager;
use tempfile::tempfile;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

#[tauri::command]
fn run_path_update(app_handle: tauri::AppHandle) {
    // Get app local data dir path
    let app_local_data_dir = app_handle.path().app_local_data_dir().unwrap();
    // Download qsv to bin dir if it doesn't exist and overwrite any existing qsv
    // Get the version of qsv
    let latest_release_endpoint = "https://api.github.com/repos/dathere/qsv/releases/latest";
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(latest_release_endpoint)
        .header(reqwest::header::USER_AGENT, "qsv easy installer")
        .send()
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();
    let release_version = res.get("name").unwrap().as_str().unwrap();
    // Download the zip file temporarily then extract the relevant qsvp file (we use qsvp instead of qsv for the broadest compatibility)
    let zip_download_url = format!("https://github.com/dathere/qsv/releases/download/{release_version}/qsv-{release_version}-x86_64-pc-windows-msvc.zip");
    let mut temp_zip_file = tempfile().unwrap();
    reqwest::blocking::get(zip_download_url)
        .unwrap()
        .copy_to(&mut temp_zip_file)
        .unwrap();
    let mut zip = zip::ZipArchive::new(temp_zip_file).unwrap();
    let mut qsvp = zip.by_name("qsvp.exe").unwrap();
    // Create a bin folder in app_local_data_dir if it doesn't exist
    let bin_dir = app_local_data_dir.join("bin");
    if !std::path::Path::exists(&bin_dir) {
        std::fs::create_dir(&bin_dir).unwrap();
    }
    // Write qsvp.exe to bin/qsv.exe
    let mut qsv_file = std::fs::File::create(bin_dir.join("qsv.exe")).unwrap();
    std::io::copy(&mut qsvp, &mut qsv_file).unwrap();
    // Add the bin dir to PATH
    let bin_dir_str = bin_dir.to_str().unwrap();
    // Get the current PATH
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (reg_key, _) = hkcu.create_subkey("Environment").unwrap();
    let path_var: String = reg_key.get_value("Path").unwrap();
    // Add bin dir to PATH
    let updated_path_var = format!("{bin_dir_str};{path_var}");
    reg_key.set_value("Path", &updated_path_var).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_path_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
