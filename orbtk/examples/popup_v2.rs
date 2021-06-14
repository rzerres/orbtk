use orbtk::{
    prelude::*,
    // only instantiate the `default` theme
    widgets::themes::theme_orbtk::{
        THEME_DEFAULT, THEME_DEFAULT_COLORS_DARK, THEME_DEFAULT_FONTS,
        register_default_fonts
    },
};

static DARK_EXT: &str = include_str!("assets/popup/default_dark.ron");

static ID_GRID: &str = "GRID";
static ID_BUTTON: &str = "BUTTON";
static ID_COMBO_BOX: &str = "COMBO BOX";
static ID_TARGET: &str = "TARGET";

#[derive(Copy, Clone)]
enum PopupAction {
    Toggle,
    UpdateRelativePosition,
}

#[derive(Default, AsAny)]
struct MainViewState {
    action: Option<PopupAction>,
    popup: Option<Entity>,
    target: Option<Entity>,
}

type List = Vec<String>;

impl MainViewState {
    fn toggle_popup(&mut self) {
        self.action = Some(PopupAction::Toggle);
    }

    fn update_relative_position(&mut self) {
        self.action = Some(PopupAction::UpdateRelativePosition);
    }
}

impl State for MainViewState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        let current_entity = ctx.entity();
        let target = ctx.entity_of_child(ID_TARGET).unwrap();
        let build_context = &mut ctx.build_context();

        self.target = Some(target);
        let popup = create_popup(
            current_entity,
            "Popup text ...",
            build_context,
        );
        //build_context.append_child_to_overlay(popup).expect("Failed to add popup to overlay");
        build_context.append_child(target, popup);
        self.popup = Some(popup);

        println!("Popup = {:?} associated to Target => {:?}", &self.popup, &self.target);
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        _ctx: &mut Context,
    ) {
        for message in messages.read::<PopupAction>() {
            match message {
                PopupAction::UpdateRelativePosition => {
                    println!("Popup: relative position has changed!");
                    MainViewState::update_relative_position(self);
                }
                PopupAction::Toggle => {
                    println!("Popup: toggled!");
                    MainViewState::toggle_popup(self);
                }
            }
        }
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                PopupAction::Toggle => {
                    println!("Popup state will be toggled...");
                    if let Some(popup) = self.popup {
                        let open = ctx.get_widget(popup).clone::<bool>("open");
                        println!("Current open: {}", open);
                        println!(
                            "Current visibility: {:#?}",
                            ctx.get_widget(popup).clone::<Visibility>("visibility")
                        );
                        if open {
                            ctx.get_widget(popup).set("open", false);
                            change_button_title("Click me to show popup", ctx);
                        } else {
                            change_button_title("Click me to hide popup", ctx);
                        }
                        println!("Popup toggled!");
                    }
                }
                PopupAction::UpdateRelativePosition => {
                    if let Some(popup) = self.popup {
                        let combo_box = ctx.entity_of_child(ID_COMBO_BOX).unwrap();
                        let selected_index: i32 = ctx.get_widget(combo_box).clone("selected_index");
                        let relative_position: RelativePosition = ctx.get_widget(popup).clone_or_default("relative_position");
                        match selected_index {
                            0 => ctx.get_widget(self.popup.unwrap()).set("relative_position",relative_position.to_bottom()),
                            1 => ctx.get_widget(self.popup.unwrap()).set("relative_position",relative_position.to_top()),
                            2 => ctx.get_widget(self.popup.unwrap()).set("relative_position",relative_position.to_left()),
                            3 => ctx.get_widget(self.popup.unwrap()).set("relative_position",relative_position.to_right()),
                            _ => panic!()
                        }
                        println!("Relative position updated.");
                    }
                }
            }
            self.action = None;
        }
    }
}

fn create_popup(target: Entity, text: &str, ctx: &mut BuildContext) -> Entity {
    Popup::new()
        // popup is target of given entity
        .target(target)
        // alternative: popup is target of given viewpoint
        //.target(Point::new(200.0,200.0))
        //Specify the popup position relative to the target (the button in this case)
        //This is also the default value if no one is specified
        .relative_position(RelativePosition::Left(15.0))
        .open(true)
        //.style("popup_form")
        .width(150.0)
        .height(150.0)
        .child(
            Container::new()
                .child(
                    TextBlock::new()
                        //.style("popup_text_block")
                        .h_align("center")
                        //.v_align("top")
                        .text(text)
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

fn change_button_title(title: &str, ctx: &mut Context) {
    let button = ctx.entity_of_child(ID_BUTTON).unwrap();
    ctx.get_widget(button)
        .set::<String16>("text", String16::from(title));
}

fn theme() -> Theme {
     register_default_fonts(Theme::from_config(
        ThemeConfig::from(DARK_EXT)
            .extend(ThemeConfig::from(THEME_DEFAULT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}


// constructs the MainView
widget!(
    MainView<MainViewState> {
        popup_pos: List
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let popup_pos = vec![
            "Top".to_string(),
            "Bottom".to_string(),
            "Left".to_string(),
            "Right".to_string(),
        ];
        let popup_pos_count = popup_pos.len();
        self.name("MainView").margin(16.0).child(
            Grid::new()
                .id(ID_GRID)
                .rows("50, 200, *, 200")
                .columns("200, *, 200")
                // .child(
                //     Button::new()
                //         .id(ID_BUTTON)
                //         .attach(Grid::row(0))
                //         .attach(Grid::column(0))
                //         //.style("button")
                //         .h_align("center")
                //         .text("Toggle position")
                //         .on_click(move |ctx, _| -> bool {
                //             ctx.get_mut::<MainViewState>(id).update_relative_position();
                //             true
                //         })
                //         .build(ctx),
                // )
                .child(
                    ComboBox::new()
                        .id(ID_COMBO_BOX)
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        //.style("combo_box_form")
                        .h_align("center")
                        .width(250.0)
                        .count(popup_pos_count)
                        .items_builder(move |bc, index| {
                            let text = MainView::popup_pos_ref(&bc.get_widget(id))[index].clone();
                            TextBlock::new().v_align("center").text(text).build(bc)
                        })
                        .selected_index(id)
                        //  .on_changed("selected_item", move |states, _entity| {
                        //     states
                        //         .get_mut::<MainViewState>(id)
                        //         .update_relative_position()
                        // })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .id(ID_BUTTON)
                        .attach(Grid::row(0))
                        .attach(Grid::column(1))
                        //.style("button")
                        .h_align("center")
                        .text("Click me to hide popup")
                        .on_click(move |ctx, _| -> bool {
                            ctx.get_mut::<MainViewState>(id).toggle_popup();
                            true
                        })
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .id(ID_TARGET)
                        .attach(Grid::row(2))
                        .attach(Grid::column(1))
                        .style("container_form")
                        .child(
                            TextBlock::new()
                                .style("target_text_block")
                                .text("Target")
                                .h_align("center")
                                .v_align("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Popup example")
                .position((100.0, 100.0))
                .size(750, 750.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
