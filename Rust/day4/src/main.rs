use md5;

fn general_solution(secret: &str, number_of_zeros: usize) -> usize {
    let mut result = 0;
    for i in 0..=10000000 {
        let message = format!("{}{}", secret, i);
        let digest = md5::compute(message);
        let digest = format!("{:x}", digest);
        if &digest[0..5] == "0".repeat(number_of_zeros) {
            result = i;
            break
        }
    }
    result
}

fn main() {
    let secret = "ckczppom";
    // let result1 = general_solution(&secret, 5);
    let result2 = general_solution(&secret, 6);
    // println!("result: {:?}", result1);
    println!("result: {:?}", result2);
}
