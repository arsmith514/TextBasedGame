struct Room {
    name: String,       // E.g. "Antechamber"
    desc_light: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    desc_dark: String,
    doors: Vec<Door>,
}
struct Door {
    target: RoomID,        // More about this in a minute
    triggers: Vec<String>, // e.g. "go north", "north"
    message: Option<String>, // What message, if any, to print when the doorway is traversed
                           // Any other info about the door would go here
}


#[derive(PartialEq, Eq, Clone, Copy)]
struct RoomID(usize);

struct GameState {
    room_id: RoomID,
    key: bool,
    sunscreen: bool,
    map: bool,
    win: bool,
    timer: usize,
}

fn title_screen() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;
    println!();
    println!("██ ███╗   █████████╗██████╗   ██████████╗  ██ ███████╗   ███╗   ███ ██████████████╗█████╗██╗   █████████ ██████╗ ███████ ███████╗");
    println!("██ ████╗  ██╚═██╔══██╔═══██╗  ╚══██╔══██║  ██ ██╔════╝   ████╗ ████ ██╔════╚═██╔══██╔══████║   ████╔════ ██╔══██ ██╔════ ██╔════╝");
    println!("██ ██╔██╗ ██║ ██║  ██║   ██║     ██║  ███████ █████╗     ██╔████╔██ █████╗   ██║  █████████║   ███████╗  ██████╔ ███████ █████╗");
    println!("██ ██║╚██╗██║ ██║  ██║   ██║     ██║  ██╔══██ ██╔══╝     ██║╚██╔╝██ ██╔══╝   ██║  ██╔══██╚██╗ ██╔██╔══╝  ██╔══██ ╚════██ ██╔══╝");
    println!("██ ██║ ╚████║ ██║  ╚██████╔╝     ██║  ██║  ██ ███████╗   ██║ ╚═╝ ██ ███████╗ ██║  ██║  ██║╚████╔╝███████ ██║  ██ ███████ ███████╗");
    println!("██ ██║ ╚████║ ██║  ╚██████╔╝     ██║  ██║  ██ ███████╗   ██║ ╚═╝ ██ ███████╗ ██║  ██║  ██║╚████╔╝███████ ██║  ██ ███████ ███████╗");
    println!("╚═ ╚═╝  ╚═══╝ ╚═╝   ╚═════╝      ╚═╝  ╚═╝  ╚═ ╚══════╝   ╚═╝     ╚═ ╚══════╝ ╚═╝  ╚═╝  ╚═╝ ╚═══╝ ╚══════ ╚═╝  ╚═ ╚══════ ╚══════╝");
    println!();

    println!(
        "Instructions: navigate by typing commands in the terminal (for example: \"go north\")."
    );
    println!("Enter \"i\" to see inventory.");
    print!("\nPress enter/shift key to begin \n> ");
    io::stdout().flush().unwrap();
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).unwrap();
}

