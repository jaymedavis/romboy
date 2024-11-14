use serde_derive::Deserialize;
use std::fs::{self, File};
use std::io::{self, copy, BufReader};
use std::path::{Path, PathBuf};

use crate::settings::Settings;

#[derive(Clone, Debug, Default)]
pub(crate) struct Rom {
    pub(crate) name: String,
    pub(crate) extension: String,
    pub(crate) path: PathBuf,
    // pub(crate) status: String
}

#[derive(Debug, Default)]
pub(crate) struct RomExtract {
    pub(crate) path: String,
    pub(crate) target: PathBuf,
}

#[derive(Clone, Debug)]
pub(crate) struct RomError {
    pub(crate) message: String,
}

impl Rom {
    pub(crate) fn new(path: &PathBuf) -> Result<Rom, RomError> {
        let mut archive = Self::get_archive(path);

        // if there's more or less than one file in the archive, return an error
        if archive.len() > 1 || archive.len() < 1 {
            return Err(RomError {
                message: format!("{} files in archive: {}, expecting one", archive.len(), path.file_name().unwrap().to_str().unwrap())
             });
        }

        // get the only file in the archive
        let file = archive.by_index(0).unwrap();

        if file.is_dir() {
            return Err(RomError {
                message: format!("the file in {} is a directory, expecting a rom", path.display())
            })
        }

        // make sure the file name is safe to extract
        match file.enclosed_name() {
            Some(p) => return Ok(
                Rom {
                    name: p.file_stem().unwrap().to_str().unwrap().to_string(),
                    extension: p.extension().unwrap().to_str().unwrap().to_string(),
                    path: path.to_path_buf(),
                }
            ),
            None => return Err(RomError {
                message: format!("not safe to extract as a path {}", file.enclosed_name().unwrap().to_str().unwrap())
            })
        };
    }

    pub(crate) fn get_platform_for_extension(settings: &Settings, rom: Rom) -> Result<String, RomError> {
        // get the platform based on the extension
        for (platform, extensions) in &settings.platforms {
            if extensions.contains(&rom.extension) {
                return Ok(platform.to_owned());
            }
        }

        return Err(RomError {
            message: format!("support for {} is not available", rom.extension)
        })
    }

    pub(crate) fn exists(settings: &Settings, rom: Rom) -> bool {
        let rom_extract = Self::rom_extract(settings, rom);

        // println!("rom path: {}", rom_extract.path);
        // println!("rom target: {}", rom_extract.target.display());

        rom_extract.target.exists()
    }

    pub(crate) fn create_file(settings: &Settings, rom: Rom) {
        let mut archive = Self::get_archive(&rom.path);
        let mut file = archive.by_index(0).unwrap();

        // get the path and target
        let rom_extract = Self::rom_extract(settings, rom);
        let path = Path::new(rom_extract.path.as_str());

        // create the platform directory if it doesn't exist
        if !path.exists() {
            std::fs::create_dir(path).unwrap();
        }

        // create the file by joining the extract path to the file name
        let target_path = path.join(file.name());

        if !target_path.exists() {
            // create the file and copy the contents from the archive
            let mut output_file = File::create(&target_path).unwrap();
            io::copy(&mut file, &mut output_file).unwrap();

            println!("creating file {}", target_path.display());
        }
    }

    fn get_archive(path: &PathBuf) -> zip::ZipArchive<BufReader<File>> {
        let zip_file = fs::File::open(path).unwrap();
        let reader = BufReader::new(zip_file);

        zip::ZipArchive::new(reader).unwrap()
    }

    fn rom_extract(settings: &Settings, rom: Rom) -> RomExtract {
        let mut archive = Self::get_archive(&rom.path);
        let file = archive.by_index(0).unwrap();

        let extract_target = format!("{}/{}", settings.roms_path(), Rom::get_platform_for_extension(settings, rom).unwrap());
        let extract_path = Path::new(extract_target.as_str());
        let target_path = extract_path.join(file.name());

        RomExtract {
            path: extract_target,
            target: target_path,
        }
    }
}
