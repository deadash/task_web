use crate::components::toast::ToastType;
use crate::components::{create_task_modal::CreateTaskModal, toast::show_toast};
use crate::components::task_list::TaskList;
use crate::services::api;
use models::task::{Task, NewTask};
use models::user::User;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let current_user = use_state(|| None::<User>);
    let tasks = use_state(Vec::<Task>::new);
    let show_create_modal = use_state(|| false);

    let close_modal = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_| {
            show_create_modal.set(false);
        })
    };

    let toggle_create_modal = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_| {
            show_create_modal.set(!*show_create_modal);
        })
    };

    let on_create_task = {
        let tasks = tasks.clone();
        Callback::from(move |new_task: NewTask| {
            let tasks = tasks.clone();
            // 发起创建任务的异步请求
            let callback = Callback::from(move |response: Result<Task, anyhow::Error>| {
                match response {
                    Ok(created_task) => {
                        // 更新任务列表
                        tasks.set((*tasks).clone().into_iter().chain(std::iter::once(created_task)).collect::<Vec<Task>>());
                        // 可能需要关闭模态窗口或重置表单
                        // ...
                        show_toast("Create task success.", ToastType::Success);
                    }
                    Err(err) => {
                        show_toast(&format!("Error creating task: {}", err), ToastType::Error);
                    }
                }
            });

            // 调用 API 函数来创建任务
            api::create_task(new_task, callback);
        })
    };

    let on_refresh_tasks = {
        let tasks = tasks.clone();
        Callback::from(move |_| {
            let tasks = tasks.clone();
            // 发起异步获取任务列表请求
            api::get_tasks(Callback::from(move |response| {
                match response {
                    Ok(fetched_tasks) => {
                        tasks.set(fetched_tasks);
                        show_toast("Refresh tasks Ok.", ToastType::Success);
                    }
                    Err(err) => {
                        show_toast(&format!("Error loading tasks: {}", err), ToastType::Error);
                    }
                }
            }))
        })
    };

    let on_delete_task = {
        let tasks = tasks.clone();
        Callback::from(move |task_id: i64| {
            let tasks = tasks.clone();
            api::delete_task(task_id, Callback::from(move |response| {
                match response {
                    Ok(_) => {
                        // 这里可能需要更新任务列表等
                        show_toast("Delete task success.", ToastType::Success);
                    }
                    Err(err) => {
                        show_toast(&format!("Error deleting task: {}", err), ToastType::Error);
                    }
                }
            }))
        })
    };

    let on_stop_task = {
        let tasks = tasks.clone();
        Callback::from(move |task_id: i64| {
            let tasks = tasks.clone();
            api::stop_task(task_id, Callback::from(move |response| {
                match response {
                    Ok(_) => {
                        // 可能需要将任务状态更新为已停止
                        show_toast("Stop task success.", ToastType::Success);
                    }
                    Err(err) => {
                        show_toast(&format!("Error stopping task: {}", err), ToastType::Error);
                    }
                }
            }))
        })
    };

    // 查看任务日志的回调
    let on_view_logs = {
        let tasks = tasks.clone();
        Callback::from(move |task_id: i64| {
            let tasks = tasks.clone();
            api::get_task_logs(task_id, Callback::from(move |response| {
                match response {
                    Ok(logs) => {
                        // 这里可以处理日志的显示逻辑
                        show_toast("View logs success.", ToastType::Success);
                    }
                    Err(err) => {
                        show_toast(&format!("Error getting task logs: {}", err), ToastType::Error);
                    }
                }
            }))
        })
    };

    {
        let current_user = current_user.clone();
        use_effect_with(
            (),
            move |_| {
            let current_user = current_user.clone();
            api::get_current_user(Callback::from(move |response| match response {
                Ok(user) => {
                    current_user.set(Some(user));
                }
                Err(error) => {
                    show_toast(&format!("Error fetching current user: {:?}", error), ToastType::Error);
                }
            }));
        });
    }

    html! {
        <>
            <div class="container mx-auto my-8 space-x-2 relative">
                <button
                    class="btn btn-success btn-sm scale-90 hover:scale-100 transition-all duration-300"
                    onclick={toggle_create_modal}
                >
                    { "Create Task" }
                </button>
                <button
                    class="btn btn-outline btn-accent btn-sm scale-90 hover:scale-100 transition-all duration-300"
                    onclick={on_refresh_tasks}
                >
                    { "Refresh List" }
                </button>
                { if *show_create_modal {
                    html! { <CreateTaskModal on_close={close_modal.clone()} on_create={on_create_task.clone()}/> }
                } else {
                    html! {} 
                }}
                // 显示当前用户信息
                { if let Some(user) = (*current_user).as_ref() {
                    html! {
                        <div class="fixed top-4 right-4">
                            <div class="bg-opacity-0 shadow-lg p-2">
                                <div class="flex flex-col justify-start space-y-1">
                                    <div class="flex justify-between items-center">
                                        <span class="text-gray-600 font-sans">{ "用户名:" }</span>
                                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-pink-500 to-yellow-500">{ &user.username }</span>
                                    </div>
                                    <div class="flex justify-between items-center">
                                        <span class="text-gray-600 font-sans">{ "计算机:" }</span>
                                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-blue-500">{ &user.computer_name }</span>
                                    </div>
                                    <div class="flex justify-between items-center">
                                        <span class="text-gray-600 font-sans">{ "IP:" }</span>
                                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-purple-500 to-indigo-500">{ &user.ip }</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}
                <TaskList
                    tasks={(*tasks).clone()}
                    on_delete={on_delete_task}
                    on_stop={on_stop_task}
                    on_view_logs={on_view_logs}
                />
                // DaisyUI Toast 容器
                <div id="toast-container" class="fixed bottom-0 right-0 m-8 flex flex-col gap-2">
                </div>
            </div>
        </>
    }
}
