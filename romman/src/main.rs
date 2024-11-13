use std::format;
use rom::Rom;
use crate::settings::Settings;
use slint::*;
use slint::VecModel;
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

    // capture our files temporarily
    let mut temp_rows: Vec<TableRow> = Vec::new();

    // table method
    // let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

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
        let asset_path = format!("assets/{}.png", &platform.unwrap());
        let image = Image::load_from_path(Path::new(&asset_path)).unwrap();

        let table_entry = TableRow {
            filename: SharedString::from(&rom.unwrap().name),
            status: SharedString::from("Synced"),
            platform: image,
        };

        temp_rows.push(table_entry);

        // create the file
        // Rom::create_file(&settings, rom.clone().unwrap());

        // table method
        // let items = Rc::new(VecModel::default());

        // let filename = StandardListViewItem::from(slint::format!("{}", path.file_name().unwrap().to_string_lossy().to_owned()));
        // items.push(filename);

        // items.push("nes".into());

        // let library_status = StandardListViewItem::from("Synced");
        // // library_status.color = Some("green".to_string());
        // items.push(library_status.into());

        // let filedate = StandardListViewItem::from(slint::format!("{}", modified.format("%Y-%m-%d %H:%M:%S")));
        // items.push(filedate.clone());

        // row_data.push(items.clone().into());
    }

    //  add our temp rows to the main window
    let table_rows: Rc<VecModel<TableRow>> = Rc::new(VecModel::from(temp_rows));
    main_window.set_rows(table_rows.into());

    // table method
    // main_window.global::<TableAdapter>().set_row_data(row_data.clone().into());

    main_window.run().unwrap();
}
