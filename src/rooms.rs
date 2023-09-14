use crate::Room;
use crate::Door;
use crate::RoomID;

pub fn get_rooms() -> [Room;11] { return [
        Room {
            name: "The Start\n-----------------------".into(), // Turn a &'static string (string constant) into a String
            desc_light: "You see an entryway to the north, east, and south.".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. There's got to be a better way...".into(),
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
                    required_item: 47,
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
                    required_item: 47,
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
                    required_item: 47,
                },
            ],
        },
        Room {
            name: "Whatsapp Room\n-----------------------".into(),
            desc_light: "its light".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. There's got to be a better way...".into(),
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
                    required_item: 47,
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
                    required_item: 47,
                },
            ],
        },
        Room {
            name: "VR Room\n-----------------------".into(),
            desc_light: "Dozens of Oculus headsets are strewn.".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. There's got to be a better way...".into(),
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
                    required_item: 47,
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
                    required_item: 47,
                },
            ],
        },
        Room {
            name: "The Library\n-----------------------".into(),
            desc_light: "Stacks of books line the walls.".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. There's got to be a better way...".into(),
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
                    required_item: 47,
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
                    required_item: 47,
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
                    required_item: 47,
                },
            ],
        },
        Room {
            name: "The Sunscreen Room\n-----------------------".into(),
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
                    required_item: 47,
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
                    required_item: 47,
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
                    required_item: 47,
                },
            ],
        },
        Room {
            name: "The Final Room\n-----------------------".into(),
            desc_light: "A final door leads to the south. It's locked. You have a sense there's no turning back after this...".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. There's got to be a better way...".into(),
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
                    required_item: 47,
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
                    required_item: 1,
                },
            ],
        },
        Room {
            name: "???\n-----------------------".into(),
            desc_light: "Doesn't seem to be anything here.".into(),
            desc_dark: "It's pitch black, and you're forced to choose a direction hoping there's something there. There's got to be a better way...".into(),
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
                required_item: 47,
            }],
        },
        Room {
            name: "BOSS FIGHT\n-----------------------".into(),
            desc_light: "You hear a hiss: 'I think it's time for an employee review.' ZUCK appears, blocking your path. There's no turning back now, you have to use something in the inventory against him.".into(),
            desc_dark: "You hear a hiss: 'I think it's time for an employee review.' ZUCK appears, blocking your path. There's no turning back now, you have to use something in the inventory against him.".into(),
            doors: vec![Door {
                target: RoomID(8),
                triggers: vec![
                    "use movie".into(),
                    "movie".into(),
                    "the social network".into(),
                    "show movie".into(),
                    "show social network".into(),
                    "use social network".into(),
                    "use the movie".into(),
                    "show the social network".into(),
                    "use the social network".into(),
                    "social network".into(),
                ],
                message: None,
                required_item: 4,
            }, Door {
                target: RoomID(9),
                triggers: vec![
                    "use lizard spray".into(),
                    "spray".into(),
                    "use spray".into(),
                    "use the spray".into(),
                    "spray him".into(),
                    "lizard spray".into(),
                    "anti-lizard spray".into(),
                ],
                message: None,
                required_item: 2,
            },
            Door {
                target: RoomID(10),
                triggers: vec![
                    "use regulations".into(),
                    "regulations".into(),
                    "show regulations".into(),
                    "show the regulations".into(),
                    "use data privacy regulations".into(),
                    "use the data privacy regulations".into(),
                    "show the data privacy regulations".into(),
                    "use data-privacy regulations".into(),
                    "use the data-privacy regulations".into(),
                    "show the data-privacy regulations".into(),
                    "data-privacy".into(),
                    "data privacy".into(),
                ],
                message: None,
                required_item: 3,
            },
            ],
        },
        Room {
            name: "BOSS FIGHT\n-----------------------".into(),
            desc_light: "You show him a copy of the popular 2010s movie, the social network. ZUCK starts groaning in agony.".into(),
            desc_dark: "You show him a copy of the popular 2010s movie, the social network. ZUCK starts groaning in agony.".into(),
            doors: vec![Door {
                target: RoomID(9),
                triggers: vec![
                    "use lizard spray".into(),
                    "spray".into(),
                    "use spray".into(),
                    "use the spray".into(),
                    "spray him".into(),
                    "lizard spray".into(),
                    "anti-lizard spray".into(),
                ],
                message: None,
                required_item: 2,
            },
            Door {
                target: RoomID(10),
                triggers: vec![
                    "use regulations".into(),
                    "regulations".into(),
                    "show regulations".into(),
                    "show the regulations".into(),
                    "use data privacy regulations".into(),
                    "use the data privacy regulations".into(),
                    "show the data privacy regulations".into(),
                    "data-privacy".into(),
                    "data privacy".into(),
                ],
                message: None,
                required_item: 3,
            },
            ],
        },
        Room {
            name: "BOSS FIGHT\n-----------------------".into(),
            desc_light: "You spray ZUCK with the lizard repellent. He starts coughing heavily, and falls to his knees.".into(),
            desc_dark: "You spray ZUCK with the lizard repellent. He starts coughing heavily, and falls to his knees.".into(),
            doors: vec![Door {
                target: RoomID(8),
                triggers: vec![
                    "use movie".into(),
                    "movie".into(),
                    "the social network".into(),
                    "show movie".into(),
                    "show social network".into(),
                    "use social network".into(),
                    "use the movie".into(),
                    "show the social network".into(),
                    "use the social network".into(),
                    "social network".into(),
                ],
                message: None,
                required_item: 4,
            },
            Door {
                target: RoomID(10),
                triggers: vec![
                    "use regulations".into(),
                    "regulations".into(),
                    "show regulations".into(),
                    "show the regulations".into(),
                    "use data privacy regulations".into(),
                    "use the data privacy regulations".into(),
                    "show the data privacy regulations".into(),
                    "data-privacy".into(),
                    "data privacy".into(),
                ],
                message: None,
                required_item: 3,
            },
            ],
        },
        Room {
            name: "BOSS FIGHT\n-----------------------".into(),
            desc_light: "You show ZUCK the data privacy regulations. He scoffs, unaffected. 'These people just submit their data anyway. They 'trust me'. Dumb fucks.'".into(),
            desc_dark: "You show ZUCK the data privacy regulations. He scoffs, unaffected. 'These people just submit their data anyway. They 'trust me'. Dumb fucks.'".into(),
            doors: vec![Door {
                target: RoomID(8),
                triggers: vec![
                    "use movie".into(),
                    "movie".into(),
                    "the social network".into(),
                    "show movie".into(),
                    "show social network".into(),
                    "use social network".into(),
                    "use the movie".into(),
                    "show the social network".into(),
                    "use the social network".into(),
                    "social network".into(),
                ],
                message: None,
                required_item: 4,
            }, Door {
                target: RoomID(9),
                triggers: vec![
                    "use lizard spray".into(),
                    "spray".into(),
                    "use spray".into(),
                    "use the spray".into(),
                    "spray him".into(),
                    "lizard spray".into(),
                    "anti-lizard spray".into(),
                ],
                message: None,
                required_item: 2,
            },
            ],
        },
    ];
}