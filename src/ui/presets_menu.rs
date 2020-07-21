use gtk::prelude::*;

use crate::presets::manager;
use std::sync::{Arc, Mutex};

use gtk_resources::UIResource;

#[derive(UIResource, Debug)]
#[resource = "/com/github/wwmm/pulseeffects/ui/presets_menu.glade"]
struct WindowResource {
    widgets_grid: gtk::Grid,
    stack: gtk::Stack,
    output_listbox: gtk::ListBox,
    output_scrolled_window: gtk::ScrolledWindow,
    output_name: gtk::Entry,
    add_output: gtk::Button,
    import_output: gtk::Button,
    input_listbox: gtk::ListBox,
    input_scrolled_window: gtk::ScrolledWindow,
    input_name: gtk::Entry,
    add_input: gtk::Button,
    import_input: gtk::Button,
}

pub fn build_ui(button: &gtk::Button) -> gtk::Grid {
    let resources = WindowResource::load().unwrap();

    let output_scrolled_window = resources.output_scrolled_window;
    let input_listbox = resources.input_listbox;
    let output_listbox = resources.output_listbox;

    output_listbox.set_sort_func(Some(Box::new(on_listbox_sort)));
    input_listbox.set_sort_func(Some(Box::new(on_listbox_sort)));

    button.connect_clicked(move |obj| {
        let top_widget = obj
            .get_toplevel()
            .expect("Could not get presets menu top level widget");
        let height = top_widget.get_allocated_height() as f32;

        output_scrolled_window.set_max_content_height((0.7 * height) as i32);

        populate_listbox(&manager::PresetType::Input, &input_listbox);

        populate_listbox(&manager::PresetType::Output, &output_listbox);
    });

    return resources.widgets_grid;
}

fn on_listbox_sort(row1: &gtk::ListBoxRow, row2: &gtk::ListBoxRow) -> i32 {
    let mut names = Vec::new();
    let name1 = row1.get_widget_name().expect("Could not get widget name");
    let name2 = row2.get_widget_name().expect("Could not get widget name");

    names.push(&name1);
    names.push(&name2);

    names.sort();
    if name1 == *names[0] {
        return -1;
    }
    if name2 == *names[0] {
        return 1;
    }
    return 0;
}

fn populate_listbox(preset_type: &manager::PresetType, listbox: &gtk::ListBox) {
    let children = listbox.get_children();

    for child in children {
        listbox.remove(&child);
    }

    let presets_manager = manager::Manager::new();

    let names = presets_manager.get_names(preset_type);

    for name in names {
        let builder =
            gtk::Builder::new_from_resource("/com/github/wwmm/pulseeffects/ui/preset_row.glade");

        let row: gtk::ListBoxRow = builder
            .get_object("preset_row")
            .expect("builder could not get the widget: preset_row");

        let apply_btn: gtk::Button = builder
            .get_object("apply")
            .expect("builder could not get the widget: apply");

        let save_btn: gtk::Button = builder
            .get_object("save")
            .expect("builder could not get the widget: save");

        let remove_btn: gtk::Button = builder
            .get_object("remove")
            .expect("builder could not get the widget: remove");

        let label: gtk::Label = builder
            .get_object("name")
            .expect("builder could not get the widget: name");

        let autoload_btn: gtk::Button = builder
            .get_object("autoload")
            .expect("builder could not get the widget: autoload");

        row.set_widget_name(name.as_str());

        label.set_text(name.as_str());

        // if (is_autoloaded(preset_type, name)) {
        //     autoload_btn->set_active(true);
        // }

        {
            let presets_manager = Arc::new(Mutex::new(presets_manager.clone()));
            let preset_type = (*preset_type).clone();
            let name = name.clone();

            apply_btn.connect_clicked(move |_btn| {
                // settings->set_string("last-used-preset", row->get_name());

                presets_manager.lock().unwrap().load(&preset_type, &name);
            });
        }

        {
            let presets_manager = Arc::new(Mutex::new(presets_manager.clone()));
            let preset_type = (*preset_type).clone();
            let name = name.clone();

            save_btn.connect_clicked(move |_btn| {
                presets_manager.lock().unwrap().save(&preset_type, &name);
            });
        }

        remove_btn.connect_clicked(|obj| {
            // app->presets_manager->remove(preset_type, name);

            // populate_listbox(preset_type);
        });

        autoload_btn.connect_clicked(|obj| {
            // if (preset_type == PresetType::output) {
            //     auto dev_name = build_device_name(preset_type, app->pm->server_info.default_sink_name);
            //     if (autoload_btn->get_active()) {
            //       app->presets_manager->add_autoload(dev_name, name);
            //     } else {
            //       app->presets_manager->remove_autoload(dev_name, name);
            //     }
            //   } else {
            //     auto dev_name = build_device_name(preset_type, app->pm->server_info.default_source_name);
            //     if (autoload_btn->get_active()) {
            //       app->presets_manager->add_autoload(dev_name, name);
            //     } else {
            //       app->presets_manager->remove_autoload(dev_name, name);
            //     }
            //   }
            //   populate_listbox(preset_type);
        });

        listbox.add(&row);
        listbox.show_all();
    }
}