fn make_map(room: RoomID) {
    println!("                        ###### ______");
    if room == RoomID(1) {
        println!("                        # *W #       ||");
    } else {
        println!("                        # W  #       ||");
    }
    println!("                        ######      ###### ");

    if room == RoomID(2) {
        println!("                          ||        # *VR# ");
    } else {
        println!("                          ||        # VR # ");
    }
    println!("                        ######      ###### ");
    if room == RoomID(0) {
        println!("                        # *S # ______||");
    } else {
        println!("                        # S  # ______||");
    }
    println!("                        ######");
    println!("                          ||  ");
    println!("######______######______######");

    if room == RoomID(3) {
        println!("# E  #______# SS #______# *L #");
    } else if room == RoomID(4) {
        println!("# E  #______# *SS#______# L  #");
    } else if room == RoomID(5) {
        println!("# *E #______# SS #______# L  #");
    } else {
        println!("# E  #______# SS #______# L  #");
    }
    println!("######      ######      ######");
    if room == RoomID(6) {
        println!(" | |         |*|");
    } else {
        println!(" | |         | |");
    }

    println!("######      ######");
    if room == RoomID(7) {
        println!("# *B #");
    } else {
        println!("# B  #");
    }
    println!("######");
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
    // room 7 = battle -- need key

    let rooms = [
        Room {
            name: "Start Room".into(), // Turn a &'static string (string constant) into a String
            desc_light: "You see an entryway to the north, east, and south.".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. If only you could find a light...".into(),
            doors: vec![
                Door {
                    target: RoomID(1),
                    triggers: vec![
                        "north".into(),
                        "up".into(),
                        "go up".into(),
                        "go north".into(),
                        "whatsapp room".into(),
                        "whatsapp.()".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(2),
                    triggers: vec![
                        "east".into(),
                        "right".into(),
                        "go right".into(),
                        "VR".into(),
                        "go east".into(),
                        "VR room".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(3),
                    triggers: vec![
                        "south".into(),
                        "go south".into(),
                        "down".into(),
                        "go down".into(),
                        "library".into(),
                        "door".into(),
                    ],
                    message: None,
                },
            ],
        },
        Room {
            name: "Whatsapp Room".into(),
            desc_light: "its light".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. If only you could find a light...".into(),
            doors: vec![
                Door {
                    target: RoomID(0),
                    triggers: vec![
                        "start".into(),
                        "south".into(),
                        "go south".into(),
                        "down".into(),
                        "go down".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(2),
                    triggers: vec![
                        "east".into(),
                        "go east".into(),
                        "right".into(),
                        "go right".into(),
                        "VR room".into(),
                        "VR".into(),
                        "door".into(),
                    ],
                    message: None,
                },
            ],
        },
        Room {
            name: "VR Room".into(),
            desc_light: "its light".into(),
            desc_dark: "its dark".into(),
            doors: vec![
                Door {
                    target: RoomID(0),
                    triggers: vec![
                        "south".into(),
                        "go south".into(),
                        "down".into(),
                        "go down".into(),
                        "start".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(1),
                    triggers: vec![
                        "north".into(),
                        "go north".into(),
                        "up".into(),
                        "go up".into(),
                        "whatsapp room".into(),
                        "whatsapp".into(),
                        "door".into(),
                    ],
                    message: None,
                },
            ],
        },
        Room {
            name: "Library".into(),
            desc_light: "its light".into(),
            desc_dark: "its dark".into(),
            doors: vec![
                Door {
                    target: RoomID(0),
                    triggers: vec![
                        "north".into(),
                        "go north".into(),
                        "up".into(),
                        "go up".into(),
                        "start".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(4),
                    triggers: vec![
                        "west".into(),
                        "go west".into(),
                        "left".into(),
                        "go left".into(),
                        "sunscreen room".into(),
                        "sun screen".into(),
                        "sunscreen".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(6),
                    triggers: vec![
                        "go south".into(),
                        "south".into(),
                        "nothing".into(),
                        "door".into(),
                    ],
                    message: None,
                },
            ],
        },
        Room {
            name: "Sunscreen Room".into(),
            desc_light: "its light".into(),
            desc_dark: "its dark".into(), // should be same as light cuz has a light
            doors: vec![
                Door {
                    target: RoomID(3),
                    triggers: vec![
                        "east".into(),
                        "go east".into(),
                        "right".into(),
                        "go right".into(),
                        "library".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(5),
                    triggers: vec![
                        "west".into(),
                        "go west".into(),
                        "left".into(),
                        "go left".into(),
                        "end".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(6),
                    triggers: vec![
                        "south".into(),
                        "go south".into(),
                        "down".into(),
                        "go down".into(),
                        "nothing".into(),
                        "door".into(),
                    ],
                    message: None,
                },
            ],
        },
        Room {
            name: "End Room".into(),
            desc_light: "its light".into(),
            desc_dark: "its dark".into(),
            doors: vec![
                Door {
                    target: RoomID(4),
                    triggers: vec![
                        "go back".into(),
                        "right".into(),
                        "go right".into(),
                        "east".into(),
                        "go east".into(),
                        "sunscreen".into(),
                        "sun screen".into(),
                        "door".into(),
                    ],
                    message: None,
                },
                Door {
                    target: RoomID(7),
                    triggers: vec![
                        "end".into(),
                        "use key".into(),
                        "key".into(),
                        "south".into(),
                        "go south".into(),
                        "down".into(),
                        "go down".into(),
                        "door".into(),
                    ],
                    message: None,
                },
            ],
        },
        Room {
            name: "Nothing".into(),
            desc_light: "its light".into(),
            desc_dark: "its dark".into(),
            doors: vec![Door {
                target: RoomID(4),
                triggers: vec![
                    "turn around".into(),
                    "go up".into(),
                    "up".into(),
                    "north".into(),
                    "go north".into(),
                    "go back".into(),
                    "back".into(),
                    "library".into(),
                ],
                message: None,
            }],
        },
        Room {
            name: "Battle Room".into(),
            desc_light: "its light".into(),
            desc_dark: "its dark".into(),
            doors: vec![],
        },
    ];

    let end_rooms = [RoomID(7)];
    let mut input = String::new();

   
    let mut at: GameState = GameState {
        room_id: RoomID(0),
        key: false,
        sunscreen: false,
        map: false,
        win: false,
        timer: 50,
    };

    title_screen();
    loop {
        // We don't want to move out of rooms, so we take a reference
        let here = &rooms[at.room_id.0];
        if !at.sunscreen {
            println!("{}\n{}", here.name, here.desc_dark);
        } else {
            println!("{}\n{}", here.name, here.desc_light);
        }

        if at.map {
            make_map(at.room_id);
        }
        // get map logic
        if at.room_id == RoomID(2) && !at.map {
            loop {
                print!("you see thing with VR - do you try it on? (yes or no) \n>");
                io::stdout().flush().unwrap();
                let mut try_vr = String::new();
                io::stdin().read_line(&mut try_vr).unwrap();
                let try_vr = try_vr.trim();
                if try_vr == "yes" {
                    at.map = true;
                    println!("you see a map!");
                    make_map(at.room_id);
                    break;
                }
                if try_vr == "no" {
                    break;
                }
            }
        }

        if at.room_id == RoomID(4) && !at.sunscreen {
            loop {
                print!("you see a UV lamp - do you grab it?\n>");
                io::stdout().flush().unwrap();
                let mut try_ss = String::new();
                io::stdin().read_line(&mut try_ss).unwrap();
                let try_ss = try_ss.trim();
                if try_ss == "yes" {
                    at.sunscreen = true;
                    println!("The lamp reveals all the doorways, oddly enough lathered in suncreen. By now knowing where all the entryways are, you'll have a much easier time getting around.");
                    break;
                }
                if try_ss == "no" {
                    break;
                }
            }
        }

        if end_rooms.contains(&at.room_id) {
            at.win = true;
            break;
        }
        if at.timer == 0 {
            break;
        }
        loop {
            print!("What will you do?\n");
            print!("({} minutes remain)\n> ", at.timer);
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if let Some(door) = here
                .doors
                .iter()
                .find(|d| d.triggers.iter().any(|t| *t == input))
            {
                if let Some(msg) = &door.message {
                    println!("{}", msg);
                }
                at.room_id = door.target;
                at.timer -= 1;
                break;
            } else {
                at.timer -=1;
                println!("Nothing there, unforunately. Your breathing starts to get more labored.");
            }
        }
    }
    if at.win == true{
        println!("You see a blinding light...as you step forward you emerge onto the sunny streets of Menlo Park. A new, yet uncertain future awaits for you.");
        println!("THE END");
    }
    if at.win == false {
        println!("You collapse to the ground, out of breath. As your vision starts to fade, you see ZUCK crouching over you, smiling. The last words you hear are: 'unfortnately there's going to be another around of layoffs...'");
        println!("THE END");
    }
}
