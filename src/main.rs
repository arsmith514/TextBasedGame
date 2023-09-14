use std::io;
use std::io::Write;

use crate::rooms::get_rooms;
use crate::displays::print_zuck;
use crate::displays::title_screen;
use crate::displays::make_map;
use crate::displays::display_inventory;
mod rooms;
mod displays;

pub struct Room {
    name: String,       // E.g. "Antechamber"
    desc_light: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    desc_dark: String,
    doors: Vec<Door>,
}

// required items
// 1 = key
// 2 = spray
// 3 = data_reg
// 4 = social_net
// 47 = none
pub struct Door {
    target: RoomID,        // More about this in a minute
    triggers: Vec<String>, // e.g. "go north", "north"
    message: Option<String>, // What message, if any, to print when the doorway is traversed
    required_item: usize,
                           // Any other info about the door would go here
}

#[derive(PartialEq, Eq, Clone)]
pub struct Item {
    name: String,
    light_desc: String,
    dark_desc: String,
    pick_up_text: String,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct RoomID(usize);


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct GameState {
    room_id: RoomID,
    key: bool,       //library
    sunscreen: bool, // SS
    map: bool,       // VR
    win: bool,       // win condition
    timer: usize,    // timer: game over if it gets to 0
    boss_hp: usize,  // how many hits boss can take
    spray: bool,      // anti lizard spray -- end room
    data_reg: bool,   // data-privacy regulations -- Whatsapp room
    social_net: bool, // copy of the Social Network -- start
    searching: bool,
}

    // room 0 = Start??
    // room 1 = Whatsapp
    // room 2 = VR
    // room 3 = Library
    // room 4 = Sunscreen room
    // room 5 = end
    // room 6 = nothing
    // room 7 = battle -- need key
    // room 8 = use social network in battle
    // room 9 = use lizard spray in battle
    // room 10 = use data privacy regulations in battle

fn get_item(item: Item, at: GameState) -> (bool, usize) {
    let mut change_val: bool = false;
    let mut searching: bool = false;
    let mut time = at.timer;
    print!("({} minutes remain)\n> ", at.timer);
            let mut end = false;
            loop {
                if (searching && !change_val) || at.sunscreen {
                    loop {
                        if at.sunscreen {
                            print!("{}", item.light_desc);
                        } else {
                            print!("{}", item.dark_desc);
                        }
                        io::stdout().flush().unwrap();
                        let mut try_vr = String::new();
                        io::stdin().read_line(&mut try_vr).unwrap();
                        let try_vr = try_vr.trim();
                        if try_vr == "yes" {
                            change_val = true;
                            println!("{}", item.pick_up_text);
                            if item.name == "map" {
                                make_map(at.room_id)
                            }
                            end = true;
                            break;
                        } else if try_vr == "no" {
                            end = true;
                            break;
                        } else if try_vr == "i" {
                            display_inventory(at)
                        } else {
                            println!("You can't do that.")
                        }
                    }
                } else {
                    print!("Do you crawl around to see what is in the room? (yes or no -- cost = 2 min) \n> ");
                    io::stdout().flush().unwrap();
                    let mut crawl = String::new();
                    io::stdin().read_line(&mut crawl).unwrap();
                    let crawl = crawl.trim();
                    if crawl == "yes" {
                        time -= 2;
                        searching = true;
                    } else if crawl == "no" {
                        end = true;
                        break;
                    } else if crawl == "i" {
                        display_inventory(at)
                    } else {
                        println!("You can't do that.")
                    }
                }
                if end {
                    break;
                }
            }
            searching = false;
            return (change_val, time);
}
fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;

    let rooms = get_rooms();

    let end_rooms = [RoomID(8), RoomID(9)];
    let mut input = String::new();

    let mut at: GameState = GameState {
        room_id: RoomID(0),
        key: false,
        sunscreen: false,
        map: false,
        win: false,
        timer: 30,
        boss_hp: 2,
        spray: false,      // anti lizard spray
        data_reg: false,   // data-privacy regulations
        social_net: false, // copy of the Social Network
        searching: false,
    };

