// 问题：怎样经同一个动态接口调用不同实现，同时继续由调用方拥有对象？
// 输入与状态：notify_dynamic 分别共享借用 Email 与 Sms，不创建 Box。
// 结果证据：同一个 &dyn Notifier 参数分别生成邮件与短信文本。
// 改一处：把参数改为 &Email，观察 Sms 不再能通过这个入口调用。
// 边界：&dyn Trait 本身不要求为被引用对象新建堆分配，也不接管对象所有权。
// 运行：cargo run --locked --example 03-dyn-borrowed

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

fn notify_dynamic(sender: &dyn Notifier, message: &str) -> String {
    sender.send(message)
}

pub fn main() {
    let email = Email;
    let sms = Sms;
    let email_message = notify_dynamic(&email, "下单成功");
    let sms_message = notify_dynamic(&sms, "下单成功");
    assert_eq!(email_message, "邮件：下单成功");
    assert_eq!(sms_message, "短信：下单成功");
    println!("Borrowed dyn: {email_message}");
    println!("Borrowed dyn: {sms_message}");
}
