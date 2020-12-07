use orbtk::prelude::*;

use crate::widgets::iconlist::iconlist_view::IconListView;

widget!(MainView {});
//    iconlist_view: IconListView
//});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext<'_>) -> Self {
        // starter page: main view
        self.name("MainView").child(IconListView::new().build(ctx))
    }
}
