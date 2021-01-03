struct S;

fn main() {
    loop {
        let x;
        if true {
            x = S;
        }
        drop(x);
        //~^ ERROR use of possibly-uninitialized variable: `x` [E0381]
    }
}
