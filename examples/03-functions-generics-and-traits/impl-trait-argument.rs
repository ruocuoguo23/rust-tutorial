// 问题：参数位置的 impl Trait 接收固定类型，还是满足能力约束的类型？
// 输入与状态：同一个 notify 分别借用 Email 与 Sms，调用方选择具体类型。
// 结果证据：输出两种通知文本；此处的 impl Trait 是匿名泛型参数。
// 改一处：把 &impl Notifier 改为 &Email，观察原 Sms 调用因类型不符而失败。
// 边界：这不是自动动态分发；需要表达多个参数属于同一类型时可使用命名泛型。
// 运行：cargo run --locked --example 03-impl-trait-argument

trait Notifier {
    fn send(&self, message: &str) -> String;
}

struct Email;
struct Sms;

impl Notifier for Email {
    fn send(&self, message: &str) -> String {
        format!("邮件：{message}")
    }
}

impl Notifier for Sms {
    fn send(&self, message: &str) -> String {
        format!("短信：{message}")
    }
}

fn notify(sender: &impl Notifier, message: &str) -> String {
    sender.send(message)
}

pub fn main() {
    let email_message = notify(&Email, "下单成功");
    let sms_message = notify(&Sms, "下单成功");
    assert_eq!(email_message, "邮件：下单成功");
    assert_eq!(sms_message, "短信：下单成功");
    println!("impl Trait argument: {email_message}");
    println!("impl Trait argument: {sms_message}");
}
