// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::Error;
use osu_lazer_manager::{
    config::{get_default_version as config_get_default_version, set_default_version},
    constants::REPOSITORY,
    fetcher::{
        appimage::{fetch_appimage, get_appimage_path},
        desktop::{fetch_desktop, get_desktop_cache_path, get_desktop_path, process_template},
        icon::{fetch_icon, get_icon_path},
    },
    paths::get_directory_path,
    utils::{directories::create_missing_directories, version::get_latest_version},
};
use std::{
    fs::{read_to_string, write},
    process::Command,
};
use tauri::{Runtime, Window};

mod error;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_versions,
            get_default_version,
            is_installed,
            is_latest_installed,
            launch,
            install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_versions() -> Result<Vec<String>, Error> {
    let repository = get_directory_path()
        .map_err(|e| anyhow::anyhow!("Failed to get osu-lazer-manager data directory: {}", e))?
        .join(REPOSITORY);

    create_missing_directories()
        .map_err(|e| anyhow::anyhow!("Failed to create directories: {}", e))?;

    let latest_version = get_latest_version().await?;

    let mut versions: Vec<String> = repository
        .read_dir()
        .map_err(|e| anyhow::anyhow!("Failed to read directory: {}", e))?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    if !versions.contains(&latest_version) {
        versions.push(latest_version);
    }

    versions.sort();

    Ok(versions)
}

#[tauri::command]
async fn get_default_version() -> Result<String, Error> {
    match config_get_default_version() {
        Ok(version) => Ok(version),
        Err(_) => Ok(get_latest_version().await?),
    }
}

#[tauri::command]
async fn is_installed(version: String) -> Result<bool, Error> {
    let repository = get_directory_path()
        .map_err(|e| anyhow::anyhow!("Failed to get osu-lazer-manager data directory: {}", e))?
        .join(REPOSITORY);

    if !repository.exists() {
        return Ok(false);
    }

    if repository.read_dir()?.count() == 0 {
        return Ok(false);
    }

    let versions = repository
        .read_dir()
        .map_err(|e| anyhow::anyhow!("Failed to read directory: {}", e))?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    Ok(versions.contains(&version))
}

#[tauri::command]
async fn is_latest_installed() -> Result<bool, Error> {
    is_installed(get_latest_version().await?).await
}

#[tauri::command]
fn launch(version: String) -> Result<(), Error> {
    let repository = get_directory_path()
        .map_err(|e| anyhow::anyhow!("Failed to get osu-lazer-manager data directory: {}", e))?
        .join(REPOSITORY)
        .join(&version);

    set_default_version(&version)?;

    Command::new(repository).spawn()?;

    Ok(())
}

#[tauri::command]
async fn install<R: Runtime>(window: Window<R>, version: String) -> Result<(), Error> {
    create_missing_directories()
        .map_err(|e| anyhow::anyhow!("Failed to create directories: {}", e))?;

    get_appimage(false, &version, window)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get appimage: {}", e))?;
    get_icon()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get icon: {}", e))?;
    get_desktop(&version)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get desktop: {}", e))?;

    Ok(())
}

async fn get_appimage<R: Runtime>(
    force_install: bool,
    specific_version: &str,
    window: Window<R>,
) -> anyhow::Result<()> {
    fetch_appimage(
        force_install,
        specific_version,
        |size| window.emit("size", size).unwrap(),
        |downloaded| window.emit("progress", downloaded).unwrap(),
    )
    .await?;

    set_default_version(specific_version)?;

    Ok(())
}

async fn get_icon() -> anyhow::Result<()> {
    if fetch_icon(|_| {}, |_| {}).await.is_err() {
        match get_icon_path()?.exists() {
            true => println!("Will use existing icon."),
            false => return Err(anyhow::anyhow!("Failed to download icon.")),
        }
    }

    Ok(())
}

async fn get_desktop(version: &str) -> anyhow::Result<()> {
    if fetch_desktop(version, |_| {}, |_| {}).await.is_err() {
        let desktop_template = read_to_string(get_desktop_cache_path()?)?;

        let output_desktop = process_template(
            &desktop_template,
            get_icon_path()?.to_str().unwrap(),
            get_appimage_path(version)?.to_str().unwrap(),
        );

        write(get_desktop_path()?, output_desktop)?;
    }

    Ok(())
}
