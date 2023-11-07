use axum::{
    routing::{get, post},
    Router
};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::sync::mpsc::unbounded_channel;
use std::net::SocketAddr;

mod handlers;
mod tasks;

// 启动服务
#[tokio::main]
async fn main() {
    // 设置 SQLite 数据库连接池
    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite:task.db")
        .await
        .expect("Could not connect to the database.");

    // 创建一个无界任务通知的通道
    let (tx, rx) = unbounded_channel::<tasks::TaskEvent>();

    // 创建任务处理器
    let mut task_processor = tasks::TaskProcessor::new(db_pool.clone(), rx).await;

    // 启动一个后台任务处理器
    tokio::spawn(async move {
        task_processor.run().await;
    });

    // 构建我们的路由器
    let app = Router::new()
        .route("/api/tasks", get(handlers::get_tasks))
        .route("/api/tasks", post(handlers::create_task))
        .route("/api/tasks/:task_id/stop", post(handlers::stop_task))
        .route("/api/tasks/:task_id/delete", post(handlers::delete_task))
        .route("/api/tasks/:task_id/logs", get(handlers::get_logs))
        .route("/api/branches", get(handlers::get_branches))
        .route("/api/branches/:branch_name/commits", get(handlers::get_commits))
        .route("/api/current_user", get(handlers::get_current_user))
        .layer(axum::Extension(db_pool))
        .layer(axum::Extension(tx))
    ;

    // 运行我们的服务
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}