// 问题：泛型函数怎样声明自己需要的通知能力？
// 输入与状态：Email 与 Sms 各自实现 Notifier；notify 借用其中一个发送器。
// 结果证据：同一函数分别得到邮件：下单成功和短信：下单成功；仅生成文本。
// 改一处：移除 N: Notifier 里的约束，观察函数体不能再依赖 send 方法（E0599）。
// 边界：有同名方法不等于实现 Trait；借用发送器也不等于转移它的所有权。
// 运行：cargo run --locked --example 03-trait-bound

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

fn notify<N: Notifier>(sender: &N, message: &str) -> String {
    sender.send(message)
}

pub fn main() {
    let email = Email;
    let sms = Sms;
    let email_message = notify(&email, "下单成功");
    let sms_message = notify(&sms, "下单成功");
    assert_eq!(email_message, "邮件：下单成功");
    assert_eq!(sms_message, "短信：下单成功");
    println!("{email_message}");
    println!("{sms_message}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_notification_preserves_empty_and_nonempty_messages() {
        assert_eq!(notify(&Email, ""), "邮件：");
        assert_eq!(notify(&Sms, ""), "短信：");
        assert_eq!(notify(&Email, "下单成功"), "邮件：下单成功");
        assert_eq!(notify(&Sms, "下单成功"), "短信：下单成功");
    }
}
