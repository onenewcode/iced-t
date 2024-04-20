use iced::{alignment, Element, Length, Sandbox, Settings};
use iced::widget::{container, text};
struct GroceryList {}
#[derive(Debug, Clone)]
enum Message {}
impl Sandbox for GroceryList {
	type Message = Message;
	
	/* 初始化应用 */
	fn new() -> GroceryList {
		Self {}
	}
	
	/**
	* 窗口的标题。它将显示在应用程序窗口的顶部。
	*/
	fn title(&self) -> String {
		String::from("Grocery List App")
	}
	
	fn update(&mut self, message: Self::Message) {
		// 更新应用信息状态
	}
	
	fn view(&self) -> Element<Self::Message> {
        container(
            text("Hello World")
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }
    fn theme(&self) -> iced::Theme {
		iced::Theme::Dark
	}
}
fn main() -> iced::Result {
	GroceryList::run(Settings::default())
}