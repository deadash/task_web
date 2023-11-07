// src/tasks.rs

use models::task::Task;
use sqlx::SqlitePool;
use tokio::sync::mpsc::UnboundedReceiver;
use std::collections::VecDeque;

// 定义表示不同任务事件的枚举
pub enum TaskEvent {
    Create(Task),
    Stop(i64),
    Delete(i64),
}

// 任务处理器结构体
pub struct TaskProcessor {
    db_pool: SqlitePool,
    queue: VecDeque<Task>, // 使用 VecDeque 实现任务队列
    receiver: UnboundedReceiver<TaskEvent>, // 用于接收任务事件的通道接收器
}

impl TaskProcessor {
    pub async fn new(db_pool: SqlitePool, receiver: UnboundedReceiver<TaskEvent>) -> Self {
        let mut processor = TaskProcessor {
            db_pool,
            queue: VecDeque::new(),
            receiver,
        };

        // 初始化时从数据库加载任务
        processor.load_tasks().await;

        processor
    }

    async fn load_tasks(&mut self) {
        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE status NOT IN ('Stopped', 'Cancelled') ORDER BY created_at ASC")
            .fetch_all(&self.db_pool)
            .await
            .expect("Failed to fetch tasks");

        for task in tasks {
            if task.status == "Running" {
                // 对于运行中的任务，执行特别的动作
                self.handle_running_task(&task).await;
            } else {
                // 未开始的任务放入队列
                self.queue.push_back(task);
            }
        }
    }

    async fn handle_running_task(&self, task: &Task) {
        // 实现特别的动作，比如可能需要检查任务状态或启动任务执行的逻辑
    }

    // 处理接收到的事件
    pub async fn run(&mut self) {
        while let Some(event) = self.receiver.recv().await {
            match event {
                TaskEvent::Create(task) => {
                    // 处理创建任务事件
                    self.queue.push_back(task);
                },
                TaskEvent::Stop(task_id) => {
                    // 处理停止任务事件
                    self.stop_task(task_id).await;
                },
                TaskEvent::Delete(task_id) => {
                    // 处理删除任务事件
                    self.delete_task(task_id).await;
                },
            }
        }
    }

    async fn stop_task(&self, task_id: i64) {
        // 实现停止任务的逻辑
    }

    async fn delete_task(&self, task_id: i64) {
        // 实现删除任务的逻辑
    }
}
