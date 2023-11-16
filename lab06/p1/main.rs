use std::fs::File;
use std::io::{self, BufRead};

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("pong!");
    }
}

struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("counted {} args", args.len());
    }
}

struct TimesCommand {
    count: usize,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        println!("This command has been called {} times", self.count);
    }
}

struct CustomCommand;

impl Command for CustomCommand {
    fn get_name(&self) -> &str {
        "custom"
    }

    fn exec(&mut self, args: &[&str]) {
        let argument = args.join(" ");
        let vowel_count = argument
            .chars()
            .filter(|c| "aeiouAEIOU".contains(*c))
            .count();

        println!("Custom command executed with arguments: {:?}", args);
        println!("Number of vowels in the arguments: {}", vowel_count);
    }
}

struct StopCommand;

impl Command for StopCommand {
    fn get_name(&self) -> &str {
        "stop"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("Stopping execution.");
        std::process::exit(0);
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self) {
        //if let Ok(file) =
        //    File::open("C:\\Users\\adria\\OneDrive\\Desktop\\rust\\lab06\\p1\\src\\in.txt")
        //{
            if let Ok(file) = File::open("in.txt") {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                if let Ok(command_line) = line {
                    let mut parts = command_line.split_whitespace().collect::<Vec<&str>>();
                    if !parts.is_empty() {
                        let command_name = parts.remove(0);
                        let args = parts.as_slice();

                        if let Some(command) = self
                            .commands
                            .iter_mut()
                            .find(|cmd| cmd.get_name() == command_name)
                        {
                            command.exec(args);
                        } else {
                            println!("Error: Unknown command '{}'", command_name);
                        }
                    }
                }
            }
        } else {
            println!("Error: Unable to open the file");
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CustomCommand {}));
    terminal.register(Box::new(StopCommand {}));

    terminal.run();
}
