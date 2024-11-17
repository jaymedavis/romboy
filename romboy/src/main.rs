use crate::rom::Rom;
use crate::settings::Settings;
use log::{trace, warn};
use slint::*;
use slint::VecModel;
use std::env;
use std::format;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::thread;

mod logging;
mod rom;
mod settings;

slint::include_modules!();

fn main() {
    let main_window = Rc::new(MainWindow::new().unwrap());

    // get the app settings
    let settings = Settings::new().expect("Failed to load settings.toml");

    // initialize logging
    logging::init(&settings);
    trace!("Starting Romboy");

    // draw the main window
    draw(&main_window, &settings);

    // tie up the events
    events(&main_window, &settings);

    main_window.run().unwrap();
}

fn draw(main_window: &MainWindow, settings: &Settings) {
    // hold our processed file information
    let mut temp_rows: Vec<TableRow> = Vec::new();

    // read the files in the zips directory
    for (_index, item) in fs::read_dir(settings.zips_path()).unwrap().into_iter().enumerate() {
        let path = item.unwrap().path();

        // validate the zip file is a valid rom
        let rom = Rom::new(&path);
        if rom.is_err() {
            warn!("can't load rom: {}", rom.err().unwrap().message);
            continue;
        }

        // validate the platform
        let platform = Rom::get_platform_for_extension(&settings, rom.clone().unwrap());
        if platform.is_err() {
            warn!("can't find platform: {}", platform.err().unwrap().message);
            continue;
        }

        // push the rows to the main window
        let in_library = Rom::exists(&settings, rom.clone().unwrap());
        let status = if in_library { "In Library" } else { "Not In Library" };

        let table_entry = TableRow {
            filename: SharedString::from(&rom.unwrap().name),
            in_library: in_library,
            platform: asset_image(platform.unwrap()),
            status: SharedString::from(status),
        };

        temp_rows.push(table_entry);
        main_window.set_percent(0.56);
    }

    //  add our temp rows to the main window
    let table_rows: Rc<VecModel<TableRow>> = Rc::new(VecModel::from(temp_rows));
    main_window.set_rows(table_rows.into());
}

fn asset_image(platform: String) -> Image {
    let asset_path = format!("assets/{}.png", platform);
    Image::load_from_path(Path::new(&asset_path)).unwrap()
}

fn events(main_window: &Rc<MainWindow>, settings: &Settings) {
    let main_window_clone = main_window.clone();
    let settings_clone = settings.clone();

    main_window.global::<TableRowBackend>().on_clicked(move |file_name: SharedString, delete_file: bool| {
        modify_file(&settings_clone, file_name.to_string(), delete_file);
        draw(&main_window_clone, &settings_clone);
	});
}

fn modify_file(settings: &Settings, file_name: String, delete_file: bool) {
    for (_index, item) in fs::read_dir(settings.zips_path()).unwrap().into_iter().enumerate() {
        let path = item.unwrap().path();

        let rom = Rom::new(&path);

        if !rom.is_err() {
            let rom = rom.unwrap();

            if rom.name == file_name {
                let settings_clone = settings.clone();
                let rom_clone = rom.clone();

                thread::spawn(move || {
                    Rom::modify_file(&settings_clone, rom_clone, delete_file);
                });
            }
        }
    }
}
