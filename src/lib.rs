use seed::{prelude::*, *};

// Init
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}

// Model
struct Model {
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
    base_url: Url,
}

struct Todo {
    id: ID,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

enum Filter {
    All,
    Active,
    Completed,
}

// Messages (event signalling)
enum Msg {
    // -- Commands -- //
    CreateTodo,
    CheckOrUncheckAll,
    ClearCompleted,
    ToggleTodo(Ulid),
    SelectTodo(Ulid),
    RemoveTodo(Ulid),
    SaveSelectedTodo,
    ClearCompleted,

    // -- Events -- //
    NewTodoTitleChanged(String),
    SelectedTodoTitleChanged(String),
    UrlChanged(subs::UrlChanged),
}
