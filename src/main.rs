struct Room {
    name: String, // E.g. "Antechamber"
    desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    doors: Vec<Door>
}
struct Door {
    target: RoomID, // More about this in a minute
    triggers:Vec<String>, // e.g. "go north", "north"
    message: Option<String> // What message, if any, to print when the doorway is traversed
    // Any other info about the door would go here
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct RoomID(usize);

fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;
    let rooms = [
        Room {
            name: "Foyer".into(), // Turn a &'static string (string constant) into a String
            desc: "This beautifully decorated foyer beckons you further into the mansion.  There is a door to the north.".into(),
            doors: vec![Door{target:RoomID(1), triggers:vec!["door".into(), "north".into(), "go north".into()], message:None}]
        },
        Room {
            name: "Antechamber".into(),
            desc: "Dark wood paneling covers the walls.  An intricate painting of a field mouse hangs slightly askew on the wall (it looks like you could fix it).  The gilded northern doorway lies open to a shadowy parlour.  You can return to the foyer to the southern door.".into(),
            doors: vec![
                Door{target:RoomID(0), triggers:vec!["door".into(), "south".into(), "go south".into(), "foyer".into()], message:None},
                Door{target:RoomID(2), triggers:vec!["north".into(), "doorway".into(), "go north".into()], message:None},
                Door{target:RoomID(3), triggers:vec!["painting".into(), "mouse".into(), "fix painting".into()], message:Some("As you adjust the painting, a trap-door opens beneath your feet!".into())}
            ]
        },
        Room {
            name: "A Room Full of Snakes!".into(),
            desc: "The shadows wriggle and shift as you enter the parlour.  The floor is covered in snakes!  The walls are covered in snakes!  The ceiling is covered in snakes!  You are also covered in snakes!\n\nBAD END".into(),
            doors:vec![]
        },
        Room {
            name: "The Vault".into(),
            desc: "When you regain consciousness, you feel a stabbing sensation in your lower back.  Reaching beneath you, you discover a massive diamond!  This room is full of gold and jewels, and a convenient ladder leading back outdoors!\n\nYou win!".into(),
            doors:vec![]
        }
    ];
    let end_rooms = [RoomID(2), RoomID(3)];
    let mut input = String::new();

    let mut at = RoomID(0);
    println!("The Spooky Mansion Adventure");
    println!("============================");
    println!();
    println!("You've been walking for hours in the countryside, and have finally stumbled on the spooky mansion you read about in the tour guide.");
    loop {
        // We don't want to move out of rooms, so we take a reference
        let here = &rooms[at.0];
        println!("{}\n{}", here.name, here.desc);
        if end_rooms.contains(&at) {
            break;
        }
        loop {
            print!("What will you do?\n> ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if let Some(door) = here.doors.iter().find(|d| d.triggers.iter().any(|t| *t == input)) {
                if let Some(msg) = &door.message {
                    println!("{}", msg);
                }
                at = door.target;
                break;
            } else {
                println!("You can't do that!");
            }
        }
    }
}
