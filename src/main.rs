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

fn title_screen() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;
    println!("");
    println!("██ ███╗   ██████████╗██████╗     ██████████╗  █████████╗    ███╗   ██████████████████╗█████╗██╗   ███████████████╗██████████████╗");
    println!("██ ████╗  ██╚══██╔══██╔═══██╗    ╚══██╔══██║  ████╔════╝    ████╗ ██████╔════╚══██╔══██╔══████║   ████╔════██╔══████╔════██╔════╝");
    println!("██ ██╔██╗ ██║  ██║  ██║   ██║       ██║  ████████████╗      ██╔████╔███████╗    ██║  █████████║   ███████╗ ██████╔████████████╗");
    println!("██ ██║╚██╗██║  ██║  ██║   ██║       ██║  ██╔══████╔══╝      ██║╚██╔╝████╔══╝    ██║  ██╔══██╚██╗ ██╔██╔══╝ ██╔══██╚════████╔══╝");
    println!("██ ██║ ╚████║  ██║  ╚██████╔╝       ██║  ██║  █████████╗    ██║ ╚═╝ █████████╗  ██║  ██║  ██║╚████╔╝█████████║  ████████████████╗");
    println!("██ ██║ ╚████║  ██║  ╚██████╔╝       ██║  ██║  █████████╗    ██║ ╚═╝ █████████╗  ██║  ██║  ██║╚████╔╝█████████║  ████████████████╗");
    println!("╚═ ╚═╝  ╚═══╝  ╚═╝   ╚═════╝        ╚═╝  ╚═╝  ╚═╚══════╝    ╚═╝     ╚═╚══════╝  ╚═╝  ╚═╝  ╚═╝ ╚═══╝ ╚══════╚═╝  ╚═╚══════╚══════╝");                                                                                                                       
    println!("");
    println!("Instructions: navigate by typing commands in the terminal (for example: \"go north\").");
    println!("Enter \"i\" to see inventory.");
    print!("\n Press a key to begin \n> ");
    io::stdout().flush().unwrap();
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).unwrap();
}

fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;

    // room 0 = Start??
    // room 1 = Whatsapp
    // room 2 = VR
    // room 3 = Library
    // room 4 = Sunscreen room
    // room 5 = end
    // room 6 = nothing 
    // room 7 = end end -- need key

    let rooms = [
        Room {
            name: "Start Room".into(), // Turn a &'static string (string constant) into a String
            desc: "".into(),
            doors: vec![
                Door{target:RoomID(1), triggers:vec!["north".into(), "go north".into(), "whatsapp room".into(), "whatsapp.()".into(), "door".into()], message:None},
                Door{target:RoomID(2), triggers:vec!["east".into(), "VR".into(), "go east".into(), "VR room".into(), "door".into()], message:None},
                Door{target:RoomID(3), triggers:vec!["south".into(), "go south".into(), "library".into(), "door".into()], message:None},
                ]
        },
        Room {
            name: "Whatsapp Room".into(),
            desc: "Dark wood paneling covers the walls.  An intricate painting of a field mouse hangs slightly askew on the wall (it looks like you could fix it).  The gilded northern doorway lies open to a shadowy parlour.  You can return to the foyer to the southern door.".into(),
            doors: vec![
                Door{target:RoomID(0), triggers:vec!["start".into(), "south".into(), "go south".into(), "door".into()], message:None},
                Door{target:RoomID(2), triggers:vec!["east".into(), "go east".into(), "VR room".into(), "VR".into(), "door".into()], message:None},
            ]
        },
        Room {
            name: "VR Room".into(),
            desc: "".into(),
            doors:vec![
                Door{target:RoomID(0), triggers:vec!["southwest".into(), "go southwest".into(), "start".into(), "door".into()], message:None},
                Door{target:RoomID(1), triggers:vec!["northwest".into(), "go northwest".into(), "whatsapp room".into(), "whatsapp".into(), "door".into()], message:None},
            ]
        },
        Room {
            name: "Library".into(),
            desc: "".into(),
            doors:vec![
                Door{target:RoomID(0), triggers:vec!["north".into(), "go north".into(), "start".into(), "door".into()], message:None},
                Door{target:RoomID(4), triggers:vec!["west".into(), "go west".into(), "sunscreen room".into(), "sun screen".into(), "sunscreen".into(), "door".into()], message:None},
                Door{target:RoomID(6), triggers:vec!["go south".into(), "south".into(), "nothing".into(), "door".into()], message:None},
            ]
        },
        Room {
            name: "Sunscreen Room".into(),
            desc: "".into(),
            doors:vec![
                Door{target:RoomID(3), triggers:vec!["east".into(), "go east".into(), "library".into(), "door".into()], message:None},
                Door{target:RoomID(5), triggers:vec!["west".into(), "go west".into(), "end".into(), "door".into()], message:None},
            ]
        },
        Room {
            name: "End Room".into(),
            desc: "".into(),
            doors:vec![
                Door{target:RoomID(4), triggers:vec!["go back".into(), "sunscreen".into(), "sun screen".into(), "door".into()], message:None},
                Door{target:RoomID(7), triggers:vec!["end".into(), "use key".into(), "key".into(), "door".into()], message:None},
            ]
        },
        Room {
            name: "Nothing".into(),
            desc: "You walk into a wall...".into(),
            doors:vec![
                Door{target:RoomID(3), triggers:vec!["turn around".into(), "go back".into(), "back".into(), "library".into()], message:None},
            ]
        },
        Room {
            name: "End End Room".into(),
            desc: "".into(),
            doors:vec![]
        },
    ];

    let end_rooms = [RoomID(7)];
    let mut input = String::new();

    let mut at = RoomID(0);
   
    title_screen();
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
