// 问题：显式 clone 一个 String 后，两份文本能否独立修改？
// 输入与状态：从 original=Ada 克隆 cloned，再只给 cloned 追加感叹号。
// 结果证据：断言和输出分别得到 Ada 与 Ada!，原 String 仍可用。
// 改一处：把 original.clone() 改为 original，观察后续使用原位置的 E0382。
// 边界：String 的 clone 复制文本；Clone 协议不保证所有类型都深拷贝资源。
// 运行：cargo run --locked --example 02-clone-value

pub fn main() {
    let original = String::from("Ada");
    let mut cloned = original.clone();
    cloned.push('!');
    assert_eq!(original, "Ada");
    assert_eq!(cloned, "Ada!");
    println!("Clone: original={original}, cloned={cloned}");
}
