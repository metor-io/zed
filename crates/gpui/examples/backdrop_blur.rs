#![cfg_attr(target_family = "wasm", no_main)]

//! Frosted-glass cards over a busy background. The cards along the top use
//! increasing blur radii; the large card drifts back and forth so you can see
//! the backdrop update every frame. Metal only: other renderers draw the
//! cards unblurred.

#[path = "example_support/fonts.rs"]
mod example_support;

use std::time::Instant;

use gpui::{
    App, Bounds, Context, Hsla, Window, WindowBounds, WindowOptions, div, hsla, prelude::*, px,
    size,
};
use gpui_platform::application;

struct BackdropBlurExample {
    start: Instant,
}

fn card(blur: f32, label: String) -> gpui::Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(16.))
        .border_1()
        .border_color(hsla(0., 0., 1., 0.35))
        .bg(hsla(0., 0., 1., 0.18))
        .backdrop_blur(px(blur))
        .text_color(gpui::white())
        .text_xl()
        .child(label)
}

impl Render for BackdropBlurExample {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        window.request_animation_frame();
        let t = self.start.elapsed().as_secs_f32();
        let drift = (t * 0.6).sin() * 0.5 + 0.5;

        let colors: Vec<Hsla> = (0..12)
            .map(|i| hsla(i as f32 / 12., 0.75, 0.55, 1.))
            .collect();

        div()
            .size_full()
            .relative()
            .bg(hsla(0.62, 0.3, 0.12, 1.))
            .text_color(gpui::white())
            // Background: diagonal bands of color plus some sharp text.
            .child(div().absolute().size_full().flex().flex_row().children(
                colors.iter().enumerate().map(|(i, color)| {
                    div()
                        .w(px(70.))
                        .h_full()
                        .bg(*color)
                        .when(i % 2 == 1, |this| this.mt(px(80.)))
                }),
            ))
            .child(
                div()
                    .absolute()
                    .top(px(260.))
                    .left(px(20.))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .text_3xl()
                    .children(
                        (0..6).map(|i| format!("{i} The quick brown fox jumps over the lazy dog")),
                    ),
            )
            // Cards with increasing blur radius.
            .child(
                div()
                    .absolute()
                    .top(px(40.))
                    .left(px(40.))
                    .flex()
                    .gap_6()
                    .children(
                        [0., 2., 6., 12., 24.]
                            .into_iter()
                            .map(|blur| card(blur, format!("{blur}px")).w(px(140.)).h(px(140.))),
                    ),
            )
            // A drifting card, so the backdrop changes under it every frame.
            .child(
                card(16., "frosted".into())
                    .absolute()
                    .top(px(300.))
                    .left(px(40. + drift * 440.))
                    .w(px(320.))
                    .h(px(200.)),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        if !example_support::load_fonts(cx) {
            return;
        }
        let bounds = Bounds::centered(None, size(px(900.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| BackdropBlurExample {
                    start: Instant::now(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
