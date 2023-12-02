#[derive(sqlx::FromRow)]
pub struct Todo {
    pub title: String,
    pub content: String,
}

markup::define! {
  MainPage<'a>(title: &'a str, name: &'a str, todos: &'a Vec<Todo>) {
    html {
      head[title = title]{}
      link[rel = "stylesheet", href="./style/page.css"]{}
      body {
        @User { name }
        @TodoList { todos }
      }
    }
  }
  User<'a>(name: &'a str) {
    div.some_class {
      p.greeting {
        {format!("Welcome, {}", name)}
      }
    }
  }
  TodoList<'a>(todos: &'a Vec<Todo>) {
    @for todo in todos.iter() {
      @TodoItem { todo }
    }
  }
  TodoItem<'a>(todo: &'a Todo) {
    ul.todo {
      li {
        h2 {
          @todo.title
        }
        p {
          @todo.content
        }
      }
    }
  }
}

