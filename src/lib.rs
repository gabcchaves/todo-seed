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

// Update
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        // -- Commands -- //
        Msg::CreateTodo => {
            log!("CreateTodo");
        }
        Msg::CheckOrUncheckAll => {
            log!("CheckOrUncheckAll");
        }
        Msg::ClearCompleted => {
            log!("ClearCompleted");
        }
        Msg::ToggleTodo => {
            log!("ToggleTodo");
        }
        Msg::SelectTodo => {
            log!("SelectTodo");
        }
        Msg::RemoveTodo => {
            log!("RemoveTodo");
        }
        Msg::SaveSelectedTodo => {
            log!("SaveSelectedTodo");
        }
        Msg::ClearCompleted => {
            log!("ClearCompleted");
        }
        // -- Events -- //
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("UrlChanged", url);
        }
        Msg::NewTodoTitleChanged(title) => {
            log!("NewTodoTitleChanged", title);
        }
        Msg::SelectedTodoTitleChanged(title) => {
            log!("SelectedTodoTitleChanged", title);
        }
    }
}
