use std::fs;

use clap::Args;

#[derive(Args, Default)]
pub(crate) struct ExampleArgs {
    /// 执行实例程序的名称
    #[clap(short, long)]
    name: String,
}

impl ExampleArgs {
    pub fn run(self) {
        match fs::read_dir("./examples") {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    if entry.file_name().to_str().unwrap()== self.name{
                        

                    }
                }
            }
            Err(why) => println!("Failed to read directory: {}", why),
        }
    }
}
