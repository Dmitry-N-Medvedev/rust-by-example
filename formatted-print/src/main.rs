fn main() {
    println!("{} days", 31);
    println!(
        "{subject} {verb} {object}",
        subject="the quick brown fox",
        verb="jumps over",
        object="the fucking dog"
    );
    println!("{} of {:b} ppl know binary, the other half does not", 1, 2);
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    let pi = 3.141592;
    println!("Pi is roughly {0:.3}", pi);
}
