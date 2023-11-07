use models::task::NewTask;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct CreateTaskModalProps {
    pub on_close: Callback<()>, // 接收关闭回调
    pub on_create: Callback<NewTask>,
}

#[function_component(CreateTaskModal)]
pub fn create_task_modal(props: &CreateTaskModalProps) -> Html {
    let name = use_state(|| "".to_string());
    let branch = use_state(|| "".to_string());
    let svn_merge_number = use_state(|| "".to_string());

    let on_submit = {
        let name = name.clone();
        let branch = branch.clone();
        let svn_merge_number = svn_merge_number.clone();
        let on_close = props.on_close.clone();
        let on_create = props.on_create.clone();
        Callback::from(move |_| {
            let task = NewTask {
                creator: (*name).clone(),
                branch: (*branch).clone(),
                svn_merge_number: (*svn_merge_number).clone(),
            };
            // 发送任务创建请求
            on_create.emit(task);
            // 清空表单字段并关闭模态框
            name.set("".to_string());
            branch.set("".to_string());
            svn_merge_number.set("".to_string());
            on_close.emit(());
        })
    };

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal modal-open">
            <div class="modal-box">
                <h3 class="font-bold text-lg">{ "Create New Task" }</h3>
                <input
                    type="text"
                    placeholder="Task Name"
                    class="input input-bordered w-full my-2"
                    value={(*name).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                        name.set(input.value());
                    })}
                />
                <input
                    type="text"
                    placeholder="Branch Name"
                    class="input input-bordered w-full my-2"
                    value={(*branch).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                        branch.set(input.value());
                    })}
                />
                <input
                    type="text"
                    placeholder="SVN Merge Number"
                    class="input input-bordered w-full my-2"
                    value={(*svn_merge_number).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                        svn_merge_number.set(input.value());
                    })}
                />
                <div class="modal-action">
                    <button class="btn btn-primary" onclick={on_submit}>{ "Add Task" }</button>
                    <button class="btn btn-ghost" onclick={on_close}>{ "Close" }</button>
                </div>
            </div>
        </div>
    }
}
