use std::io;
use std::process::exit;
use std::sync::LazyLock;

static ENTRANCE: Room = Room {
    name: LazyLock::new(|| String::from("entrance")),
    description: LazyLock::new(|| {
        String::from(
            "You find yourself inside a small circular room made of darkness and stone slab walls.\nAbove; the square of light from which you fell.\nAnd below; more stone tiles - mossy and dripping with the angst of decrepit abandonment.\nThere is an open archway to the South.
        ",
        )
    }),

    investigation_text: LazyLock::new(|| String::from("You find nothing of note")),
};

static ROOM2: Room = Room {
    name: LazyLock::new(|| String::from("room2")),
    description: LazyLock::new(|| {
        String::from(
            "You find yourself inside a square stone-slabbed room.\nThere is a smattering of destritus on the floor; old boots, ripped clothing, and dry bones.\nThere is an exit to the South.",
        )
    }),

    investigation_text: LazyLock::new(|| String::from("You find nothing of note")),
};

static LARGEROOM: Room = Room {
    name: LazyLock::new(|| String::from("largeroom")),
    description: LazyLock::new(|| {
        String::from(
            "You find yourself inside a long rectangular room with stone-slabbed walls, mossy this time.\nThere is a long table in the centre of the room, reaching almost to each end.\nIt has many empty plates and tipped wooden cups.

        How did they get all of this down here??
        
        There are exits to the northwest and northeast.",
        )
    }),

    investigation_text: LazyLock::new(|| String::from("You find nothing of note")),
};

static ROOM4: Room = Room {
    name: LazyLock::new(|| String::from("room4")),
    description: LazyLock::new(|| {
        String::from(
            "You find yourself inside a square room with stone-slabbed walls.\nThere is a message on the wall, written in oxidised brown blood:

        'YOU ARE ALREADY DEAD'\nThere are exits to the North, South, and East.",
        )
    }),

    investigation_text: LazyLock::new(|| String::from("You find nothing of note")),
};

static SIDEROOM: Room = Room {
    name: LazyLock::new(|| String::from("sideroom")),
    description: LazyLock::new(|| {
        String::from(
            "You find yourself inside a square room with stone-slabbed walls.\nThis is the most barren of all the rooms.\nThere is naught but bones.\nThere is an exit to the West",
        )
    }),

    investigation_text: LazyLock::new(|| {
        String::from(
            "
            One brick seems outlined with cleanliness.
            You take a closer look and find that it is loose.
            With a firm grip you dislodge the brick and find a key.
            You place this in your pocket.
            ",
        )
    }),
};

static EXIT: Room = Room {
    name: LazyLock::new(|| String::from("exit")),
    description: LazyLock::new(|| {
        String::from(
            "You find yourself inside a circular room with stone-slabbed walls.\nThere are skeletons lain against the walls, with a large pile of human bones and clothing piled against the west wall.\nYou feel a slight chilling breeze coming from the north wall.\nThere is an exit to the South",
        )
    }),

    investigation_text: LazyLock::new(|| {
        String::from("
            Using the moisture clung to your skin, following the light wind, you find that one particular crack is hiding a secret.
            There is no peep of light, but perhaps something thin could be squeezed in?
            ",
            )
    }),
};

struct Room {
    name: LazyLock<String>,
    description: LazyLock<String>, // general description of the room
    investigation_text: LazyLock<String>,
}

struct Player<'a> {
    room: &'a Room,
    key_in_inventory: bool,
}

impl Player<'_> {
    fn new() -> Self {
        Self {
            room: &ENTRANCE,
            key_in_inventory: false,
        }
    }

    fn change_room(&mut self, desired_direction: &str) {
        let room_name = self.room.name.as_str();
        match (room_name, desired_direction) {
            // All valid room transitions listed
            ("room2", "north") => self.room = &ENTRANCE,
            ("room2" | "room4", "south") => self.room = &LARGEROOM,
            ("entrance", "south") | ("largeroom", "northwest") => self.room = &ROOM2,
            ("largeroom", "northeast") | ("sideroom", "west") => self.room = &ROOM4,
            ("room4", "north") => self.room = &EXIT,
            ("room4", "east") => self.room = &SIDEROOM,
            (_, _) => {
                println!("That is not a valid room");
                return; // Pattern for if trying to go to a room that is not connected to the current room
            }
        }
        println!("{}", self.room.description.clone());
    }

    fn use_item(&self) {
        if self.key_in_inventory && self.room.name.clone() == "exit" {
            println!(
                "
            You put the key into the crack with an expectant grunt.
            Nothing happens...
            Until it does and the wall opens up on a swivel!
            "
            );
            exit(0);
        }
    }

    fn investigate(&mut self) {
        println!("{:?}", self.room.investigation_text.clone());
        if self.room.name.clone() == "sideroom" {
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
            "help" => Player::<'_>::help(),
            _ => println!("Invalid "),
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
    println!("{}", player.room.description.clone());

    loop {
        let mut desired_action: String = String::new();

        io::stdin()
            .read_line(&mut desired_action)
            .expect("Failed to read line");

        let desired_action: String = match desired_action.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let attempted_action: &str = first_word(&desired_action);
        let attempted_target: &str = second_word(&desired_action);

        player.execute_action(attempted_action, attempted_target);
    }
}
