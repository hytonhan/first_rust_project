mod day01;
mod day02;

fn main() {
    // println!("Hello, world!");

    use std::io;

    let mut exit: bool = false;
    while exit == false
    {
        let mut input = String::new();
        println!("Enter day number: ");
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input = input.trim().to_string();
        println!("Input: {}", input);

        let start_time: std::time::Instant = std::time::Instant::now();
        match input.as_str() {
            "1" => day01::day01(),
            "2" => day02::day02(),
            "-1" => exit = true,
            _ => println!("wtf")
        }
        println!("Exec time: {:.2} ms", start_time.elapsed().as_millis());
    }
}
