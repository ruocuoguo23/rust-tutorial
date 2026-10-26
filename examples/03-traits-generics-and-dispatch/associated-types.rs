// 问题：Trait 的关联类型如何固定实现的输出类型？
// 输入与状态：Counter 的 Item=u8；next 从 0 到 2 前产生 0、1，之后保持 None。
// 结果证据：main 与测试断言正常耗尽、空区间、逆区间及 u8 上界。
// 改一处：把 Source<Item = u8> 约束改为 Item = u16，观察 Counter 不满足约束。
// 边界：关联类型属于 Trait 实现；递增仅在 next < end 时发生以避免溢出。
// 运行：cargo run --locked --example 03-associated-types

trait Source {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    next: u8,
    end: u8,
}

impl Source for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.end {
            return None;
        }
        let value = self.next;
        self.next += 1; // next < end <= u8::MAX, so increment cannot overflow.
        Some(value)
    }
}

fn one<S: Source<Item = u8>>(source: &mut S) -> Option<u8> {
    source.next()
}

pub fn main() {
    let mut counter = Counter { next: 0, end: 2 };
    assert_eq!(one(&mut counter), Some(0));
    assert_eq!(one(&mut counter), Some(1));
    assert_eq!(one(&mut counter), None);
    println!("Source<Item=u8>: 0, 1, None");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_stays_exhausted_and_handles_the_u8_boundary() {
        let mut source = Counter {
            next: 254,
            end: 255,
        };
        assert_eq!(one(&mut source), Some(254));
        assert_eq!(one(&mut source), None);
        assert_eq!(one(&mut source), None);
        assert_eq!(one(&mut Counter { next: 0, end: 0 }), None);
        assert_eq!(one(&mut Counter { next: 2, end: 1 }), None);
    }
}
