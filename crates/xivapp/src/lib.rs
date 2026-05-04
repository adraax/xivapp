use gpui::{
    AnyView, App, AppContext, Bounds, Entity, FocusHandle, Focusable, InteractiveElement as _,
    ParentElement, Pixels, Render, SharedString, Size, Styled as _, Window, WindowBounds,
    WindowOptions, div, px, size,
};
use gpui_component::{Root, TitleBar, v_flex};

use crate::title_bar::AppTitleBar;

mod app_menus;
mod themes;
mod title_bar;

pub struct AppRoot {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) title_bar: Entity<AppTitleBar>,
    pub(crate) view: AnyView,
}

impl AppRoot {
    pub fn new(title: impl Into<SharedString>, view: impl Into<AnyView>, cx: &mut App) -> Self {
        let title_bar = cx.new(|cx| AppTitleBar::new(title, cx));

        Self {
            focus_handle: cx.focus_handle(),
            title_bar,
            view: view.into(),
        }
    }
}

impl Focusable for AppRoot {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AppRoot {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div().id("app-root").size_full().child(
            v_flex()
                .size_full()
                .child(self.title_bar.clone())
                .child(
                    div()
                        .track_focus(&self.focus_handle)
                        .flex_1()
                        .overflow_hidden()
                        .child(self.view.clone()),
                )
                .children(sheet_layer)
                .children(dialog_layer)
                .children(notification_layer),
        )
    }
}

pub fn init(cx: &mut App) {
    gpui_component::init(cx);
    themes::init(cx);
}

pub fn create_new_window<F, E>(title: &str, create_view_fn: F, cx: &mut App)
where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    create_new_window_with_size(title, None, create_view_fn, cx);
}

pub fn create_new_window_with_size<F, E>(
    title: &str,
    window_size: Option<Size<Pixels>>,
    create_view_fn: F,
    cx: &mut App,
) where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    let mut window_size = window_size.unwrap_or(size(px(1600.), px(1200.)));

    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }

    let windows_bound = Bounds::centered(None, window_size, cx);
    let title = SharedString::from(title.to_string());

    cx.spawn(async move |cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(windows_bound)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(Size {
                width: px(400.),
                height: px(400.),
            }),
            kind: gpui::WindowKind::Normal,
            ..Default::default()
        };

        let window = cx
            .open_window(options, |window, cx| {
                let view = create_view_fn(window, cx);
                let app_root = cx.new(|cx| AppRoot::new(title.clone(), view, cx));

                let focus_handle = app_root.focus_handle(cx);
                window.defer(cx, move |window, cx| {
                    if window.focused(cx).is_none() {
                        focus_handle.focus(window, cx);
                    }
                });

                cx.new(|cx| Root::new(app_root, window, cx))
            })
            .expect("failed to open window");

        window.update(cx, |_, window, _| {
            window.activate_window();
            window.set_window_title(&title);
        })?;

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}
