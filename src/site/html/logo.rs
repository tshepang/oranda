use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::config::Config;
use crate::errors::*;

fn get_remote_logo(logo: &String, dist_dir: &String) -> Result<PathBuf> {
    let resp = reqwest::blocking::get(logo);

    match resp {
        Err(_) => {
            return Err(OrandaError::RequestFailed {
                url: logo.to_string(),
                resource: String::from("Logo"),
            });
        }
        Ok(img) => {
            let logo_path = Path::join(Path::new(dist_dir), Path::new(&logo).file_name().unwrap());

            let mut logo_file = File::create(&logo_path)?;
            logo_file.write_all(&img.bytes().unwrap())?;

            Ok(logo_path)
        }
    }
}

pub fn get_logo(config: &Config) -> Result<Option<&PathBuf>> {
    if !config.logo.is_none() {
        return Ok(None);
    }

    let mut logo_url = None;
    let logo = Option::expect(config.logo, "hmm, shouldn't really happen");

    if logo.starts_with("http") {
        let remote_fetched_path = get_remote_logo(&logo, &config.dist_dir).unwrap();
        logo_url = Some(&remote_fetched_path);
    } else {
        if !Path::new(&logo).exists() {
            return Err(OrandaError::FileNotFound {
                filedesc: "Logo".to_owned(),
                path: logo.to_owned(),
            });
        }

        let new_path = Path::join(
            Path::new(&config.dist_dir),
            Path::new(&logo).file_name().unwrap(),
        );
        fs::copy(&logo, &new_path).unwrap();
        logo_url = Some(&new_path);
    }

    Ok(&logo_url)
}
