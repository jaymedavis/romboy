use crate::settings::Settings;
use rom::Rom;
use slint::*;
use slint::VecModel;
use std::env;
use std::format;
use std::fs;
use std::path::Path;
use std::rc::Rc;

mod settings;
mod rom;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();

    // get the app settings
    let settings = Settings::new().expect("Failed to load settings.toml");

    // hold our processed file information
    let mut temp_rows: Vec<TableRow> = Vec::new();

    // read the files in the zips directory
    for (_index, item) in fs::read_dir(settings.zips_path()).unwrap().into_iter().enumerate() {
        let path = item.unwrap().path();

        // validate the zip file is a valid rom
        let rom = Rom::new(&path);
        if rom.is_err() {
            println!("error loading rom: {}", rom.err().unwrap().message);
            continue;
        }

        // validate the platform
        let platform = Rom::get_platform_for_extension(&settings, rom.clone().unwrap());
        if platform.is_err() {
            println!("error validating platform: {}", platform.err().unwrap().message);
            continue;
        }

        // push the rows to the main window
        let status = if Rom::exists(&settings, rom.clone().unwrap()) { "In Library" } else { "Not In Library" };

        let table_entry = TableRow {
            filename: SharedString::from(&rom.unwrap().name),
            status: SharedString::from(status),
            platform: asset_image(platform.unwrap()),
        };

        temp_rows.push(table_entry);

        // create the file
        // Rom::create_file(&settings, rom.clone().unwrap());
    }

    //  add our temp rows to the main window
    let table_rows: Rc<VecModel<TableRow>> = Rc::new(VecModel::from(temp_rows));
    main_window.set_rows(table_rows.into());

    // tie up our events
    main_window.global::<TableRowBackend>().on_clicked(|id| {
		println!("On button clicked: id={}", id);
	});

    main_window.run().unwrap();
}

fn asset_image(platform: String) -> Image {
    let asset_path = format!("assets/{}.png", platform);
    Image::load_from_path(Path::new(&asset_path)).unwrap()
}
