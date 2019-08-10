type UserName = String;

#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(UserName), // タプル構造体
    Working {
        assignee: UserName,
        remaining_hours: u16,
    }, // 名前付きフィールド構造体
    Done,
} // 4 つのバリアントを定義

use crate::Task::*; // バリアント名を直接書けるようにしている

fn main() {
    // Task 型の値を 3 つ作ってベクタに格納している
    let tasks = vec![
        AssignedTo(String::from("daichi")),
        Working {
            assignee: String::from("tsumugi"),
            remaining_hours: 1,
        },
        Done,
        Open,
    ];

    // 個々のタスクの状況をレポートする
    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => {
                println!("タスク{}は{}さんにアサインされています", i, assignee);
            }
            Working {
                assignee,
                remaining_hours,
            } => println!(
                "タスク{}は{}さんが作業中です。残り{}時間の見込み",
                i, assignee, remaining_hours
            ),
            _ => println!("タスク{}はその他のステータス({:?})です", i, task),
        }
    }
    // タスク0はdaichiさんにアサインされています
    // タスク1はtsumugiさんが作業中です。残り1時間の見込み
    // タスク2はその他のステータス(Done)です
    // タスク3はその他のステータス(Open)です
}
