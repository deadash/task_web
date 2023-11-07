// src/handlers.rs

use std::net::{SocketAddr, IpAddr};

use axum::{
    extract::{Extension, Path},
    Json, response::IntoResponse, http::StatusCode,
};
use sqlx::SqlitePool;
use tokio::sync::mpsc::UnboundedSender;

use models::{task::{Task, NewTask}, user::User};

use crate::tasks::TaskEvent;

pub async fn get_tasks(Extension(db_pool): Extension<SqlitePool>) -> Json<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&db_pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(tasks)
}

pub async fn create_task(
    Extension(db_pool): Extension<SqlitePool>,
    Extension(tx): Extension<UnboundedSender<TaskEvent>>,
    Json(new_task): Json<NewTask>,
) -> impl IntoResponse {
    if new_task.creator == "error" {
        return (StatusCode::INTERNAL_SERVER_ERROR, "测试错误！！！！！").into_response();
    }
    let task: Task = sqlx::query_as(
        r#"
        INSERT INTO tasks (created_at, creator, branch, svn_merge_number, status)
        VALUES (CURRENT_TIMESTAMP, ?1, ?2, ?3, ?4)
        RETURNING id, created_at, creator, branch, svn_merge_number, status
        "#,
    )
    .bind(new_task.creator)
    .bind(new_task.branch)
    .bind(new_task.svn_merge_number)
    .bind("Pending") // 默认状态值
    .fetch_one(&db_pool)
    .await
    .unwrap_or_else(|e| {
        eprintln!("Failed to create task: {}", e);
        panic!("Failed to create task");
    });

    (StatusCode::CREATED, Json(task)).into_response()
}

pub async fn stop_task(
    Path(task_id): Path<i64>,
    Extension(db_pool): Extension<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE tasks SET status = 'Stopped' WHERE id = ?")
        .bind(task_id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => (StatusCode::OK, "Task stopped").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to stop task: {}", e)).into_response(),
    }
}

pub async fn delete_task(
    Path(task_id): Path<i64>,
    Extension(db_pool): Extension<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(task_id)
        .execute(&db_pool)
        .await
    {
        Ok(_) => (StatusCode::OK, "Task deleted").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete task: {}", e)).into_response(),
    }
}

pub async fn get_logs(
    Path(task_id): Path<i64>,
    Extension(db_pool): Extension<SqlitePool>,
) -> impl IntoResponse {
    // 这里的逻辑取决于你如何存储日志信息
    // 以下是一个示例，如果你有一个日志字段
    let log = sqlx::query_as::<_, (String,)>("SELECT log FROM tasks WHERE id = ?")
        .bind(task_id)
        .fetch_one(&db_pool)
        .await
        .map(|(log,)| log)
        .unwrap_or_else(|_| "No log found".to_string());

    (StatusCode::OK, log)
}

pub async fn get_branches(
) -> Json<Vec<String>> {
    // 这里需要与你的版本控制系统集成，以下是一个模拟示例
    let branches = vec!["main".to_string(), "dev".to_string(), "feature-xyz".to_string()];
    Json(branches)
}

pub async fn get_commits(
    Path(branch_name): Path<String>,
) -> Json<Vec<String>> {
    // 这里需要与你的版本控制系统集成，以下是一个模拟示例
    let commits = vec![
        "commit1".to_string(),
        "commit2".to_string(),
        // ...更多提交
    ];
    Json(commits)
}

async fn create_user_from_ip(ip: IpAddr) -> anyhow::Result<User>
{
    let user = User {
        username: "example_user".to_string(),
        computer_name: "example_computer".to_string(),
        ip: ip.to_string(),
    };

    Ok(user)
}

pub async fn get_current_user(
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,
) -> Result<Json<User>, StatusCode> {
    // 使用客户端IP生成用户逻辑
    let user = create_user_from_ip(addr.ip()).await.map_err(|e| {
        eprintln!("Failed to create user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(user))
}