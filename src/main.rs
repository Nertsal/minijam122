use geng::prelude::*;

mod assets;
mod game;
mod loading_screen;

use assets::*;
use game::Game;
use loading_screen::LoadingScreen;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "console"))]
    std::panic::set_hook(Box::new({
        fn hook(info: &std::panic::PanicInfo) {
            let mut f = std::fs::File::create(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap_or(".".as_ref())
                    .join("panic.txt"),
            )
            .unwrap();
            let _ = writeln!(f, "{info}");
        }
        hook
    }));

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Golf Game".to_string(),
        vsync: false,
        ..default()
    });

    // #[cfg(not(target_arch = "wasm32"))]
    // geng.set_icon(&static_path().join("assets").join("icon.png"))
    //     .unwrap();

    geng.audio().set_volume(0.0);

    geng::run(
        &geng,
        geng::LoadingScreen::new(
            &geng,
            LoadingScreen::new(&geng),
            <Assets as geng::LoadAsset>::load(&geng, &static_path().join("assets")),
            {
                let geng = geng.clone();
                move |assets| Game::new(&geng, &Rc::new(assets.unwrap()))
            },
        ),
    );
}
