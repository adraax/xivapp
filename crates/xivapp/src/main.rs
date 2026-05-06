#![warn(clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::allow_attributes_without_reason,
    reason = "I'm not a masochist"
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::missing_docs_in_private_items
)]

use gpui::{
    App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Styled as _,
    Window, div, prelude::FluentBuilder as _,
};
use gpui_component::{
    Icon, IconName, StyledExt as _, h_flex,
    sidebar::{
        Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarToggleButton,
    },
};
use gpui_component_assets::Assets;
use xivapp::{create_new_window, init};

pub struct HelloWorld {
    collapsed: bool,
}

impl Render for HelloWorld {
    #[inline]
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().v_flex().child(
            Sidebar::new("sidebar")
                .collapsed(self.collapsed)
                .collapsible(true)
                .header(
                    SidebarHeader::new().child(
                        h_flex()
                            .child(Icon::new(IconName::Heart).mr_2())
                            .when(!self.collapsed, |this| this.child("Home")),
                    ),
                )
                .child(SidebarGroup::new("Menu").child(
                    SidebarMenu::new().child(SidebarMenuItem::new("Files").icon(IconName::Folder)),
                ))
                .footer(
                    SidebarToggleButton::new()
                        .collapsed(self.collapsed)
                        .on_click(cx.listener(|this, _, _, _| this.collapsed = !this.collapsed)),
                ),
        )
    }
}

impl HelloWorld {
    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { collapsed: false }
    }

    fn view(window: &mut Window, app: &mut App) -> Entity<Self> {
        app.new(|cx| Self::new(window, cx))
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(Assets);
    app.run(move |cx| {
        init(cx);
        cx.activate(true);

        create_new_window("XIV Toolbox", HelloWorld::view, cx);
    });
}
