use cfg_if::cfg_if;
use orbtk::prelude::*;
use orbtk::shell::event::Key;

use std::process;

use crate::{data::constants::*, widgets::menu::menu_view::MenuView};

/// Valid `actions` that are handled as state changes in the `Menu` widget.
#[derive(Debug, Clone, Copy)]
pub enum MenuAction {
    CreateMenu,
    CreateMenuToggleTheme,
    RemoveMenu,
    RemoveMenuToggleTheme,
    SetTheme,
    UpdateMenuRelativePosition,
}

/// Valid `structures` that are handled inside the state of the `Menu` widget.
#[derive(AsAny, Default)]
pub struct MenuState {
    action: Option<MenuAction>,
    menu: Entity,
    //menu: Option<Entity>,
    //menu_toggle_theme: Option<Entity>
    menu_toggle_theme: Entity,
}

/// Method definitions, that react on any given state change inside the `Menu` widget.
impl MenuState {
    /// Create a menu as an overlay widget that is a child of the given parent.
    /// The menu elements are placed inside a grid.
    //pub fn create_menu(&mut self, id_str: &str, _ctx: &mut Context<'_>) {
    pub fn create_menu(&mut self, ctx: &mut Context<'_>) {
        self.action = Some(MenuAction::CreateMenu);
        let menu_entity = self.menu;
        let open = ctx.get_widget(self.menu).clone::<bool>("open");
        println!("crate_menu(): Current open: {}", open);
        println!(
            "create_menu(): Current visibility: {:#?}",
            ctx.get_widget(menu_entity)
                .clone::<Visibility>("visibility")
        );
    }

    /// Create a toggle_theme menu as a child of the given parent
    /// Select the active theme from a `ComboBox` offering a list of valid `themes`
    fn create_menu_toggle_theme(&mut self, _ctx: &mut Context<'_>) {
        self.action = Some(MenuAction::CreateMenuToggleTheme);
    }

    /// Remove the menu popup box
    fn remove_menu(&mut self, ctx: &mut Context<'_>) {
        self.action = Some(MenuAction::RemoveMenu);
        ctx.remove_child(ctx.entity());
        println!("Popup {:?} removed.", ctx.entity());
    }

    /// Remove the menu popup box
    fn remove_menu_toggle_theme(&mut self, _ctx: &mut Context<'_>) {
        self.action = Some(MenuAction::RemoveMenuToggleTheme);
    }

    /// Sets a new action.
    pub fn set_action(&mut self, action: MenuAction) {
        self.action = action.into();
    }

    /// Update the relative position of the menu overlay
    fn update_menu_relative_position(&mut self) {
        println!("TODO: Update relative position of menu {}.", ID_MENU_POPUP);
        //self.action = Some(MenuAction::UpdateRelativePosition);
    }
}

/// Supported methods handled inside the `MenuState`
impl State for MenuState {
    /// Initialize the state of widgets inside `MenuState`
    fn init(&mut self, _: &mut Registry, _ctx: &mut Context<'_>) {
        // Initialize required entities
        // let menu_button = ctx
        //     .entity_of_child(ID_MENU_STACK)
        //     .expect("MenuState.init: Can't find resource entity 'PolicycheckView::ID_POLICY_CHECK_BUTTON_MENU'.");
    }

