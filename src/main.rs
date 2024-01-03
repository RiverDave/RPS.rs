use rand::Rng;

#[derive(PartialEq)]
enum Draw {
    RpsPaper,
    RpsRock,
    RpsScissors,
}

// create a struct for a player that has a name and a draw
struct Player {
    name: String,
    curr_draw: Draw,
    id: u32,
    isbot: bool,
}

impl Player {
    fn new(nname: String, is_bot: bool) -> Player {
        // constructor

        static mut NEXT_ID: u32 = 0;

        // modifying a static variable requires for the code to be inside an unsafe block
        // there are surely other workarounds but this one was the first that came to mind
        unsafe {
            NEXT_ID += 1;

            Player {
                name: nname,
                curr_draw: Draw::RpsPaper,
                id: NEXT_ID,
                isbot: is_bot,
            }
        }
    }

    fn set_draw(&mut self, var: Draw) {
        self.curr_draw = var;
    }

    fn prompt_user(&mut self) {
        let result: i32;

        if self.isbot == true {
            println!("Bot is drawing..: ");
            //generate a random number between 1 and 3
            let mut rng = rand::thread_rng();
            result = rng.gen_range(1..4);
        } else {

            println!("---------------------------");
            println!("Hi! {}: Please choose an option: ", self.name);
            //prompt the user for input
            println!("---------------------------");
            let mut input: String = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            result = input.trim().parse().expect("Please type a number!");
        }
        // i need to store the result of the match in a variable in the struct

        match result {
            1 => self.set_draw(Draw::RpsRock),
            2 => self.set_draw(Draw::RpsPaper),
            3 => self.set_draw(Draw::RpsScissors),
            4 => std::process::exit(0),
            _ => println!("Invalid input!"),
        }
    }

    // fn get_name(self) -> String {
    //     self.name
    // }
    //
    // fn change_name(&mut self, var: String) {
    //     self.name = var;
    // }
    //
    // fn change_draw(&mut self, var: Draw) {
    //     self.curr_draw = var;
    // }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, draw: {}, IsBot: {}, id: {}",
            self.name, self.curr_draw, self.isbot, self.id
        )
    }
}

impl std::fmt::Display for Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // std::formatter contains a buffer where we write to.
            Draw::RpsScissors => write!(f, "Scissors"),
            Draw::RpsRock => write!(f, "Rock"),
            Draw::RpsPaper => write!(f, "Paper"),
        }
    }
}

//In this case, implementations is a way to overload formatting. we're implementing a trait for a
//type(Player & draw) in this case.
//Im doing it this way since you cant output the contents of a struct manually
//without enabling a debug flag, so we use an implementation instead.

//implementing comparision between type(player) and enum(Draw):
impl PartialEq<Draw> for Player {
    fn eq(&self, other: &Draw) -> bool {
        self.curr_draw == *other
    }
}

fn main() {
    //construct a player with a name and a draw
    let mut player1: Player = Player::new(String::from("p1"), false);
    let mut player2: Player = Player::new(String::from("p2"), true);

    display_menu();
    player1.prompt_user();
    player2.prompt_user();

    // println!("Player 1  {}", player1);
    // println!("Player 2  {}", player1);

    let mut end = false;

    while end == false {
        let result = process_winner(&player1, &player2);
        if result == 1 {
            end = true;
        } else if result == 2 {
            end = true;
        } else {
            println!("There's no winner! Try again!");
            player1.prompt_user();
            player2.prompt_user();
        }
    }

    println!("---------------------------");
    println!("Player 1 chose {}", player1.curr_draw);
    println!("---------------------------");
    println!("Player 2 chose {}", player2.curr_draw);
    println!("---------------------------");

    process_winner(&player1, &player2);
}

fn process_winner(p1: &Player, p2: &Player) -> u32 {
    //brute force way of checking who won
    if p1.curr_draw == p2.curr_draw {
        0 //no one won
    } else if p1.curr_draw == Draw::RpsRock && p2.curr_draw == Draw::RpsScissors {
        println!("{} wins!", p1.name);
        1
    } else if p1.curr_draw == Draw::RpsPaper && p2.curr_draw == Draw::RpsRock {
        println!("{} wins!", p1.name);
        1
    } else if p1.curr_draw == Draw::RpsScissors && p2.curr_draw == Draw::RpsPaper {
        println!("{} wins!", p1.name);
        1
    } else {
        println!("{} wins!", p2.name);
        2
    }
}

fn display_menu() {
    println!("---------------------------");
    println!("Welcome to Rock, Paper, Scissors!");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissors");
    println!("4. Quit");
    println!("---------------------------")
}

// fn draw(var: u32) -> draw {}
