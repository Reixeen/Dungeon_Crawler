use std::io;
use std::process::exit;

#[derive(Clone)]
pub struct Room {
    name: &'static str, // static 
    description: &'static str, // general description of the room
    investigation_text: &'static str,
}

static ENTRANCE: Room = Room {
    name: "Entrance",
    description: "You find yourself inside a small circular room made of darkness and stone slab walls.\nAbove; the square of light from which you fell.\nAnd below; more stone tiles - mossy and dripping with the angst of decrepit abandonment.\nThere is an open archway to the South.",
    investigation_text: "You find nothing of note"
    };

static ROOM2:Room = Room {
    name: "Room2",
    description: "You find yourself inside a square stone-slabbed room.\nThere is a smattering of destritus on the floor; old boots, ripped clothing, and dry bones.\nThere is an exit to the South.",
    investigation_text: "You find nothing of note",
    };

static LARGEROOM: Room = Room {
    name: "Largeroom",
    description: "You find yourself inside a long rectangular room with stone-slabbed walls, mossy this time.\nThere is a long table in the centre of the room, reaching almost to each end.\nIt has many empty plates and tipped wooden cups.\nHow did they get all of this down here??\nThere are exits to the northwest and northeast.",
    investigation_text: "You find nothing of note",
    };
    
static ROOM4: Room = Room {
    name: "Room4",
    description: "You find yourself inside a square room with stone-slabbed walls.\nThere is a message on the wall, written in oxidised brown blood:\n'YOU ARE ALREADY DEAD'\nThere are exits to the North, South, and East.",
    investigation_text: "You find nothing of note",
    };

static SIDEROOM: Room = Room {
    name: "Sideroom",
    description: "You find yourself inside a square room with stone-slabbed walls.\nThis is the most barren of all the rooms.\nThere is naught but bones.\nThere is an exit to the West",
    investigation_text: "One brick seems outlined with cleanliness.\nYou take a closer look and find that it is loose.\nWith a firm grip you dislodge the brick and find a key.\nYou place this in your pocket.\n",
    };

static EXIT: Room = Room {
    name: "Exit",
    description: "You find yourself inside a circular room with stone-slabbed walls.\nThere are skeletons lain against the walls, with a large pile of human bones and clothing piled against the west wall.\nYou feel a slight chilling breeze coming from the north wall.\nThere is an exit to the South",
    investigation_text: "Using the moisture clung to your skin, following the light wind, you find that one particular crack is hiding a secret.\nThere is no peep of light, but perhaps something thin could be squeezed in?",
    };
    
struct Player {
    room: Room,
    key_in_inventory: bool,
}        
impl Player {
    fn new() -> Self {
        Self {
            room: ENTRANCE.clone(),
            key_in_inventory: false,
        }
    }

    fn change_room(&mut self, desired_direction: &str) {
        let room_name = self.room.name;
        match (room_name, desired_direction) {
            // All valid room transitions listed
            ("Room2", "north")                                                     => self.room = ENTRANCE.clone(),
            ("Room2" | "Room4", "south")                                           => self.room = LARGEROOM.clone(),
            ("Entrance", "south") | ("Largeroom", "northwest")                     => self.room = ROOM2.clone(),
            ("Largeroom", "northeast") | ("Sideroom", "west")  | ("Exit", "south") => self.room = ROOM4.clone(),
            ("Room4", "north")                                                     => self.room = EXIT.clone(),
            ("Room4", "east")                                                      => self.room = SIDEROOM.clone(),
            (_, _) => {
                println!("That is not a valid room");
                return; // Pattern for if trying to go to a room that is not connected to the current room
            }
        }
        println!("{}\n", self.room.description);
    }

    fn use_item(&self) {
        if self.key_in_inventory && self.room.name == "Exit" {
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
            println!("You have nothing to use here")
        };
    }
    

    fn investigate(&mut self) {
        println!("{}",self.room.investigation_text);
        if self.room.name == "Sideroom" {
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
            "help" => Player::help(),
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

fn main() {
    let mut player = Player::new();
    println!("{}\n", player.room.description);

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