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
    won: bool,
}

impl Player {
    fn new(nname: String) -> Player {
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
                won: false,
            }
        }
    }

    fn set_draw(&mut self, var: Draw) {
        self.curr_draw = var;
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
            "name: {}, draw: {}, hasWon: {}, id: {}",
            self.name, self.curr_draw, self.won, self.id
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
    let mut player1: Player = Player::new(String::from("p1"));
    let mut player2: Player = Player::new(String::from("p2"));

    player1.set_draw(Draw::RpsRock);
    player2.set_draw(Draw::RpsScissors);

    let result = process_winner(player1, player2);
    println!("result: {}", result);
}

fn process_winner(p1: Player, p2 :Player) -> u32 {

    //brute force way of checking who won
    if p1.curr_draw == p2.curr_draw{
        0 //no one won
    }else if p1.curr_draw == Draw::RpsRock && p2.curr_draw == Draw::RpsScissors {
        1
    } else if p1.curr_draw == Draw::RpsPaper && p2.curr_draw == Draw::RpsRock {
        1
    } else if p1.curr_draw == Draw::RpsScissors && p2.curr_draw == Draw::RpsPaper {
        1
    } else {
        2
    }
}
