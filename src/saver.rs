use std::{fs, path::Path};

use directories::BaseDirs;
use druid::Widget;
use serde::{Deserialize, Serialize};

use crate::data::{TodoItem, TodoState};

pub struct Saver;

// Custom widget
impl Widget<TodoState> for Saver {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut TodoState,
        _env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &TodoState,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        _env: &druid::Env,
    ) {
        if data.todos != old_data.todos {
            let config = read_state_path();
            let config_path = Path::new(&config);

            let tasks = TaskData {
                tasks: data.todos.clone().into_iter().collect(),
            };

            fs::write(config_path, serde_json::to_string(&tasks).unwrap())
                .expect("Config path does not fully exist");

            println!("Saved successfully")
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &TodoState,
        _env: &druid::Env,
    ) -> druid::Size {
        druid::Size {
            width: 0.,
            height: 0.,
        }
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &TodoState, _env: &druid::Env) {}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskData {
    pub tasks: Vec<TodoItem>,
}

pub fn read_stored() -> TaskData {
    let default_data = TaskData { tasks: Vec::new() };

    let config = read_state_path();
    let config_path = Path::new(&config);

    let data = match fs::read_to_string(config_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Cannot read from given path {}", config);
            eprintln!("{:?}", e);
            return default_data;
        }
    };

    match serde_json::from_str(&data) {
        Ok(d) => {
            println!("Read successfully");
            return d;
        }
        Err(e) => {
            eprintln!("Data corrupted can no longer use it");
            eprintln!("{:?}", e);
            return default_data;
        }
    };
}

fn read_state_path() -> String {
    let base_dirs = BaseDirs::new().expect("Did not found path");

    format!(
        "{}/{}",
        base_dirs.home_dir().to_str().unwrap(),
        "state.json"
    )
}
