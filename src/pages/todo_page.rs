use leptos::*;

use crate::components::aggregates::{TodoItem, Todo_Item};
use crate::components::layouts::Todo_Wrapper;

#[component]
pub fn TodoList() -> impl IntoView {
    let (todo_items, set_todo_items) = create_signal::<Vec<TodoItem>>(vec![
        TodoItem {
            id: 0,
            task: String::from("Take out the trash"),
            status: false,
        },
        TodoItem {
            id: 1,
            task: String::from("Make the bed"),
            status: false,
        },
        TodoItem {
            id: 2,
            task: String::from("Mow the lawn"),
            status: true,
        },
        TodoItem {
            id: 3,
            task: String::from("Wash the dishes"),
            status: false,
        },
    ]);

    let delete_todo_item = move |todo_id: i16| {
        set_todo_items
            .update(move |todo_items| todo_items.retain(|todo_item| todo_item.id != todo_id))
    };
    view! {
        <Todo_Wrapper>
            <div class="flex flex-col rounded mb-20 text-black">
                <h2 class="text-2xl font-medium mb-4">"Add Task"</h2>
                <form class="w-full flex flex-col">
                    <div class="flex items-center justify-between">
                        <input
                            class="w-2/3 px-2 py-1 border-b-2 border-black focus:outline-none"
                            type="text"
                            placeholder="Add new task"
                        />
                        <input class="hover:cursor-pointer" type="submit" value="Submit"/>
                    </div>
                </form>
            </div>
            <div class="h-screen">
                <div class="pt-20 mb-10 mx-auto px-5 w-full lg:px-0 lg:max-w-[90ch]">
                    <For
                        // or just todo_items, both works
                        each=move || todo_items.get()
                        key=|task| task.id
                        children=move |task: TodoItem| {
                            view! { <Todo_Item delete_callback=delete_todo_item todo_item=task/> }
                        }
                    />

                </div>
            </div>
        </Todo_Wrapper>
    }
}
