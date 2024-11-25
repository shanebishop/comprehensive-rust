pub mod window;
pub mod widget;
pub mod label;
pub mod button;

use widget::Widget;
use window::Window;
use label::Label;
use button::Button;

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
