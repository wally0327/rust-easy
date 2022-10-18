fn main() {
    let s1 = "hello s1";
    let s2 = s1;

    println!("{}", s1);

    let s3 = String::from("123");
    let s4 = s3;

    // println!("{}", s3); 不可用
}
