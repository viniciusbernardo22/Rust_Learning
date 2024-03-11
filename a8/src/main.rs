fn count_down(num: i32) {
    let mut n = num;
    while n > 0 {
        println!("{}", n);
        n -= 1;
    } 
}

fn count(num:i32) {
    for i in 1..num + 1 {
        println!("{}", i);
    }
}
fn main() {
    count_down(10);
    println!("---------------------");
    count(10);
}
