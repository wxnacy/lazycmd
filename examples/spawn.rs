use lazycmd::spawn;

fn main() {
    let result = spawn(["ls", "-l"]).unwrap();
    println!("{result:#?}");
}
