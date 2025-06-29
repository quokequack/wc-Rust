mod format_output;

fn main() {

    let output = format_output::format_output(
        Some(7145),
        Some(58164),
        Some(342190),
        Some("test.txt")
    );

    println!("{}", output);
}
