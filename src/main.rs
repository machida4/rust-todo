#[derive(Clone, Debug)]
enum Status {
    Doing,
    Done,
}

#[derive(Clone, PartialEq, Debug)]
struct TodoId(u32);

struct DraftTodo {
    content: String,
}

#[derive(Clone, Debug)]
struct Todo {
    id: TodoId,
    content: String,
    status: Status,
}

struct TodoManager {
    todos: Vec<Todo>,
    current_id: u32,
}

#[derive(Debug)]
enum TodoError {
    ContentTooLong,
    NotFound,
}

impl TodoManager {
    pub fn new() -> Self {
        TodoManager {
            todos: Vec::new(),
            current_id: 1,
        }
    }

    pub fn all(&self) -> &[Todo] {
        &self.todos
    }

    pub fn find(&self, id: TodoId) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    pub fn create(&mut self, content: String) -> Result<Todo, TodoError> {
        let draft_todo = DraftTodo { content };
        TodoManager::validate_content(&draft_todo)?;

        let new_todo = Todo {
            id: TodoId(self.current_id),
            content: draft_todo.content,
            status: Status::Doing,
        };

        self.current_id += 1;
        self.todos.push(new_todo.clone());

        Ok(new_todo)
    }

    pub fn delete(&mut self, id: TodoId) -> Result<(), TodoError> {
        if let Some(index) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.remove(index);
            Ok(())
        } else {
            Err(TodoError::NotFound)
        }
    }

    // TODO: Doing -> Doneにできるようにする
    // TODO: Doingのものだけ引っ張ってくる

    fn validate_content(draft_todo: &DraftTodo) -> Result<(), TodoError> {
        if draft_todo.content.len() > 100 {
            Err(TodoError::ContentTooLong)
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut manager = TodoManager::new();

    // 複数のTodoを追加
    match manager.create("Learn Rust".to_string()) {
        Ok(todo) => println!("Created Todo: {:?}", todo),
        Err(e) => println!("Error: {:?}", e),
    }

    match manager.create("Practice coding".to_string()) {
        Ok(todo) => println!("Created Todo: {:?}", todo),
        Err(e) => println!("Error: {:?}", e),
    }

    match manager.create("Write documentation".to_string()) {
        Ok(todo) => println!("Created Todo: {:?}", todo),
        Err(e) => println!("Error: {:?}", e),
    }

    // 全てのTodoを表示
    println!("All Todos: {:?}", manager.all());

    // 特定のTodoを削除 (ID 2のTodoを削除)
    match manager.delete(TodoId(2)) {
        Ok(_) => println!("Todo with ID 2 removed."),
        Err(e) => println!("Error removing Todo: {:?}", e),
    }

    // Todoリストを再度表示（ID 2が削除されているはず）
    println!("All Todos after removal: {:?}", manager.all());

    // 存在しないIDで削除を試みる（エラー処理）
    match manager.delete(TodoId(10)) {
        Ok(_) => println!("Todo with ID 10 removed."),
        Err(e) => println!("Error removing Todo: {:?}", e),
    }

    // 再度追加
    match manager.create("Prepare for meeting".to_string()) {
        Ok(todo) => println!("Created Todo: {:?}", todo),
        Err(e) => println!("Error: {:?}", e),
    }

    // 最終的なTodoリストを表示
    println!("Final Todos: {:?}", manager.all());
}
