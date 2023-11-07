use yew::prelude::*;
use models::task::Task;

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<Task>,
    pub on_delete: Callback<i64>,
    pub on_stop: Callback<i64>,
    pub on_view_logs: Callback<i64>,
}

#[function_component(TaskList)]
pub fn task_list(props: &TaskListProps) -> Html {
    html! {
        <div class="overflow-x-auto mt-8">
            <table class="table w-full">
                <thead>
                    <tr>
                        <th>{ "ID" }</th>
                        <th>{ "Created At" }</th>
                        <th>{ "Creator" }</th>
                        <th>{ "Branch" }</th>
                        <th>{ "SVN Merge Number" }</th>
                        <th>{ "Status" }</th>
                        <th>{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for props.tasks.iter().map(|task| {
                            let on_delete = {
                                let on_delete = props.on_delete.clone();
                                let task_id = task.id;
                                Callback::from(move |_| on_delete.emit(task_id))
                            };
                            let on_stop = {
                                let on_stop = props.on_stop.clone();
                                let task_id = task.id;
                                Callback::from(move |_| on_stop.emit(task_id))
                            };
                            let on_view_logs = {
                                let on_view_logs = props.on_view_logs.clone();
                                let task_id = task.id;
                                Callback::from(move |_| on_view_logs.emit(task_id))
                            };
                            html! {
                                <tr key={task.id}>
                                    <td>{ task.id }</td>
                                    <td>{ &task.created_at }</td>
                                    <td>{ &task.creator }</td>
                                    <td>{ &task.branch }</td>
                                    <td>{ &task.svn_merge_number }</td>
                                    <td>{ &task.status }</td>
                                    <td class="flex items-center space-x-2">
                                        <button class="btn btn-ghost btn-xs btn-outline btn-info" onclick={on_view_logs}>{ "Logs" }</button>
                                        {
                                            if task.status == "Running" {
                                                html! {
                                                    <button class="btn btn-ghost btn-outline btn-warning" onclick={on_stop}>
                                                        { "Stop" }
                                                    </button>
                                                }
                                            } else {
                                                html! {
                                                    <button class="btn btn-ghost btn-xs btn-outline btn-error" onclick={on_delete}>
                                                        { "Delete" }
                                                    </button>
                                                }
                                            }
                                        }
                                    </td>
                                </tr>
                            }
                        })
                    }
                </tbody>
            </table>
        </div>
    }
}
