use speed_limit::speed_limit;

#[speed_limit(a < 130)]
fn function(a: u32) {
    println!("{}", a);
}

fn main() {
    function(80);
    println!("after first call");
    function(140);
}