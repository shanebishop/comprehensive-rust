pub mod button;
pub mod label;
pub mod widget;
pub mod window;

use button::Button;
use label::Label;
use widget::Widget;
use window::Window;

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
