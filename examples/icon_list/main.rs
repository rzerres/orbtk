// suppress creation of a new console window on window
#![windows_subsystem = "windows"]

// use cfg_if::cfg_if;

use orbtk::prelude::*;

//#[cfg(target_os = "windows")]
//use orbtk::theme_fluent::{THEME_FLUENT, THEME_FLUENT_COLORS_DARK, THEME_FLUENT_FONTS};

use widgets::main_view;

mod data;
mod widgets;

// Style extension
static DEFAULT_DARK_EXT: &str = include_str!("../../assets/icon_list/default_dark.ron");
//cfg_if! {
//    if #[cfg(windows)] {
static FLUENT_DARK_EXT: &str = include_str!("../../assets/icon_list/fluent_dark.ron");
//static FLUENT_LIGHT_EXT: &str = include_str!("../../assets/icon_list/fluent_light.ron");
//    }
//}

// German localization file.
static ICON_LIST_DE_DE: &str = include_str!("../../assets/icon_list/icon_list_de_DE.ron");

//cfg_if! {
//if #[cfg(windows)] {
// /// Extend and register theme assets.
// fn theme() -> Theme {
//     register_default_fonts(Theme::from_config(
//         ThemeConfig::from(DEFAULT_DARK_EXT)
//             .extend(ThemeConfig::from(THEME_DEFAULT))
//             .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
//             .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
//     ))
// }
// fn theme_fluent() -> Theme {
//     register_fluent_fonts(Theme::from_config(
//         ThemeConfig::from(FLUENT_DARK_EXT)
//             .extend(ThemeConfig::from(THEME_FLUENT))
//             .extend(ThemeConfig::from(THEME_FLUENT_COLORS_DARK))
//             .extend(ThemeConfig::from(THEME_FLUENT_FONTS)),
//     ))
// register_fluent_fonts(Theme::from_config(
//     ThemeConfig::from(FLUENT_LIGHT_EXT)
//         .extend(ThemeConfig::from(THEME_FLUENT))
//         .extend(ThemeConfig::from(THEME_FLUENT_COLORS_DARK))
//         .extend(ThemeConfig::from(THEME_FLUENT_FONTS)),
//}
//} else {
// /// Extend and register theme assets.
// fn theme() -> Theme {
//     register_default_fonts(Theme::from_config(
//         ThemeConfig::from(DEFAULT_DARK_EXT)
//             .extend(ThemeConfig::from(THEME_DEFAULT))
//             .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
//             .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
//    ))
// }
//}
//}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let machine_kind = if cfg!(unix) {
        "unix"
    } else if cfg!(windows) {
        "windows"
    } else {
        "unknown/unsupported"
    };
    println!("icon_list: running on machine {}", &machine_kind);

    // use this only if you want to run it as web application.
    orbtk::initialize();

    // if no dictionary is set for the default language e.g. english
    // the content of the text property will be drawn.
    let localization = RonLocalization::create()
        .language("en_US")
        .dictionary("de_DE", ICON_LIST_DE_DE)
        .build();

    Application::new()
        .localization(localization)
        //.theme(theme())
        //.theme(theme_fluent())
        .window(|ctx| {
            Window::new()
                .title("IconList example")
                .h_align("center")
                .position((100.0, 100.0))
                .size(600.0, 360.0)
                .resizeable(true)
                .child(main_view::MainView::new().build(ctx))
                .build(ctx)
        })
        .run();

    Ok(())
}
