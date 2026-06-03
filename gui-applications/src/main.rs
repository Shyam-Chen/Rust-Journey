use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // ========== 應用程式的狀態 - 開始 ==========
    let mut name = "Arthur".to_owned();
    let mut age = 42;
    // ========== 應用程式的狀態 - 結束 ==========

    eframe::run_ui_native("My egui App", options, move |ui, _frame| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("My egui Application");

            render_name_input(ui, &mut name);
            render_age_control(ui, &mut age);

            ui.label(format!("Hello, {name}!, age = {age}"));

            render_counter(ui);
        });
    })
}

// ========== --- ==========

fn render_name_input(ui: &mut egui::Ui, name: &mut String) {
    ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(name).labelled_by(name_label.id);
    });
}

fn render_age_control(ui: &mut egui::Ui, age: &mut u32) {
    ui.add(egui::Slider::new(age, 0..=120).text("age"));

    if ui
        .add_enabled(*age < 60, egui::Button::new("Increment"))
        .clicked()
    {
        *age += 1;
    }

    if ui.button("Increment").clicked() {
        *age += 1;
    }
}

// ========== --- ==========

fn render_counter(ui: &mut egui::Ui) {
    let id = ui.make_persistent_id("counter");

    // ========== 自己的狀態 - 開始 ==========
    // 從 Memory 讀取
    let mut count = ui.data_mut(|d| *d.get_persisted_mut_or_insert_with(id, || -> i32 { 1 }));
    // ========== 自己的狀態 - 結束 ==========

    ui.horizontal(|ui| {
        if ui.button("➖").clicked() {
            count -= 1;
        }

        ui.label(format!("count = {count}"));

        if ui.button("➕").clicked() {
            count += 1;
        }
    });

    // 將更新後的值寫回 Memory
    ui.data_mut(|d| d.insert_persisted(id, count));
}
