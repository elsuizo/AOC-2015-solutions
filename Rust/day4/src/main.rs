use md5;

fn main() {
    let digest = md5::compute(b"abcdef609043");
    println!("digest: {:?}", digest);
}
