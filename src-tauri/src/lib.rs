use tauri::{image::Image, menu::{IconMenuItem, IconMenuItemBuilder, MenuBuilder}};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {

    tauri::Builder::default()
        .setup(|app| {
            app.set_theme(Some(tauri::Theme::Dark));
            let icon_settings = Image::from_bytes(include_bytes!("../icons/settings.png")).unwrap();


            let icon_item_settings = IconMenuItemBuilder::new("settings")
                .icon(icon_settings)
                .id("settings")
                .build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&icon_item_settings])
                .build()?;

            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                        println!("menu event: {:?}", event.id());        
                        match event.id().0.as_str() {
                            "settings" => {
                                println!("menu settings event");
                            }
                            _ => {
                                println!("unexpected menu event {:?}", event.id().0.as_str());
                            }
                        }
                    });

            app.set_menu(menu)?;
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())

}
