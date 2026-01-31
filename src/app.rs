use crate::models::AppDatabase;
use crate::storage::Storage;

#[derive(PartialEq)]
enum Tab {
    Calendar,
    Tasks,
    Menus,
    Recipes,
}

pub struct NmptmApp {
    storage: Storage,
    db: AppDatabase,
    current_tab: Tab,
}

impl NmptmApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let storage = Storage::new("resources/database.json");
        let db = storage.load().unwrap_or_default();

        Self {
            storage,
            db,
            current_tab: Tab::Tasks,
        }
    }
}

impl eframe::App for NmptmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Calendar, "📅 Calendar");
                ui.selectable_value(&mut self.current_tab, Tab::Tasks, "✅ Tasks");
                ui.selectable_value(&mut self.current_tab, Tab::Menus, "🍴 Menus");
                ui.selectable_value(&mut self.current_tab, Tab::Recipes, "📖 Recipes");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Calendar => {
                    ui.heading("Calendar View");
                    ui.label("Calendar content goes here...");
                }
                Tab::Tasks => {
                    ui.heading("Tasks List");
                    ui.label(format!("Count: {}", self.db.tasks.len()));
                    // Add task list implementation here
                }
                Tab::Menus => {
                    ui.heading("Menus");
                    ui.label("Menu list goes here...");
                }
                Tab::Recipes => {
                    ui.heading("Recipes");
                    ui.label("Recipe list goes here...");
                }
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Err(e) = self.storage.save(&self.db) {
            eprintln!("Failed to save database: {}", e);
        }
    }
}
