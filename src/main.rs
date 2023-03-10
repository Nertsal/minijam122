use geng::prelude::*;

mod assets;
mod camera;
mod game;
mod gltf_load;
mod loading_screen;
mod menu;
mod model;
mod render;
mod ui;
mod util;

use assets::*;
use camera::Camera;
use game::*;
use gltf_load::*;
use loading_screen::LoadingScreen;
use menu::Menu;
use model::*;
use render::*;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    // #[cfg(not(target_arch = "wasm32"))]
    // #[cfg(not(feature = "console"))]
    // std::panic::set_hook(Box::new({
    //     fn hook(info: &std::panic::PanicInfo) {
    //         let mut f = std::fs::File::create(
    //             std::env::current_exe()
    //                 .unwrap()
    //                 .parent()
    //                 .unwrap_or_else(|| ".".as_ref())
    //                 .join("panic.txt"),
    //         )
    //         .unwrap();
    //         let _ = writeln!(f, "{info}");
    //     }
    //     hook
    // }));

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Golf Game".to_string(),
        vsync: true,
        fixed_delta_time: 1.0 / 200.0,
        ..default()
    });

    // #[cfg(not(target_arch = "wasm32"))]
    // geng.set_icon(&static_path().join("assets").join("icon.png"))
    //     .unwrap();

    geng.audio().set_volume(0.5);

    geng::run(
        &geng,
        geng::LoadingScreen::new(
            &geng,
            LoadingScreen::new(&geng),
            <Assets as geng::LoadAsset>::load(&geng, &static_path()),
            {
                let geng = geng.clone();
                move |assets| {
                    let assets = assets.unwrap();
                    let mut music = assets.sfx.music.effect();
                    music.set_volume(0.5);
                    music.play();
                    // TODO: music.set_volume when change setting
                    Menu::new(&geng, &Rc::new(assets))
                }
            },
        ),
    );
}
