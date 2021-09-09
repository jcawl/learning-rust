use std::fs::File;

fn main() {
    //shortcut method that is implemented just like the match
    //if error will panic, if ok will return vale inside
    let f = File::open("hellot.txt").unwrap();
}
