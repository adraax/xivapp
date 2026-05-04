use std::rc::Rc;

use gpui::{
    AnyElement, App, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, div,
};
use gpui_component::{
    ActiveTheme, IconName, Sizable, TitleBar, WindowExt,
    badge::Badge,
    button::{Button, ButtonVariants},
    label::Label,
    menu::AppMenuBar,
};

use crate::app_menus;

type ElementFn = dyn Fn(&mut Window, &mut App) -> AnyElement;

pub struct AppTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
    child: Rc<ElementFn>,
}

impl AppTitleBar {
    pub fn new(title: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let app_menu_bar = app_menus::init(title, cx);

        Self {
            app_menu_bar,
            child: Rc::new(|_, _| div().into_any_element()),
        }
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notifications_count = window.notifications(cx).len();

        TitleBar::new()
            .child(div().flex().items_center().child(self.app_menu_bar.clone()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .child((self.child.clone())(window, cx))
                    .child(
                        Label::new("Theme : ")
                            .secondary(cx.theme().theme_name())
                            .text_sm(),
                    )
                    .child(
                        div().relative().child(
                            Badge::new().count(notifications_count).max(99).child(
                                Button::new("notification_baell")
                                    .small()
                                    .ghost()
                                    .compact()
                                    .icon(IconName::Bell),
                            ),
                        ),
                    ),
            )
    }
}
