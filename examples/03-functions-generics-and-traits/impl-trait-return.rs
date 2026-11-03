// 问题：返回位置的 impl Trait 隐藏什么，谁决定具体返回类型？
// 输入与状态：make_notifier 选择 Email；调用方只依赖 Notifier 的 send 能力。
// 结果证据：返回的发送器生成邮件：下单成功；不会实际访问网络。
// 改一处：另定义实现 Notifier 的 Sms，再尝试按 bool 分支返回 Email 或 Sms，观察 E0308。
// 边界：同一次泛型实例化的返回路径须统一具体类型；impl Trait 不是任意实现的联合。
// 运行：cargo run --locked --example 03-impl-trait-return

trait Notifier {
    fn send(&self, message: &str) -> String;
}

struct Email;

impl Notifier for Email {
    fn send(&self, message: &str) -> String {
        format!("邮件：{message}")
    }
}

fn make_notifier() -> impl Notifier {
    Email
}

pub fn main() {
    let sender = make_notifier();
    let message = sender.send("下单成功");
    assert_eq!(message, "邮件：下单成功");
    println!("impl Trait return: {message}");
}
