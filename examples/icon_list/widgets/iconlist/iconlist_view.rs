use orbtk::prelude::*;

use crate::{
    data::constants::*,
    widgets::iconlist::iconlist_state::IconListState,
    widgets::menu::menu_state::{MenuAction, MenuState},
};

/// Define the members of the glyph structure. Each entry has an
/// identifier that describes the glyph, the glyph symbol and the
/// string describing the unicode point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlyphStruct {
    /// The byteglyph
    pub glyph_icon: String,
    pub glyph_identifier: String,
    pub glyph_unicode: String,
}

/// `TableData` is a vector of valid glyphs. Each vector member boxes the glyph structure.
pub type TableData = Vec<GlyphStruct>;

// Macro that initializes the widget structures/variables for the policy check view
widget!(IconListView<IconListState>{
    data: TableData
});

/// The template implementation of the main view
/// All GUI elements are styled using the "style" attribute referencing to a ron based css
impl Template for IconListView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // define a button to handle the menu
        let button_menu = Button::new()
            .id(ID_BUTTON_MENU)
            .style("button_single_content")
            //.style("button")
            .icon(material_icons_font::MD_MENU)
            .attach(Grid::column(2))
            .h_align("end")
            .on_click(move |_ctx, _| {
                println!("WIP: call MenuAction::CreateMenu({})", ID_MENU_STACK);
                //ctx.get_mut::<MenuState>(id)
                //    .set_action(MenuAction::CreateMenu);
                //    .set_action(MenuAction::CreateMenu(ID_MENU_STACK));
                true
            })
            .build(ctx);

        // define a container to hold the header
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

        // define a container to handle the icon grid
        let icon_table = Container::new()
            .id(ID_ICON_TABLE)
            .min_width(420)
            .attach(Grid::row(2))
            .attach(Grid::column(1))
            .style(STYLE_TABLE_FORM)
            .child(
                TableView::new()
                    .id(ID_TABLE_VIEW)
                    .column("Symbol", ID_TABLE_VIEW_HEADER_SYMBOL, IconAlignment::Start)
                    .column("Identifier", ID_TABLE_VIEW_HEADER_IDENTIFIER, IconAlignment::End)
                    .column("Unicode string", ID_TABLE_VIEW_HEADER_UNICODE, IconAlignment::Start)
                    .icon_alignment("End")
                    .data_source(id.0)
                    .row_builder(move |ctx, row_index: usize, row_cells| {
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
                                .build(ctx),
                        );
                        row_cells.push(
                            TextBlock::new()
                                .style("content_area")
                                .text(row.glyph_unicode)
                                .build(ctx),
                        );
                    })
                    .on_sort(
                        |sort_predicate, sort_direction, data_source, ctx| match sort_predicate {
                            ID_TABLE_VIEW_HEADER_SYMBOL => match sort_direction {
                                TableSortDirection::Ascending => {
                                    ctx.get_widget(data_source)
                                        .get_mut::<TableData>("data")
                                        .sort_by(|a, b| a.glyph_icon.cmp(&b.glyph_icon));
                              }
                                TableSortDirection::Descending => {
                                    ctx.get_widget(data_source)
                                        .get_mut::<TableData>("data")
                                        .sort_by(|a, b| b.glyph_icon.cmp(&a.glyph_icon));
                                }
                            },
                            ID_TABLE_VIEW_HEADER_IDENTIFIER => match sort_direction {
                                TableSortDirection::Ascending => {
                                    ctx.get_widget(data_source)
                                        .get_mut::<TableData>("data")
                                        .sort_by(|a, b| a.glyph_identifier.cmp(&b.glyph_identifier));
                                }
                                TableSortDirection::Descending => {
                                    ctx.get_widget(data_source)
                                        .get_mut::<TableData>("data")
                                        .sort_by(|a, b| b.glyph_identifier.cmp(&a.glyph_identifier));
                                }
                            },
                            ID_TABLE_VIEW_HEADER_UNICODE => match sort_direction {
                                TableSortDirection::Ascending => {
                                    ctx.get_widget(data_source)
                                        .get_mut::<TableData>("data")
                                        .sort_by(|a, b| a.glyph_unicode.cmp(&b.glyph_unicode));
                                }
                                TableSortDirection::Descending => {
                                    ctx.get_widget(data_source)
                                        .get_mut::<TableData>("data")
                                        .sort_by(|a, b| b.glyph_unicode.cmp(&a.glyph_unicode));
                                }
                            },
                            _ => {
                                println!("no match");
                            }
                        },
                    )
                    .build(ctx),
            )
            .build(ctx);

        // Starter page: list glyph icons
        self.name("IconlistView")
            // initialize struct (derived from default macro)
            //.icon_list(IconList::default())
            .child(
                Grid::new()
                    .id(ID_ICON_LIST)
                    .columns(Columns::create().push(50).push("*").push(50))
                    .rows(
                        Rows::create()
                            .push("auto") // header
                            .push(28) // separator
                            .push("*") // table
                            .push(28), // separator
                    )
                    // row 0: Header
                    .child(header_bar)
                    // row 2: Table View
                    .child(icon_table)
                    .build(ctx),
            )
    }
}
