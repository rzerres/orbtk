use orbtk::prelude::*;

use crate::{
    data::constants::*,
    widgets::iconlist::iconlist_view::{GlyphStruct, TableData},
};

/// Handles the request of the `IconListView` widget.
#[derive(Default, AsAny)]
pub struct IconListState;

/// Method definitions, that react on any given state change inside the `MainView` widget.
impl State for IconListState {
    fn init(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        let table_data = vec![
            GlyphStruct {
                glyph_icon: "MD_WIFI".to_string(),
                glyph_identifier: "Wifi".to_string(),
                glyph_unicode: "E701".to_string(),
            },
            GlyphStruct {
                glyph_icon: "MD_AIRPLANEMODE_ACTIVE".to_string(),
                glyph_identifier: "Airplane Mode Active".to_string(),
                glyph_unicode: "E709".to_string(),
            },
        ];

        // calculate the vector length
        let table_data_len = table_data.len();
        //TableView::data_set(&mut ctx.widget(), table_data);
        ctx.widget().set::<TableData>("data", table_data);

        // initialize the row counter to create the grid row members
        TableView::row_count_set(&mut ctx.child(ID_TABLE_VIEW), table_data_len);
        println!("WIP: initialized child's row_counter to {}", table_data_len);

        //ctx.switch_theme(theme_redox());
        //ctx.switch_theme(theme_default_light());
        ctx.switch_theme(theme_default_dark());
        //ctx.switch_theme(theme_fluent_dark());
    }
}
