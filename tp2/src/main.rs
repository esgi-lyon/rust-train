
fn main() {
    let c = 10;
    let mut b = &c;
    {
        let a = 10;
        if a == c {
            // &a produce doesn't leave long enough
            // error with b = &a; ref destroyed in block
            // Can't borrow a reference used previously (immutable references)
            // Need to create a variable in global scope
            // impossible: b = &mut a.clone();
            // rustc --explain E0716
        }
    }

    println!("{}", b);

    fn foo(a: u32) -> u32 {
        a * 2
    }

    let b = 10;
    let j = b;
    let c = foo(b);
    let d = foo(j);
    println!("{} {}", c, d);

}
