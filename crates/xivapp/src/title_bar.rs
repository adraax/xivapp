use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext, Context, Entity, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, div,
};
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt, ThemeRegistry, TitleBar, WindowExt,
    badge::Badge,
    button::{Button, ButtonVariants},
    dialog::{Dialog, DialogHeader, DialogTitle},
    label::Label,
    menu::AppMenuBar,
    select::{SearchableVec, Select, SelectDelegate, SelectState},
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
        let original_theme = cx.theme().clone();
        let mut raw_themes = ThemeRegistry::global(cx)
            .themes()
            .values()
            .collect::<Vec<_>>();

        raw_themes.sort_by(|a, b| {
            b.is_default
                .cmp(&a.is_default)
                .then_with(|| a.mode.cmp(&b.mode).reverse())
                .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        let themes = SearchableVec::new(
            raw_themes
                .iter()
                .map(|el| el.name.clone())
                .collect::<Vec<_>>(),
        );

        let cloned_theme = themes.clone();

        let state = cx.new(|cx| {
            SelectState::new(
                themes,
                Some(
                    cloned_theme
                        .position(original_theme.theme_name())
                        .unwrap_or_default(),
                ),
                window,
                cx,
            )
        });

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
                        Dialog::new(cx)
                            .trigger(
                                Button::new("theme-button")
                                    .child(
                                        Label::new("Theme : ")
                                            .secondary(cx.theme().theme_name())
                                            .text_sm(),
                                    )
                                    .ghost()
                                    .dropdown_caret(true),
                            )
                            .content(move |content, _window, _cx| {
                                content
                                    .child(
                                        DialogHeader::new()
                                            .child(DialogTitle::new().child("Dialog")),
                                    )
                                    .child(
                                        div().v_flex().gap_3().child(
                                            Select::new(&state)
                                                .placeholder("Theme ...")
                                                .title_prefix("Theme : "),
                                        ),
                                    )
                            }),
                    )
                    .child(
                        div().relative().child(
                            Badge::new().count(notifications_count).max(99).child(
                                Button::new("notification_bell")
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