    /// Handle messages for the `MenuState`
    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context<'_>,
    ) {
        for message in messages.read::<MenuAction>() {
            match message {
                MenuAction::SetTheme => {
                    let theme_index = *MenuView::selected_index_ref(&ctx.widget());

                    cfg_if! {
                        if #[cfg(windows)] {
                            match theme_index {
                                0 => ctx.switch_theme(theme_default_dark()),
                                1 => ctx.switch_theme(theme_default_light()),
                                2 => ctx.switch_theme(theme_redox()),
                                3 => ctx.switch_theme(theme_fluent_dark()),
                                4 => ctx.switch_theme(theme_fluent_light()),
                                _ => {
                                    println!("theme_index {} is not supported.", theme_index);
                                }
                            }
                        } else {
                            match theme_index {
                                0 => ctx.switch_theme(theme_default_dark()),
                                1 => ctx.switch_theme(theme_default_light()),
                                2 => ctx.switch_theme(theme_redox()),
                                _ => {
                                    println!("theme_index {} is not supported.", theme_index);
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    /// Update the state of widgets inside the `Menu` view.
    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                MenuAction::CreateMenu => {
                    // target is a child of the given parent entity (here: ID_MENU_BUTTON)
                    let _target = ctx
                        .entity_of_child(ID_MENU_BUTTON)
                        //.expect("Can't find entity of resource '{}'.", ID_MENU_BUTTON);
                        .expect("Can't find entity of resource 'ID_MENU_BUTTON'.");
                    //.entity_of_child(<PolicycheckView>ID_MENU_BUTTON)
                    //.expect("PolicycheckState: Can't find entity of resource 'ID_POLICY_CHECK_POPUP_MENU'.");

                    // create the popup entity
                    let parent = ctx.entity();
                    let menu = create_menu_popup(parent, &mut ctx.build_context());

                    // create the overlay entity consuming the popup entity
                    ctx.build_context().append_child_to_overlay(menu).expect(
                        "Failed create an overlay that consumes popup `menu` as its child.",
                    );
                    //self.menu = Some(menu);
                    self.menu = menu;

                    println!("Popup Menu created: {:?}", self.menu);
                }
                MenuAction::CreateMenuToggleTheme => {
                    let menu_target = ctx.entity_of_child(ID_MENU_LABEL_TOGGLE_THEME).expect(
                        "MenuState: Can't find entity of resource 'ID_MENU_LABEL_TOGGLE_THEME'.",
                    );
                    let current_entity = ctx.entity();
                    let build_context = &mut ctx.build_context();

                    // create a new menu overlay
                    self.menu_toggle_theme =
                        create_menu_toggle_theme_popup(current_entity, build_context);

                    // create a menu_popup widget as a child of entity "ID_POPUP_MENU"
                    build_context.append_child(menu_target, self.menu_toggle_theme);

                    println!(
                        "Popup Menu Toggle Theme created: {:?}",
                        self.menu_toggle_theme
                    );
                }
                MenuAction::RemoveMenu => {
                    self.remove_menu(ctx);
                }
                MenuAction::RemoveMenuToggleTheme => {
                    ctx.remove_child(self.menu_toggle_theme);
                    println!("Popup {:?} removed.", ctx.entity());
                }
                MenuAction::UpdateMenuRelativePosition => {
                    let menu = self.menu;
                    println!("Relative position of menu {:?} updated.", menu);

                    //if let Some(menu) = self.menu {
                    // let theme_combo_box = ctx.entity_of_child(ID_MENU_TOGGLE_THEME).unwrap();
                    // let selected_index: i32 = ctx.get_widget(theme_combo_box).clone("selected_index");
                    // // let relative_position: RelativePosition =
                    // //    ctx.get_widget(ID_MENU_POPUP).clone_or_default("relative_position");
                    // match selected_index {
                    //     0 => {
                    //         RelativePosition::relative_position_mut(&mut ctx.widget()).into_bottom();
                    //         //RelativePosition::get_mut::<MenuState>("relative_position").into_bottom();
                    //         // ctx
                    //         //     .get_widget(self.ID_MENU_POPUP.unwrap())
                    //         //     .set("relative_position", relative_position.into_bottom());
                    //     },
                    //     1 => RelativePosition::relative_position_mut(&mut ctx.widget()).into_top(),
                    //     2 => RelativePosition::relative_position_mut(&mut ctx.widget()).into_left(),
                    //     3 => RelativePosition::relative_position_mut(&mut ctx.widget()).into_right(),
                    //     _ => panic!(),
                    // }
                    //println!("Relative position of menu {:?} updated.", menu);
                    //}
                }
                _ => (),
            }
        }
    }
}

/// Create a new popup presenting the menu components
fn create_menu_popup(id: Entity, ctx: &mut BuildContext<'_>) -> Entity {
    Popup::new()
        //.id(ID_MENU_POPUP)
        .target(id.0)
        .style("popup_menu")
        // Specify the popup position relative to the target
        .relative_position(RelativePosition::Bottom(5.0))
        .open(true)
        //.width(280)
        //.height(140)
        .on_key_down(move |ctx, key_event| {
            match key_event.key {
                Key::Q(..) => {
                    //if is_ctrl_home_down(ctx)
                    println!("KeyHandler: got Ctrl+Q");
                    process::exit(0);
                    //}
                }
                Key::Escape => {
                    println!("KeyHandler: got Escape");
                    ctx.get_mut::<MenuState>(id)
                        .set_action(MenuAction::RemoveMenu);
                }
                _ => {
                    println!("KeyHandler: got {:?}", key_event.key);
                }
            };
            true
        })
        .child(
            Grid::new()
                .id(ID_MENU_GRID)
                .columns(
                    Columns::create()
                        .push("180") // Menu Button
                        .push("1") // Seperator
                        .push("auto"), // Keyboard Shortcut
                )
                .rows(Rows::create().push("auto").push("auto").push("auto"))
                .child(
                    Button::new()
                        .id(ID_MENU_LABEL_ACCOUNT)
                        .style("button_menu")
                        //.h_align("start")
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        //.attach(Grid::column_span(2))
                        .icon(material_icons_font::MD_PERSON)
                        .text("Account")
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .id(ID_MENU_LABEL_TOGGLE_THEME)
                        .style("button_menu")
                        //.h_align("start")
                        .attach(Grid::row(1))
                        .attach(Grid::column(0))
                        //.attach(Grid::column_span(2))
                        .icon(material_icons_font::MD_EDIT)
                        .text("Toggle theme")
                        .on_click(move |ctx, _| {
                            ctx.get_mut::<MenuState>(id)
                                .set_action(MenuAction::CreateMenuToggleTheme);
                            true
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .id(ID_MENU_LABEL_QUIT)
                        .style("button_menu")
                        //.h_align("start")
                        .attach(Grid::row(2))
                        .attach(Grid::column(0))
                        //.attach(Grid::column_span(2))
                        .icon(material_icons_font::MD_SETTINGS_POWER)
                        .text("Quit")
                        .on_mouse_down(move |_states, _| {
                            process::exit(0);
                        })
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .id(ID_MENU_SHORTCUT_QUIT)
                        //.style("button_menu")
                        .attach(Grid::row(2))
                        .attach(Grid::column(2))
                        .margin((0, 0, 16, 0))
                        .h_align("end")
                        .v_align("center")
                        .text("CTRL+Q")
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

/// Create a new popup submenu to toogle the active theme
fn create_menu_toggle_theme_popup(id: Entity, ctx: &mut BuildContext<'_>) -> Entity {
    cfg_if! {
        if #[cfg(windows)] {
            // define the list of supported themes
            let themes = vec![
                "default_dark".to_string(),
                "default_light".to_string(),
                "redox".to_string(),
                "fluent_dark".to_string(),
                "fluent_light".to_string()
            ];
        } else {
            // define the list of supported themes
            let themes = vec![
                "default_dark".to_string(),
                "default_light".to_string(),
                "redox".to_string(),
            ];
        }
    }

    let themes_count = themes.len();
    //ProgressBar::val_set(&mut ctx.child(ID_POLICY_CHECK_PROGRESS_BAR), new_width);

    Popup::new()
        .id(ID_MENU_TOGGLE_THEME)
        .target(id.0)
        .style("container_menu")
        .open(true)
        .width(280)
        .height(140)
        .on_key_down(move |ctx, key_event| {
            match key_event.key {
                Key::Escape => {
                    println!("KeyHandler: got Escape");
                    ctx.get_mut::<MenuState>(id)
                        .set_action(MenuAction::RemoveMenuToggleTheme);
                }
                _ => {
                    println!("KeyHandler: got {:?}", key_event.key);
                }
            };
            true
        })
        .child(
            ComboBox::new()
                .attach(Grid::column(2))
                .attach(Grid::row(6))
                .count(themes_count)
                .items_builder(move |ctx, index| {
                    let theme_name = MenuView::themes_ref(&ctx.get_widget(id))[index].clone();
                    TextBlock::new()
                        .v_align("center")
                        .text(theme_name)
                        .build(ctx)
                })
                .on_changed("selected_index", move |ctx, _entity| {
                    ctx.send_message(MenuAction::SetTheme, id);
                    println!("changed theme.");
                })
                .selected_index(id)
                .build(ctx),
        )
        .build(ctx)
}
