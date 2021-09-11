use bevy::prelude::{Res, ResMut};
use rand::seq::SliceRandom;

use crate::space_core::resources::{
    used_names::UsedNames
};

const MALE_FIRST_NAMES: [&str; 102] = [
    "Anakin",
    "Angel",
    "Artemis",
    "Arthur",
    "Bastian",
    "Cullen",
    "Emmett",
    "Falkor",
    "Faramir",
    "Fox",
    "Gandalf",
    "Gaius",
    "Geordi",
    "Grant",
    "Han",
    "Harry",
    "Hugo",
    "Idris",
    "Jareth",
    "Joffrey",
    "John",
    "Kael",
    "Logan",
    "Ludo",
    "Mary",
    "Milo",
    "Odo",
    "Ronan",
    "Rory",
    "Rowan",
    "Rylan",
    "Sauron",
    "Septimus",
    "Spike",
    "Spock",
    "Sulu",
    "Tyrian",
    "Westley",
    "William",
    "Xavier",
    "Zack",
    "Yarian",
    "Zorrish",
    "Samlan",
    "Wylran",
    "Elldrick",
    "Archiah",
    "Sorkkon",
    "Xiah",
    "Yazan",
    "Ryland",
    "Kaiton",
    "Aidken",
    "Gideon",
    "Kieran",
    "Ureem",
    "Malax",
    "Kalban",
    "Wavarek",
    "Rex",
    "Yariq",
    "Tariq",
    "Finriel",
    "Israel",
    "Xumir",
    "Irivan",
    "Samion",
    "Finnec",
    "Falko",
    "Waverek",
    "Parker",
    "Ronias",
    "Orby",
    "Tiran",
    "Kalett",
    "Yarwick",
    "Jango",
    "Brolek",
    "Xavian",
    "Sorkku",
    "Ignazlan",
    "Lyrikkon",
    "Kelslow",
    "Iklan",
    "Zannik",
    "Fenncom",
    "Emsen",
    "Kartan",
    "Yazan",
    "Blayden",
    "Kellek",
    "Wayven",
    "Pakon",
    "Kenmon",
    "Graygal",
    "Bobba",
    "Cadael",
    "Xantry",
    "Bengorn",
    "Yaddu",
    "Ikev",
    "Lokesh"
];

const FEMALE_FIRST_NAMES : [&str;44] = [
    "Aeryn",
    "Amelia",
    "Anastasia",
    "Aquila",
    "Arya",
    "Astrid",
    "Padme",
    "Aurora",
    "Aurra",
    "Auryn",
    "Buttercup",
    "Cherlindrea",
    "Clara",
    "Cora",
    "Danan",
    "Diana",
    "Donna",
    "Echo",
    "Elora",
    "Eowyn",
    "Felicity",
    "Fleur",
    "Galadriel",
    "Glinda",
    "Isabaeu",
    "Kamala",
    "Kara",
    "Kathryn",
    "Anno",
    "Katniss",
    "Nyota",
    "Padme",
    "Peggy",
    "Raven",
    "Renesmee",
    "Ripley",
    "River",
    "Sarah",
    "Sonya",
    "Sorsha",
    "Tauriel",
    "Teyla",
    "Valerian",
    "Willow"
];

const LAST_NAMES : [&str;47] = [
	"Voight",
	"Barick",
	"Nicolau",
	"Cantos",
	"Tian",
	"Carthen",
	"McRaven",
	"Foxwell",
	"Fett",
	"Albach",
	"Amidala",
	"Steward",
	"Woldt",
	"Cedeno",
	"Catlow",
	"Kinton",
	"Zahra",
	"Castillion",
	"Nyseth",
	"Rhyne",
	"Malik",
	"Sonoda",
	"Avison",
	"Philips",
	"Sarratt",
	"Zechiel",
	"Callahan",
	"Chrysalis",
	"Nadir",
	"Corona",
	"Rahman",
	"Alastair",
	"Haskovo",
	"Vitality",
	"Sharjah",
	"Khepri",
	"Raptor",
	"Colfax",
	"Moondust",
	"Atrius",
	"Dianthus",
	"Kelmis",
	"Bani-Mazar",
	"Alpheus",
	"Skywalker",
	"Kenobi",
	"Maul"
];


pub fn get_full_name(gender : bool, unique : bool, used_names : &Res<UsedNames>) -> String {

    let rng = &mut rand::thread_rng();

    let first_name : &str;

    match gender {
        true => {
            first_name = MALE_FIRST_NAMES.choose(rng).unwrap();
        },
        false => {
            first_name = FEMALE_FIRST_NAMES.choose(rng).unwrap();
        }
    }

    let rng2 = &mut rand::thread_rng();


    let full_name : String = first_name.to_owned() + " " + LAST_NAMES.choose(rng2).unwrap();

    if unique == true {
        if used_names.names.contains_key(&full_name) {
            get_full_name(gender, unique, used_names);
        }
    }

    full_name

}

pub fn get_dummy_name(used_names : &mut ResMut<UsedNames>) -> String {

    let return_name = "Dummy ".to_string() + &used_names.dummy_i.to_string();

    used_names.dummy_i +=1;

    return_name

}
