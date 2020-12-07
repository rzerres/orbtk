use orbtk::prelude::*;

use crate::{
    data::constants::*,
    widgets::menu::menu_state::{MenuAction, MenuState},
};

type List = Vec<String>;

// Macro that initializes the widget structures/variables for the menu view
widget!(
    MenuView<MenuState> {
        //menu_stack: Entity,
        //button_menu: Entity,
        selected_index: i32,
        themes: List
    }
);

/// The template implementation of the menu view
/// All GUI elements are styled using the "style" attribute referencing to a ron based css
impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext<'_>) -> Self {
        //self.themes(themes).child(MenuState::create_menu(ID_MENU_POPUP, ctx))
        self.child(
            Stack::new()
                .id(ID_MENU_STACK)
                .child(
                    TextBlock::new()
                        .style("header")
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        .style("small_text")
                        .text("Select theme")
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .id(ID_MENU_BUTTON)
                        //.style("button_single_content")
                        .style("button")
                        .icon(material_icons_font::MD_MENU)
                        .attach(Grid::column(2))
                        .h_align("end")
                        .on_click(move |ctx, _| {
                            println!("WIP: on_click -> open menu popup from MenuView");
                            ctx.get_mut::<MenuState>(id)
                                .set_action(MenuAction::CreateMenu);
                            true
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
