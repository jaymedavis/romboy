use std::{fs, path::PathBuf};
use serde_derive::Deserialize;
use std::io::BufReader;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Rom {
    pub(crate) name: String,
    pub(crate) extension: String,
    pub(crate) status: String
}

impl Rom {
    pub(crate) fn new(path: &PathBuf) -> Rom {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut archive = zip::ZipArchive::new(reader).unwrap();
        let mut result = Rom::default();

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => {
                    println!("processing {}", path.display());
                    path
                }
                None => {
                    println!("{} has a suspicious path", file.name());
                    continue;
                }
            };

            if !file.is_dir() {
                let extension = outpath.extension().unwrap().to_str().unwrap();

                result = Rom {
                    name: outpath.file_stem().unwrap().to_str().unwrap().to_string(),
                    extension: extension.to_string(),
                    status: "Synced".to_string()
                };
            }
        }

        result
    }
}
