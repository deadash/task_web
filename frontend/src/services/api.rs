// src/api.rs

use gloo_net::http::{Request, Response};
use models::{task::{NewTask, Task}, user::User};
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use anyhow::Result;
use futures::future::LocalBoxFuture;

// 获取所有任务
pub fn get_tasks(callback: Callback<Result<Vec<Task>>>) {
    spawn_local(async move {
        let response = Request::get("/api/tasks")
            .send()
            .await;

        handle_response(response, callback).await;
    });
}

pub fn create_task(new_task: NewTask, callback: Callback<Result<Task>>) {
    let post_request = Request::post("/api/tasks")
        .json(&new_task).unwrap()
        .send();

    spawn_local(async move {
        match post_request.await {
            Ok(response) if response.status() == 200 || response.status() == 201 => {
                match response.json::<Task>().await {
                    Ok(task) => callback.emit(Ok(task)),
                    Err(error) => callback.emit(Err(anyhow::Error::new(error))),
                }
            },
            Ok(response) => {
                // 非成功状态码的情况，尝试获取错误信息的文本
                let error_message = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
                callback.emit(Err(anyhow::Error::msg(format!("HTTP Error {}: {}", response.status(), error_message))))
            },
            Err(error) => {
                // 网络错误或请求无法发送的情况
                callback.emit(Err(anyhow::Error::new(error)));
            },
        }
    });
}

// 停止任务
pub fn stop_task(task_id: i64, callback: Callback<Result<()>>) {
    spawn_local(async move {
        let response = Request::post(&format!("/api/tasks/{}/stop", task_id))
            .send()
            .await;

        handle_response(response, callback).await;
    });
}

// 删除任务
pub fn delete_task(task_id: i64, callback: Callback<Result<()>>) {
    spawn_local(async move {
        let response = Request::post(&format!("/api/tasks/{}/delete", task_id))
            .send()
            .await;

        handle_response(response, callback).await;
    });
}

// 获取任务日志
pub fn get_task_logs(task_id: i64, callback: Callback<Result<String>>) {
    spawn_local(async move {
        let response = Request::get(&format!("/api/tasks/{}/logs", task_id))
            .send()
            .await;

        handle_response(response, callback).await;
    });
}

// 获取所有分支
pub fn get_branches(callback: Callback<Result<Vec<String>>>) {
    spawn_local(async move {
        let response = Request::get("/api/branches")
            .send()
            .await;

        handle_response(response, callback).await;
    });
}

// 获取分支下的提交
// pub fn get_commits(branch_name: &str, callback: Callback<Result<Vec<String>>>) {
//     spawn_local(async move {
//         let response = Request::get(&format!("/api/branches/{}/commits", branch_name))
//             .send()
//             .await;

//         handle_response(response, callback).await;
//     });
// }

pub fn get_current_user(callback: Callback<Result<User, anyhow::Error>>) {
    spawn_local(async move {
        let response = Request::get("/api/current_user")
            .send()
            .await;

        handle_response(response, callback).await;
    });
}


trait HandleResponse<T> {
    fn handle_response(response: Response) -> LocalBoxFuture<'static, Result<T>>;
}

impl<T> HandleResponse<T> for T
where
    T: for<'de> Deserialize<'de> + 'static,
{
    default fn handle_response(response: Response) -> LocalBoxFuture<'static, Result<T>> {
        let future = async move {
            if response.status() == 200 {
                let text = response.text().await.unwrap_or_else(|_| "Failed to read response text".to_string());
                serde_json::from_str::<T>(&text).map_err(anyhow::Error::from)
            } else {
                let error_message = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
                Err(anyhow::Error::msg(format!("HTTP Error {}: {}", response.status(), error_message)))
            }
        };
        Box::pin(future)
    }
}

impl HandleResponse<()> for () {
    fn handle_response(response: Response) -> LocalBoxFuture<'static, Result<()>> {
        Box::pin(async move {
            if response.status() == 200 {
                Ok(())
            } else {
                let error_message = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
                Err(anyhow::Error::msg(format!("HTTP Error {}: {}", response.status(), error_message)))
            }
        })
    }
}

// 通用的响应处理函数
async fn handle_response<T: HandleResponse<T> + 'static>(
    response: Result<Response, gloo_net::Error>,
    callback: Callback<Result<T>>
) {
    match response {
        Ok(response) => {
            let future = T::handle_response(response);
            let pinned_future = Box::pin(future);
            let result = pinned_future.await;
            callback.emit(result);
        },
        Err(error) => {
            callback.emit(Err(anyhow::Error::new(error)));
        },
    }
}