use macroquad::prelude::*;
use macroquad_xp_barre_menu::*;

#[macroquad::main("Default")]
async fn main() {
    let mut barre = Barre::default()
        .with_menu(
            Menu::new("Game")
                .with_item(Button::new("New", || println!("New")))
                .with_item(Separator)
                .with_item(Radio::new(vec!["Beginner", "Intermediate", "Expert"], |v| println!("Value: {}", v)))
                .with_item(Button::new("Custom...", || println!("New")))
                .with_item(Separator)
                .with_item(Checkbox::new("Marks (?)", false, |v| println!("Value: {}", v)))
                .with_item(Checkbox::new("Color", false, |v| println!("Value: {}", v)))
                .with_item(Checkbox::new("Sound", false, |v| println!("Value: {}", v)))
                .with_item(Separator)
                .with_item(Button::new("Best Times...", || println!("New")))
                .with_item(Separator)
                .with_item(Button::new("Exit", || std::process::exit(0))),
        )
        .with_menu(Menu::new("Help").with_item(Button::new("Did you really think you would find help here ?", || println!("About"))));
    let settings = &Settings::default();
    loop {
        clear_background(WHITE);
        barre.draw(settings);
        next_frame().await
    }
}
