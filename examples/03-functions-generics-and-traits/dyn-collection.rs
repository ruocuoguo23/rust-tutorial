// 问题：一个 Vec 怎样拥有两种不同具体类型的通知器？
// 输入与状态：Email 与 Sms 装进 Box，集合元素统一为 Box<dyn Notifier>。
// 结果证据：按顺序调用两个元素，分别输出邮件：下单成功和短信：下单成功。
// 改一处：改用 vec![Email, Sms]，观察普通 Vec 不能把两种具体类型当成同一种元素类型。
// 边界：Box 负责拥有值，dyn 负责动态接口；本例是零大小类型，不据此断言分配次数。
// 运行：cargo run --locked --example 03-dyn-collection

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

pub fn main() {
    let senders: Vec<Box<dyn Notifier>> = vec![Box::new(Email), Box::new(Sms)];
    assert_eq!(senders[0].send("下单成功"), "邮件：下单成功");
    assert_eq!(senders[1].send("下单成功"), "短信：下单成功");
    for sender in senders {
        println!("Owned dyn: {}", sender.send("下单成功"));
    }
}
