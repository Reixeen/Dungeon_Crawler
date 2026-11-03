use std::collections::HashMap;
use std::io;
use std::process::exit;

use crate::AllRooms::*;
use crate::MovementDirection::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MovementDirection {
    // all possible directions that rooms can have doors in
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast, // unused with this dungeon layout
    Southwest, // unused with this dungeon layout
}

#[derive(Clone, PartialEq, Eq)]
pub enum AllRooms {
    // all rooms in the current dungeon layout
    Entrance,
    Room2,
    Largeroom,
    Room4,
    Sideroom,
    Exit,
}

impl AllRooms {
    // return a hashmap of contains sets of (door direction, room that is through that door)
    fn find_valid_exits(&self) -> HashMap<MovementDirection, AllRooms> {
        match self {
            Entrance => HashMap::from([(South, Room2)]),
            Room2 => HashMap::from([(North, Entrance), (South, Largeroom)]),
            Largeroom => HashMap::from([(Northwest, Room2), (Northeast, Room4)]),
            Room4 => HashMap::from([(South, Largeroom), (East, Sideroom), (North, Exit)]),
            Sideroom => HashMap::from([(West, Room4)]),
            Exit => HashMap::from([(South, Room4)]),
        }
    }

    fn print_description(&self) {
        //
        let mut description: String = String::new();

        match self {
            Entrance  => description.push_str("You find yourself inside a small circular room made of darkness and stone slab walls.\nAbove; the square of light from which you fell.\nAnd below; more stone tiles - mossy and dripping with the angst of decrepit abandonment.\nThere is an open archway to the South."),
            Room2     => description.push_str("You find yourself inside a square stone-slabbed room.\nThere is a smattering of destritus on the floor; old boots, ripped clothing, and dry bones.\nThere is an exit to the South."),
            Largeroom => description.push_str("You find yourself inside a long rectangular room with stone-slabbed walls, mossy this time.\nThere is a long table in the centre of the room, reaching almost to each end.\nIt has many empty plates and tipped wooden cups.\nHow did they get all of this down here??\nThere are exits to the northwest and northeast."),
            Room4     => description.push_str("You find yourself inside a square room with stone-slabbed walls.\nThere is a message on the wall, written in oxidised brown blood:\n'YOU ARE ALREADY DEAD'\nThere are exits to the North, South, and East."),
            Sideroom  => description.push_str("You find yourself inside a square room with stone-slabbed walls.\nThis is the most barren of all the rooms.\nThere is naught but bones.\nThere is an exit to the West",),
            Exit      => description.push_str("You find yourself inside a circular room with stone-slabbed walls.\nThere are skeletons lain against the walls, with a large pile of human bones and clothing piled against the west wall.\nYou feel a slight chilling breeze coming from the north wall.\nThere is an exit to the South"),
        }
        println!("{description}");
    }

    fn print_investigation(&self) {
        let mut description: String = String::new();

        match self {
            Sideroom  => description.push_str("One brick seems outlined with cleanliness.\nYou take a closer look and find that it is loose.\nWith a firm grip you dislodge the brick and find a key.\nYou place this in your pocket.\n"),
            Exit      => description.push_str("Using the moisture clung to your skin, following the light wind, you find that one particular crack is hiding a secret.\nThere is no peep of light, but perhaps something thin could be squeezed in?"),
            _         => description.push_str("You find nothing of note"),
        }
        println!("{description}");
    }
}

#[derive(Clone, PartialEq, Eq)]

pub struct Player {
    room: AllRooms,
    key_in_inventory: bool,
}

impl Player {
    fn new() -> Self {
        Self {
            room: Entrance,
            key_in_inventory: false,
        }
    }

    fn change_room(&mut self, desired_direction: &str) {
        let desired_direction = find_intended_direction(desired_direction);
        for (direction, new_room) in self.room.find_valid_exits() {
            if direction == desired_direction {
                self.room = new_room;
                self.room.print_description();
            }
        }
    }

    fn use_item(&self) {
        if self.key_in_inventory && self.room == Exit {
            println!(
                "
            You put the key into the crack with an expectant grunt.
            Nothing happens...
            Until it does and the wall opens up on a swivel!

            CONGRATULATIONS
            "
            );
            exit(0)
        } else {
            println!("You have nothing to use here");
        }
    }

    fn investigate(&mut self) {
        self.room.print_investigation();
        if self.room == Sideroom {
            self.key_in_inventory = true;
        }
    }

    fn inventory(&self) {
        if self.key_in_inventory {
            println!("You have a key");
        } else {
            println!("You have nothing on your person");
        }
    }

    fn help() {
        println!(
            "
        investigate, // Investigate the room for useful information + clues
        go,          // Go to a different room
        use,         // Open a door or container
        inventory,   // View contents of inventory
        help,        // View this menu"
        );
    }

    fn execute_action(&mut self, attempted_action: &str, attempted_target: &str) {
        match attempted_action {
            "go" => self.change_room(attempted_target),
            "investigate" => self.investigate(),
            "use" => self.use_item(),
            "inventory" => self.inventory(),
            "help" => Self::help(),
            _ => println!("Invalid action. Please try again."),
        }
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    s
}

fn find_intended_direction(desired_direction: &str) -> MovementDirection {
    match desired_direction {
        "north" => North,
        "south" => South,
        "east" => East,
        "west" => West,
        "northeast" => Northeast,
        "northwest" => Northwest,
        "southeast" => Southeast,
        "southwest" => Southwest,
        _ => {
            println!("That is not a valid direction");
            North
        }
    }
}

fn main() {
    let mut player = Player::new();
    player.room.print_description();

    loop {
        let mut desired_action: String = String::new();

        io::stdin()
            .read_line(&mut desired_action)
            .expect("Failed to read line");

        let desired_action: String = match desired_action.to_lowercase().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let attempted_action: &str = first_word(&desired_action);
        let attempted_target: &str = second_word(&desired_action);

        player.execute_action(attempted_action, attempted_target);
    }
}
