// suppress creation of a new console window on window
#![windows_subsystem = "windows"]

//use cfg_if::cfg_if;

use orbtk::{
    prelude::*,
    //theme_default::{THEME_DEFAULT, THEME_DEFAULT_COLORS_DARK, THEME_DEFAULT_FONTS},
    //theme_fluent::{THEME_FLUENT, THEME_FLUENT_COLORS_DARK, THEME_FLUENT_FONTS},
    //theming::config::ThemeConfig,
};

//#[cfg(target_os = "windows")]
//use orbtk::theme_fluent::{THEME_FLUENT, THEME_FLUENT_COLORS_DARK, THEME_FLUENT_FONTS};

use crate::data::constants::*;

mod widgets::menu;
mod data;

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

/// Define the members of the glyph structure. Each entry has an
/// identifier that describes the glyph, the glyph symbol and the
/// string describing the unicode point.
#[derive(Debug, Clone, PartialEq)]
pub struct GlyphStruct {
    glyph_icon: String,
    glyph_identifier: String,
    unicode_string: String,
}

/// `TableData` is a vector of valid glyphs. Each vector member boxes the glyph structure.
type TableData = Vec<GlyphStruct>;

/// Valid `structures` that are handled inside the state of the `main view` widget.
#[derive(Default, AsAny)]
struct MainViewState;

// Macro that initializes the widget structures/variables for the policy check view
widget!(MainView<MainViewState>{
    data: TableData
});

/// The template implementation of the main view
/// All GUI elements are styled using the "style" attribute referencing to a ron based css
impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let button_menu = Button::new()
            .id(ID_BUTTON_MENU)
            .style("button_single_content")
            .icon(material_icons_font::MD_MENU)
            .attach(Grid::column(2))
            .h_align("end")
            .on_click(move |_ctx, _| {
                println!("WIP: open menu popup from MenuView");
                // ctx.get_mut::<MenuState>(id)
                //      .set_action(MenuAction::CreateMenu(ID_MENU_STACK));
                true
            })
            .build(ctx);

        let header_bar = Container::new()
            .id(ID_HEADER_BAR)
            .style(STYLE_HEADER_BAR)
            .attach(Grid::row(0))
            .attach(Grid::column(1))
            .attach(Grid::column_span(2))
            .child(
                Grid::new()
                    .child(
                        TextBlock::new()
                            .id(ID_HEADER)
                            .style(STYLE_HEADER)
                            .v_align("center")
                            .h_align("left")
                            .text("Icon list")
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .child(button_menu)
            .build(ctx);

        let icon_table = Container::new()
            .id(ID_ICON_TABLE)
            .min_width(420)
            .attach(Grid::row(2))
            .attach(Grid::column(1))
            .style(STYLE_TABLE_FORM)
            .child(
                TableView::new()
                    .id(ID_TABLE_VIEW)
                    .column("Symbol", ctx)
                    .column("Identifier", ctx)
                    .column("Unicode string", ctx)
                    .row_builder(move | ctx, row_index: usize, row_cells | {
                        let row = ctx
                            .get_widget(id)
                            .get::<TableData>("data")
                            .get(row_index)
                            .unwrap()
                            .clone();
                        //let icon_font = "material_icons_font::".to_string();
                        //let icon_name = &row.glyph_icon.to_string();
                        //let icon_name: &str = row.glyph_icon.to_owned();
                        //icon_name.push_str(&row.glyph_icon);

                        // create a string slice (str)
                        let icon_font = "material_icons_font::";
                        let icon_name = row.glyph_icon.to_owned();
                        println!("Icon: '{}''{}'", icon_font, icon_name);
                        //let active_theme = Theme::get(&mut ctx.window());
                        //let props = active_theme.properties("resources");
                        //let set_icon = props.get(icon_name.to_owned());
                        row_cells.push(
                            Button::new()
                                //.style("button_single_content")
                                .icon(material_icons_font::MD_WIFI)
                                //.icon_font("MDL2-Assets-Regular")
                                //.icon(MDL2_WIFI)
                                .icon(material_icons_font::MD_AIRPLANEMODE_ACTIVE)
                                .icon(mdl2_assets_font::MDL2_AIRPLANE)
                                //.icon(set_icon)
                                .build(ctx),
                        );
                        row_cells.push(
                            TextBlock::new()
                                .style("content_area")
                                .text(row.glyph_identifier)
                                .build(ctx));
                        row_cells.push(
                            TextBlock::new()
                                .style("content_area")
                                .text(row.unicode_string)
                                .build(ctx));
                    })
                    .build(ctx),
            )
            .build(ctx);

        // starter page: main view
        self.name("MainView")
            // initialize struct (derived default macro)
            .child(
                Grid::new()
                    .id(ID_ICON_LIST)
                    .columns(
                        Columns::create()
                            .push(50)
                            .push("*")
                            .push(50)
                    )
                    .rows(
                        Rows::create()
                            .push("auto") // header
                            .push(28)     // separator
                            .push("*")    // table
                            .push(28)     // separator
                    )

                     // row 0: Header
                    .child(header_bar)

                     // row 2: Table View
                    .child(icon_table)

                    .build(ctx)
            )
    }
}

/// Method definitions, that react on any given state change inside the `MainView` widget.
impl State for MainViewState {
    fn init(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        let table_data = vec![
            GlyphStruct {
                glyph_icon: "MD_WIFI".to_string(),
                glyph_identifier: "Wifi".to_string(),
                unicode_string: "E701".to_string(),
            },
            GlyphStruct {
                glyph_icon: "MD_AIRPLANEMODE_ACTIVE".to_string(),
                glyph_identifier: "Airplane Mode Active".to_string(),
                unicode_string: "E709".to_string(),
            },
        ];

        // calculate the vector length
        let table_data_len = table_data.len();
        //TableView::data_set(&mut ctx.widget(), table_data);
        ctx.widget().set::<TableData>("data", table_data);

        // initialize the row counter to create the grid row members
        TableView::row_count_set(&mut ctx.child(ID_TABLE_VIEW), table_data_len);
        println!("WIP: initialized child's row_conter to {}",
                 table_data_len);

        //ctx.switch_theme(theme_redox());
        //ctx.switch_theme(theme_default_light());
        ctx.switch_theme(theme_default_dark());
        //ctx.switch_theme(theme_fluent_dark());
    }
}

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

    // if no dictionary is set for the default language e.g. english the content of the text property will drawn.
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
                .h_align("center")
                .title("IconList example")
                .position((100.0, 100.0))
                .size(600.0, 360.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();

     Ok(())
}
