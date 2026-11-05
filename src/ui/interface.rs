use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, rgb};

pub struct MainInterface {
    pub count: i32,
}

impl Render for MainInterface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_3()
            .p_4()
            .bg(rgb(0x131315))
            .text_base()
            .text_color(rgb(0xFFFFFF))
            .child("Hello GPUI-ce!")
    }
}