    title_screen();

    println!("You wake up, dazed and confused, in complete darkness. The last thing you remember is falling asleep at your job at Meta towards the end of your 80-hour work week. A million questions swirl around your mind, but right now there's only one thing to do. ESCAPE.");
    
    let vr_item: Item = Item {
        name: "map".into(),
        light_desc: "You see a VR headset on the ground - do you try it on? (yes or no) \n> ".into(),
        dark_desc: "As you crawl around, you feel some kind of headset - do you try it on? (yes or no) \n> ".into(),
        pick_up_text: "With the headset on, you see a map with your current location in AR! Turns out all the other features are locked in beta, but this will at least make things a little easier.".into(),
    };

    let key_item: Item = Item {
        name: "key".into(),
        light_desc: "You see a key - do you grab it? (yes or no) \n> ".into(),
        dark_desc: "You nearly trip over a key - do you grab it? (yes or no) \n> ".into(),
        pick_up_text: "You stuff it in your bag.".into(),
    };

    let sunscreen_item: Item = Item {
        name: "sunscreen".into(),
        light_desc: "You see a UV lamp - do you grab it? (yes or no) \n> ".into(),
        dark_desc: "You see a UV lamp - do you grab it? (yes or no) \n> ".into(),
        pick_up_text: "The lamp reveals the entire room with all its doorways, oddly enough lathered in sunscreen. With this, you'll have a much easier time getting around.".into(),
    };

    let data_item: Item = Item {
        name: "data".into(),
        light_desc: "You see a printed copy of data privacy regulations - do you grab it? (yes or no) \n> ".into(),
        dark_desc: "You feel a large stack of paper. ZUCK has been reading about data privacy regulations recently... - do you grab it? (yes or no) \n> ".into(),
        pick_up_text: "You stuff it in your bag.".into(),
    };

    let spray_item: Item = Item {
        name: "spray".into(),
        light_desc: "You see an anti-lizard spray - do you grab it? (yes or no) \n> ".into(),
        dark_desc: "You feel a canister... a spray... it could be for lizards... - do you grab it? (yes or no) \n> ".into(),
        pick_up_text: "You stuff it in your bag.".into(),
    };

