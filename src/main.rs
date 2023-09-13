#[derive(Debug)]
struct TodoItem {
    _id: u32,
    _name: String,
    _completed: bool,
    _created_time: u64,
    _completed_time: u64,
}

impl TodoItem {
    fn serialize(&self) -> String {
        format!(
            "{{\"id\":{},\"name\":\"{}\",\"completed\":{},\"created_time\":{},\"completed_time\":{}}}",
            self._id, self._name, self._completed, self._created_time, self._completed_time
        )
    }
}

struct Manager {
    idx: u32,
    todo_items: Vec<TodoItem>,
}
impl Manager {
    fn add_item(&mut self, name: &String) {
        let todo_item = TodoItem {
            _id: self.idx,
            _name: name.clone(),
            _completed: false,
            _created_time: 0,
            _completed_time: 0,
        };
        self.todo_items.push(todo_item);
        self.idx += 1;
    }

    fn _list_items(&self) {
        for item in &self.todo_items {
            println!("{:?}", item);
        }
    }

    fn save(&self) {
        for item in &self.todo_items {
            let str = item.serialize();
            println!("{}", str);
        }
    }
}

fn main() {
    let mut manager = Manager {
        idx: 0,
        todo_items: Vec::new(),
    };

    manager.add_item(&String::from("Learn Rust"));
    manager.save();
}