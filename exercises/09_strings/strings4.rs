// Calls of this function should be replaced with calls of `string_slice` or `string`.
// 定义一个 trait，用于处理字符串
trait StringHandler {
    fn process(&self);
}

// 实现 trait 用于 &str 类型
impl StringHandler for &str {
    fn process(&self) {
        string_slice(self);
        // println!("Processing &str: {}", self);
    }
}

// 实现 trait 用于 String 类型
impl StringHandler for String {
    fn process(&self) {
        string(self.clone());
        //println!("Processing String: {}", self);
    }
}

// 泛型函数，用于接受实现了 StringHandler trait 的类型
fn placeholder<T: StringHandler>(arg: T) {
    arg.process(); // 根据 arg 实现的 process 方法进行处理
}

fn string_slice(arg: &str) {
    println!("&str {arg}");
}

fn string(arg: String) {
    println!("String {arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder::<&str>("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
