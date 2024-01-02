

enum Draw{
    RpsPaper,
    RpsRock,
    RpsScissors
}

fn main() {

    let current_draw = Draw::RpsScissors;
    let bot_draw = Draw::RpsRock;

    match current_draw{

        Draw::RpsPaper => {
            println!("1");
        }

        Draw::RpsRock => {
            println!("2");
        }

        Draw::RpsScissors => {
            println!("3");
        }
    }
}

fn process_draw(d1 : Draw, d2: Draw) -> Draw{

    
    Draw::RpsPaper

}
