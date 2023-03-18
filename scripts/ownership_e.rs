fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // s1 is moved in add();
    println!("s1 - {s2} - {s3} : {s}");

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let t = format!("{s4}-{s5}-{s6}");

    println!("{s4} - {s5} - {s6} : {t}");
}
