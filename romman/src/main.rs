use chrono::{DateTime, Local};
use slint::{StandardListViewItem, VecModel};
use slint::*;
use std::fs;
use std::rc::Rc;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();

    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    for (index, item) in fs::read_dir("./").unwrap().into_iter().enumerate() {
        let items = Rc::new(VecModel::default());

        let entry = item.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        let modified: DateTime<Local> = metadata.modified().unwrap().into();

        let filename = StandardListViewItem::from(slint::format!("{}", path.display()));
        items.push(filename.clone());

        items.push("nes".into());

        items.push("synced".into());

        let filedate = StandardListViewItem::from(slint::format!("{}", modified.format("%Y-%m-%d %H:%M:%S")));
        items.push(filedate.clone());

        row_data.push(items.clone().into());
    }

    main_window.global::<TableAdapter>().set_row_data(row_data.clone().into());

    main_window.run().unwrap();
}
