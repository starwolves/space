use std::{fs, path::Path};

use bevy::log::info;
use bevy::log::warn;
use bevy::prelude::{App, Commands, Plugin, Resource, Startup};
use cargo_metadata::Metadata;

#[derive(Resource)]
pub struct MetadataResource {
    pub commit: Option<String>,
    pub data: Option<Metadata>,
    pub is_binary_run: bool,
}

pub(crate) fn load_metadata(mut commands: Commands) {
    let commit_path = Path::new("data").join("commit.txt");
    let commit;
    if commit_path.exists() {
        commit = Some(fs::read_to_string(commit_path).unwrap());
    } else {
        commit = None;
    }

    let metadata_path = Path::new("data").join("cargo_metadata.json");
    let mut cmd = cargo_metadata::MetadataCommand::new();
    info!("Space Frontiers binaries, codebase and assets fall under proprietary licenses. See files: LICENSE, LICENSE_ASSETS.");

    let cargo_metadata;
    let is_binary_run = metadata_path.exists();
    if is_binary_run {
        let dats = fs::read_to_string(metadata_path).unwrap();
        cargo_metadata = serde_json::from_str(&dats).unwrap();
    } else {
        cmd.manifest_path(Path::new("Cargo.toml"));
        match cmd.exec() {
            Ok(r) => {
                info!(
                    "Running {} crates ({} internal)",
                    r.packages.len(),
                    r.workspace_members.len()
                );
                let mut bevy_version_option = None;
                let mut sf_version_option = None;

                for package in r.packages.iter() {
                    if package.name == "bevy" {
                        bevy_version_option = Some(package.version.clone());
                    } else if package.name == "app" {
                        sf_version_option = Some(package.version.clone());
                    }
                }
                if sf_version_option.is_none() || bevy_version_option.is_none() {
                    warn!("Couldnt find bevy or app packages");
                    return;
                }

                let sf_version = sf_version_option.unwrap();
                let bevy_version = bevy_version_option.unwrap();
                info!(
                    "Bevy v{}.{}.{}",
                    bevy_version.major, bevy_version.minor, bevy_version.patch
                );
                info!(
                    "Space Frontiers v{}.{}.{}",
                    sf_version.major, sf_version.minor, sf_version.patch,
                );
                cargo_metadata = Some(r);
            }
            Err(_rr) => {
                warn!("No metadata supplied.");
                cargo_metadata = None;
            }
        }
    }

    match &commit {
        Some(c) => {
            info!("Commit: {}", c);
        }
        None => {}
    }

    commands.insert_resource(MetadataResource {
        commit,
        data: cargo_metadata.clone(),
        is_binary_run,
    });
}

pub struct MetadataPlugin;

impl Plugin for MetadataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_metadata);
    }
}
