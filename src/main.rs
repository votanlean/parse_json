use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //get
    let list_todos = get_todos().await?;
    println!("List todos: {:?}", list_todos); //List todos: [Todo { user_id: 1, id: Some(1), title: "delectus aut autem", completed: false }, Todo { user_id: 1, id: Some(2), title: "quis ut nam facilis et officia qui", completed: false }, Todo { user_id: 1, id: Some(3), title: "fugiat veniam minus", completed: false }, Todo { user_id: 1, id: Some(4), title: "et porro tempora", completed: true }, Todo { user_id: 1, id: Some(5), title: "laboriosam mollitia et enim quasi adipisci quia provident illum", completed: false }, Todo { user_id: 1, id: Some(6), title: "qui ullam ratione quibusdam voluptatem quia omnis", completed: false }, Todo { user_id: 1, id: Some(7), title: "illo expedita consequatur quia in", completed: false }, Todo { user_id: 1, id: Some(8), title: "quo adipisci enim quam ut ab", completed: true }, Todo { user_id: 1, id: Some(9), title: "molestiae perspiciatis ipsa", completed: false }, Todo { user_id: 1, id: Some(10), title: "illo est ratione doloremque quia maiores aut", completed: true }, Todo { user_id: 1, id: Some(11), title: "vero rerum temporibus dolor", completed: true }, Todo { user_id: 1, id: Some(12), title: "ipsa repellendus fugit nisi", completed: true }, Todo { user_id: 1, id: Some(13), title: "et doloremque nulla", completed: false }, Todo { user_id: 1, id: Some(14), title: "repellendus sunt dolores architecto voluptatum", completed: true }, Todo { user_id: 1, id: Some(15), title: "ab voluptatum amet voluptas", completed: true }, Todo { user_id: 1, id: Some(16), title: "accusamus eos facilis sint et aut voluptatem", completed: true }, Todo { user_id: 1, id: Some(17), title: "quo laboriosam deleniti aut qui", completed: true }, Todo { user_id: 1, id: Some(18), title: "dolorum est consequatur ea mollitia in culpa", completed: false }, Todo { user_id: 1, id: Some(19), title: "molestiae ipsa aut voluptatibus pariatur dolor nihil", completed: true }, Todo { user_id: 1, id: Some(20), title: "ullam nobis libero sapiente ad optio sint", completed: true }]

    //post struct todo
    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "Into Rust".to_owned(),
        completed: false,
    };
    let new_struct_todo = post_struct_todo(&new_todo).await?;
    println!("New struct todo: {:?}", new_struct_todo); //New struct todo: Todo { user_id: 1, id: Some(201), title: "Into Rust", completed: false }

    //post arbitrary json
    let new_arbitrary_json_todo = post_arbitrary_json_todo(&serde_json::json!({
        "userId": 1,
        "title": "foo",
        "completed": false,
    }))
    .await?;
    println!("New arbitrary json todo: {:?}", new_arbitrary_json_todo); //New arbitrary json todo: Object {"completed": Bool(false), "id": Number(201), "title": String("foo"), "userId": Number(1)}

    Ok(())
}

async fn get_todos() -> Result<Vec<Todo>, reqwest::Error> {
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    Ok(todos)
}

async fn post_struct_todo(todo: &Todo) -> Result<Todo, reqwest::Error> {
    let todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(todo)
        .send()
        .await?
        .json()
        .await?;

    Ok(todo)
}

async fn post_arbitrary_json_todo(
    todo: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
    let todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(todo)
        .send()
        .await?
        .json()
        .await?;

    Ok(todo)
}
