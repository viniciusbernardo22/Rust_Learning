
fn maior_numero(nums: &[i32]) -> i32 {

    let mut maior: i32 = nums[0];

    for i in nums {
        if i > &maior {
            maior = *i;
        }
    }

    return maior;
}

fn main() {
    let nums = [3, 7, 20, 9, 10];
    maior_numero(&nums);
    println!("{}", maior_numero(&nums));
}