    let movie_item: Item = Item {
        name: "movie".into(),
        light_desc: "You see a copy of The Social Network - do you grab it? (yes or no) \n> ".into(),
        dark_desc: "As you search around, you feel a DVD case. It feels like The Social Network... - do you grab it? (yes or no) \n> ".into(),
        pick_up_text: "You stuff it in your bag.".into(),
    };
    let mut count = 0;
    loop {
        // We don't want to move out of rooms, so we take a reference
        let here = &rooms[at.room_id.0];
        if !at.sunscreen {
            println!("{}\n{}", here.name, here.desc_dark);
        } else {
            println!("{}\n{}", here.name, here.desc_light);
        }

        // Don't print map if at boss battle 
        if at.map
            && (at.room_id.0 != 7)
            && (at.room_id.0 != 8)
            && (at.room_id.0 != 9)
            && (at.room_id.0 != 10)
        {
            make_map(at.room_id);
        }

        // get map logic
        if at.room_id == RoomID(2) && !at.map {
            let (change, time) = get_item(vr_item.clone(), at);
            if change {
                at.map = true
            }
            at.timer = time;
        }

        // get UV lamp logic
        if at.room_id == RoomID(4) && !at.sunscreen {
            print!("({} minutes remain)\n> ", at.timer);
            let mut end = false;
            loop {
                print!("You see a UV lamp - do you grab it? (yes or no) \n> ");
                io::stdout().flush().unwrap();
                let mut try_ss = String::new();
                io::stdin().read_line(&mut try_ss).unwrap();
                let try_ss = try_ss.trim();
                if try_ss == "yes" {
                    at.sunscreen = true;
                    println!("The lamp reveals the entire room with all its doorways, oddly enough lathered in sunscreen. With this, you'll have a much easier time getting around.");
                    break;
                } else if try_ss == "no" {
                    break;
                } else if try_ss == "i" {
                    display_inventory(at)
                } else {
                    println!("You can't do that.")
                }
            }
        }

        // get social_net logic
        if at.room_id == RoomID(0) && !at.social_net {
            let (change, time) = get_item(movie_item.clone(), at);
            if change {
                at.social_net = true
            }
            at.timer = time;
        }

        // get data_reg logic
        if at.room_id == RoomID(1) && !at.data_reg {
            let (change, time) = get_item(data_item.clone(), at);
            if change {
                at.data_reg = true
            }
            at.timer = time;
        }

        // get spray logic
        if at.room_id == RoomID(5) && !at.spray {
            let (change, time) = get_item(spray_item.clone(), at);
            if change {
                at.spray = true
            }
            at.timer = time;
        }

        // get key logic
        if at.room_id == RoomID(3) && !at.key {
            let (change, time) = get_item(key_item.clone(), at);
            if change {
                at.key = true
            }
            at.timer = time;
        }
        loop {
            print!("What will you do?\n> ");
            print!("({} minutes remain)\n> ", at.timer);
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "i" {
                display_inventory(at);
            } else {
                if let Some(door) = here
                    .doors
                    .iter()
                    .find(|d| d.triggers.iter().any(|t| *t == input))
                {
                    if let Some(msg) = &door.message {
                        println!("{}", msg);
                    }
                    if door.required_item == 1 {
                        if at.key {
                            at.room_id = door.target;
                            at.timer -= 1;
                            at.key = false;
                        } else {
                            println!("\n");
                            println!("\n**You don't have the necessary item!**\n");
                            println!("\n");
                        }
                    } else if door.required_item == 2 {
                        if at.spray {
                            at.room_id = door.target;
                            at.timer -= 1;
                            at.spray = false;
                            at.boss_hp -= 1;
                        } else {
                            println!("\n");
                            println!("\n**You don't have the necessary item!**\n");
                            println!("\n");
                        }
                    } else if door.required_item == 3 {
                        if at.data_reg {
                            at.room_id = door.target;
                            at.timer -= 1;
                            at.data_reg = false;
                        } else {
                            println!("\n");
                            println!("\n**You don't have the necessary item!**\n");
                            println!("\n");
                        }
                    } else if door.required_item == 4 {
                        if at.social_net {
                            at.room_id = door.target;
                            at.timer -= 1;
                            at.social_net = false;
                            at.boss_hp -= 1;
                        } else {
                            println!("\n");
                            println!("\n**You don't have the necessary item!**\n");
                            println!("\n");
                        }
                    } else {
                        at.room_id = door.target;
                        at.timer -= 1;
                    }
                    // print zuck ascii when we get to boss fight (only once)
                    if at.room_id.0 == 7 && count == 0 {
                        print_zuck();
                        count += 1;
                    }
                    break;
                } else {
                    at.timer -= 1;
                    if at.timer == 0 {
                        break;
                    }
                    println!("That doesn't seem to work, unfortunately. Your breathing starts to get more labored.");
                }
            }
        }
        if end_rooms.contains(&at.room_id) {
            if at.boss_hp == 0 {
                let here = &rooms[at.room_id.0];
                println!("{}\n{}", here.name, here.desc_light);
                at.win = true;
            }
        }
        // game is over conditions
        if at.win {
            break;
        }
        if at.timer == 0 {
            break;
        }
    }
    if at.win {
        println!("ZUCK collapses. You rush out the exit.");
        println!("You see a blinding light...as you step forward you emerge onto the sunny streets of Menlo Park. A new, yet uncertain future awaits for you.");
        println!("You win!!!");
        println!("THE END");
    }
    if !at.win {
        println!("You collapse to the ground, out of breath. As your vision starts to fade, you see ZUCK crouching over you, smiling. The last words you hear are: 'unfortunately there's going to be another around of layoffs...'");
        println!("You lose...");
        println!("THE END");  
    }
}