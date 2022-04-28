use bevy_ecs::system::{Res, ResMut};
use rand::seq::SliceRandom;

use crate::core::pawn::resources::UsedNames;

#[allow(dead_code)]
pub struct FullNameGenerator {
    male_first_names: Vec<String>,
    female_first_names: Vec<String>,
    last_names: Vec<String>,
}

impl FullNameGenerator {
    pub fn new() -> Self {
        Self {
            male_first_names: vec![
                "Anakin".to_string(),
                "Angel".to_string(),
                "Abel".to_string(),
                "Artemis".to_string(),
                "Arthur".to_string(),
                "Bastian".to_string(),
                "Cullen".to_string(),
                "Emmett".to_string(),
                "Falkor".to_string(),
                "Faramir".to_string(),
                "Fox".to_string(),
                "Gandalf".to_string(),
                "Gaius".to_string(),
                "Geordi".to_string(),
                "Grant".to_string(),
                "Han".to_string(),
                "Harry".to_string(),
                "Hugo".to_string(),
                "Idris".to_string(),
                "Jareth".to_string(),
                "Joffrey".to_string(),
                "John".to_string(),
                "Kael".to_string(),
                "Logan".to_string(),
                "Ludo".to_string(),
                "Mary".to_string(),
                "Milo".to_string(),
                "Odo".to_string(),
                "Ronan".to_string(),
                "Rory".to_string(),
                "Rowan".to_string(),
                "Rylan".to_string(),
                "Sauron".to_string(),
                "Septimus".to_string(),
                "Spike".to_string(),
                "Spock".to_string(),
                "Sulu".to_string(),
                "Tyrian".to_string(),
                "Westley".to_string(),
                "William".to_string(),
                "Xavier".to_string(),
                "Zack".to_string(),
                "Yarian".to_string(),
                "Zorrish".to_string(),
                "Samlan".to_string(),
                "Wylran".to_string(),
                "Elldrick".to_string(),
                "Archiah".to_string(),
                "Sorkkon".to_string(),
                "Xiah".to_string(),
                "Yazan".to_string(),
                "Ryland".to_string(),
                "Kaiton".to_string(),
                "Aidken".to_string(),
                "Gideon".to_string(),
                "Kieran".to_string(),
                "Ureem".to_string(),
                "Malax".to_string(),
                "Kalban".to_string(),
                "Wavarek".to_string(),
                "Rex".to_string(),
                "Yariq".to_string(),
                "Tariq".to_string(),
                "Finriel".to_string(),
                "Israel".to_string(),
                "Xumir".to_string(),
                "Irivan".to_string(),
                "Samion".to_string(),
                "Finnec".to_string(),
                "Falko".to_string(),
                "Waverek".to_string(),
                "Parker".to_string(),
                "Ronias".to_string(),
                "Orby".to_string(),
                "Tiran".to_string(),
                "Steve".to_string(),
                "Steven".to_string(),
                "Kalett".to_string(),
                "Yarwick".to_string(),
                "Jango".to_string(),
                "Brolek".to_string(),
                "Xavian".to_string(),
                "Sorkku".to_string(),
                "Ignazlan".to_string(),
                "Lyrikkon".to_string(),
                "Kelslow".to_string(),
                "Iklan".to_string(),
                "Zannik".to_string(),
                "Fenncom".to_string(),
                "Emsen".to_string(),
                "Kartan".to_string(),
                "Yazan".to_string(),
                "Blayden".to_string(),
                "Kellek".to_string(),
                "Wayven".to_string(),
                "Pakon".to_string(),
                "Kenmon".to_string(),
                "Graygal".to_string(),
                "Bobba".to_string(),
                "Cadael".to_string(),
                "Xantry".to_string(),
                "Bengorn".to_string(),
                "Yaddu".to_string(),
                "Ikev".to_string(),
                "Lokesh".to_string(),
                "Wolf".to_string(),
                "Falco".to_string(),
            ],
            female_first_names: vec![
                "Aeryn".to_string(),
                "Amelia".to_string(),
                "Anastasia".to_string(),
                "Aquila".to_string(),
                "Arya".to_string(),
                "Astrid".to_string(),
                "Padme".to_string(),
                "Aurora".to_string(),
                "Aurra".to_string(),
                "Auryn".to_string(),
                "Buttercup".to_string(),
                "Cherlindrea".to_string(),
                "Clara".to_string(),
                "Cora".to_string(),
                "Danan".to_string(),
                "Diana".to_string(),
                "Donna".to_string(),
                "Echo".to_string(),
                "Elora".to_string(),
                "Eowyn".to_string(),
                "Felicity".to_string(),
                "Fleur".to_string(),
                "Galadriel".to_string(),
                "Glinda".to_string(),
                "Isabaeu".to_string(),
                "Kamala".to_string(),
                "Kara".to_string(),
                "Kathryn".to_string(),
                "Anno".to_string(),
                "Katniss".to_string(),
                "Nyota".to_string(),
                "Nancy".to_string(),
                "Padme".to_string(),
                "Peggy".to_string(),
                "Raven".to_string(),
                "Renesmee".to_string(),
                "Ripley".to_string(),
                "River".to_string(),
                "Sarah".to_string(),
                "Sonya".to_string(),
                "Sorsha".to_string(),
                "Tauriel".to_string(),
                "Teyla".to_string(),
                "Valerian".to_string(),
                "Willow".to_string(),
                "Krystal".to_string(),
                "Katelyne".to_string(),
            ],
            last_names: vec![
                "Voight".to_string(),
                "Barick".to_string(),
                "Nicolau".to_string(),
                "Cantos".to_string(),
                "Tian".to_string(),
                "Carthen".to_string(),
                "McRaven".to_string(),
                "Foxwell".to_string(),
                "Fett".to_string(),
                "Albach".to_string(),
                "Amidala".to_string(),
                "Steward".to_string(),
                "Woldt".to_string(),
                "Cedeno".to_string(),
                "Catlow".to_string(),
                "Kinton".to_string(),
                "Zahra".to_string(),
                "Castillion".to_string(),
                "Nyseth".to_string(),
                "Rhyne".to_string(),
                "Malik".to_string(),
                "Sonoda".to_string(),
                "Avison".to_string(),
                "Philips".to_string(),
                "Sarratt".to_string(),
                "Zechiel".to_string(),
                "Callahan".to_string(),
                "Chrysalis".to_string(),
                "Nadir".to_string(),
                "Corona".to_string(),
                "Rahman".to_string(),
                "Alastair".to_string(),
                "Haskovo".to_string(),
                "Vitality".to_string(),
                "Sharjah".to_string(),
                "Khepri".to_string(),
                "Raptor".to_string(),
                "Colfax".to_string(),
                "Moondust".to_string(),
                "Atrius".to_string(),
                "Dianthus".to_string(),
                "Kelmis".to_string(),
                "Bani-Mazar".to_string(),
                "Alpheus".to_string(),
                "Skywalker".to_string(),
                "Kenobi".to_string(),
                "Maul".to_string(),
                "McCloud".to_string(),
                "O'Donnell".to_string(),
                "Lombardi".to_string(),
            ],
        }
    }
}

pub fn get_full_name(gender: bool, unique: bool, used_names: &Res<UsedNames>) -> String {
    let rng = &mut rand::thread_rng();

    let first_name: &str;

    let names = FullNameGenerator::new();

    match gender {
        true => {
            first_name = names.male_first_names.choose(rng).unwrap();
        }
        false => {
            first_name = names.male_first_names.choose(rng).unwrap();
        }
    }

    let rng2 = &mut rand::thread_rng();

    let full_name: String = first_name.to_owned() + " " + names.last_names.choose(rng2).unwrap();

    if unique == true {
        if used_names.names.contains_key(&full_name) {
            get_full_name(gender, unique, used_names);
        }
    }

    full_name
}

pub fn get_dummy_name(used_names: &mut ResMut<UsedNames>) -> String {
    let return_name = "Dummy ".to_string() + &used_names.dummy_i.to_string();

    used_names.dummy_i += 1;

    return_name
}
