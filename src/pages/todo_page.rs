use leptos::ev::SubmitEvent;
use leptos::html::Input;
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

    let todo_input_ref: NodeRef<Input> = create_node_ref();

    let last_todo_id = move || todo_items().iter().map(|todo| todo.id).max();

    let on_submit_handler = move |ev: SubmitEvent| {
        ev.prevent_default();

        let mut new_todo_items = todo_items();
        let todo_id = last_todo_id().unwrap_or_default() + 1;

        new_todo_items.push(TodoItem {
            id: todo_id,
            task: todo_input_ref().expect("<input> to exist").value(),
            status: false,
        });
        set_todo_items.set(new_todo_items);
    };

    let delete_todo_item = move |todo_id: i16| {
        set_todo_items
            .update(move |todo_items| todo_items.retain(|todo_item| todo_item.id != todo_id))
    };
    view! {
        <Todo_Wrapper>
            <div class="flex flex-col mb-20 text-black rounded">
                <h2 class="mb-4 text-2xl font-medium">"Add Task"</h2>
                <form on:submit=on_submit_handler class="flex flex-col w-full">
                    <div class="flex justify-between items-center">
                        <input
                            class="py-1 px-2 w-2/3 border-b-2 border-black focus:outline-none"
                            type="text"
                            placeholder="Add new task"
                            node_ref=todo_input_ref
                        />
                        <input
                            class="hover:cursor-pointer"
                            type="submit"
                            value="Submit"
                        />
                    </div>
                </form>
            </div>
            <div class="h-screen">
                <div class="px-5 pt-20 mx-auto mb-10 w-full lg:px-0 lg:max-w-[90ch]">
                    <For
                        // or just todo_items, both works
                        each=move || todo_items.get()
                        key=|task| task.id
                        children=move |task: TodoItem| {
                            view! {
                                <Todo_Item
                                    delete_callback=delete_todo_item
                                    todo_item=task
                                />
                            }
                        }
                    />

                </div>
            </div>
        // <Post />
        </Todo_Wrapper>
    }
}
