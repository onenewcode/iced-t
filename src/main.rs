use iced::{alignment, widget::{button, column, container, row, scrollable, text, text_input, Column}, Element, Length, Padding, Sandbox, Settings};
struct GroceryList {
	grocery_items: Vec<String>,
    input_value: String,
}
#[derive(Debug, Clone)]
enum Message {
	InputValue(String),
	Submitted,
    DeleteItem(usize),
}
impl Sandbox for GroceryList {
	type Message = Message;
	
	/* 初始化应用 */
    fn new() -> GroceryList {
        Self {
            grocery_items: vec![
                "Eggs".to_owned(), 
                "Milk".to_owned(), 
                "Flour".to_owned()
            ],
            input_value: String::default()
        }
    }
	
	/**
	* 窗口的标题。它将显示在应用程序窗口的顶部。
	*/
	fn title(&self) -> String {
		String::from("Grocery List App")
	}
	
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputValue(value) => self.input_value = value,
            Message::Submitted => {
                self.grocery_items.push(self.input_value.clone());
                self.input_value = String::default(); // Clear the input value
            }
            Message::DeleteItem(item) => {
                self.grocery_items.remove(item);
            },    
        }
    }
	fn view(&self) -> Element<Self::Message> {
        container(
            column!(
                items_list_view(&self.grocery_items),
                row!(
                    text_input("Input grocery item", &self.input_value)
                    .on_input(|value| Message::InputValue(value))
                    .on_submit(Message::Submitted), 
    
                    button("Submit")
                    .on_press(Message::Submitted)
                )
                .spacing(30)
                .padding(Padding::from(30))
            )
            .align_items(iced::Alignment::Center)
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

    for (index, value) in items.into_iter().enumerate() {
        column = column.push(grocery_item(index, value));
    }

    scrollable(
        container(
            column
        )
    )
    .height(250.0)
    .width(300)
    .into()
}
fn grocery_item(index: usize, value: &str) -> Element<'static, Message> {
    row!(
        text(value),
        button("Delete")
        .on_press(Message::DeleteItem(index))
    )
    .align_items(iced::Alignment::Center)
    .spacing(30)
    .into()
}
fn main() -> iced::Result {
	GroceryList::run(Settings::default())
}