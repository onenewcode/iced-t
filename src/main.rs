use iced::{alignment, Element, Length, Sandbox, Settings};
use iced::widget::{container, text, Column, column};
struct GroceryList {
	grocery_items: Vec<String>
}
#[derive(Debug, Clone)]
enum Message {}
impl Sandbox for GroceryList {
	type Message = Message;
	
	/* 初始化应用 */
	fn new() -> GroceryList {
		Self {
			grocery_items: vec![	
				"Eggs".to_owned(),
				"Milk".to_owned(),
				"Flour".to_owned()
			]
		}
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
       
            items_list_view(&self.grocery_items),
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
// 设置数据排列方式
fn items_list_view(items: &Vec<String>) -> Element<'static, Message> {
	let mut column = Column::new()
	.spacing(20)
	.align_items(iced::Alignment::Center)
	.width(Length::Fill);
	
	for value in items {
		column = column.push(text(value));
	}  
	
	container(
		column
	)
	.height(250.0)
	.width(300)
	.into()
}
fn main() -> iced::Result {
	GroceryList::run(Settings::default())
}