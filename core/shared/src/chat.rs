use const_format::concatcp;
use rand::Rng;

pub const BILLBOARD_SHOUT_FONT: &str =
    "res://assets/fonts/RobotoFamily/RobotoCondensed/RobotoCondensed-BoldShoutDyna.tres";
pub const BILLBOARD_SHOUT_ITALIC_FONT: &str =
    "res://assets/fonts/RobotoFamily/RobotoCondensed/RobotoCondensed-BoldShoutItalicDyna.tres";

pub const NEARBY_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularBoldDyna.tres";
pub const _NEARBY_ITALIC_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumItDyna.tres";
pub const _NEARBY_NORMAL_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumDyna.tres";
pub const NEARBY_SHOUT_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldDyna.tres";
pub const _NEARBY_MACHINE_MEDIUM_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightMediumDyna.tres";
pub const _NEARBY_MACHINE_ITALIC_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicDyna.tres";
pub const _NEARBY_MACHINE_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightBoldDyna.tres";
pub const _NEARBY_MACHINE_ITALIC_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicBoldDyna.tres";

pub const FURTHER_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularBoldDynaSmall.tres";
pub const FURTHER_ITALIC_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumItDynaSmall.tres";
pub const FURTHER_NORMAL_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumDynaSmall.tres";
pub const FURTHER_SHOUT_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldDynaSmall.tres";
pub const FURTHER_MACHINE_MEDIUM_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightMediumDynaSmall.tres";
pub const FURTHER_MACHINE_ITALIC_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicDynaSmall.tres";
pub const FURTHER_MACHINE_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightBoldDynaSmall.tres";
pub const _FURTHER_MACHINE_ITALIC_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicBoldDynaSmall.tres";

pub const FAR_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularBoldDynaSmaller.tres";
pub const FAR_ITALIC_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumItDynaSmaller.tres";
pub const FAR_NORMAL_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumDynaSmaller.tres";
pub const FAR_SHOUT_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldDynaSmaller.tres";
pub const FAR_MACHINE_MEDIUM_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightMediumDynaSmaller.tres";
pub const FAR_MACHINE_ITALIC_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicDynaSmaller.tres";
pub const FAR_MACHINE_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightBoldDynaSmaller.tres";
pub const _FAR_MACHINE_ITALIC_BOLD_FONT: &str =
    "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicBoldDynaSmaller.tres";

pub const ASTRIX: &str = "[color=#bdbdbd]*******[/color]";

pub const HEALTHY_COLOR: &str = "#3cff00";
pub const UNHEALTHY_COLOR: &str = "#ff003c";
pub const GOLD_COLOR: &str = "#ffea00";
pub const WARNING_COLOR: &str = "#ffa600";

pub const TALK_STYLE_STANDARD_STANDARD: &str = "says";
pub const TALK_STYLE_STANDARD_EXCLAIMS: &str = "exclaims";
pub const TALK_STYLE_STANDARD_SHOUTS: &str = "shouts";
pub const TALK_STYLE_STANDARD_ASKS: &str = "asks";

pub const TALK_STYLE_MACHINE_STANDARD: &str = "states";
pub const TALK_STYLE_MACHINE_EXCLAIMS: &str = "announces";
pub const TALK_STYLE_MACHINE_SHOUTS: &str = "shouts";
pub const TALK_STYLE_MACHINE_ASKS: &str = "queries";

pub const TALK_DATA_STANDARD_I_NEARBY_START: &str = "[i]";
pub const TALK_DATA_STANDARD_I_NEARBY_END: &str = "[/i]";

pub const TALK_DATA_STANDARD_I_FURTHER_START: &str = concatcp!("[font=", FURTHER_ITALIC_FONT, "]");
pub const TALK_DATA_STANDARD_I_FURTHER_END: &str = "[/font]";

pub const TALK_DATA_STANDARD_I_FAR_START: &str = concatcp!("[font=", FAR_ITALIC_FONT, "]");
pub const TALK_DATA_STANDARD_I_FAR_END: &str = "[/font]";

pub const TALK_DATA_STANDARD_B_NEARBY_START: &str = "[b]";
pub const TALK_DATA_STANDARD_B_NEARBY_END: &str = "[/b]";

pub const TALK_DATA_STANDARD_B_FURTHER_START: &str = concatcp!("[font=", FURTHER_BOLD_FONT, "]");
pub const TALK_DATA_STANDARD_B_FURTHER_END: &str = "[/font]";

pub const TALK_DATA_STANDARD_B_FAR_START: &str = concatcp!("[font=", FAR_BOLD_FONT, "]");
pub const TALK_DATA_STANDARD_B_FAR_END: &str = "[/font]";

pub const TALK_DATA_STANDARD_NORMAL_NEARBY_START: &str = "";
pub const TALK_DATA_STANDARD_NORMAL_NEARBY_END: &str = "";

pub const TALK_DATA_STANDARD_NORMAL_FURTHER_START: &str =
    concatcp!("[font=", FURTHER_NORMAL_FONT, "]");
pub const TALK_DATA_STANDARD_NORMAL_FURTHER_END: &str = "[/font]";

pub const TALK_DATA_STANDARD_NORMAL_FAR_START: &str = concatcp!("[font=", FAR_NORMAL_FONT, "]");
pub const TALK_DATA_STANDARD_NORMAL_FAR_END: &str = "[/font]";

pub const TALK_DATA_MACHINE_I_NEARBY_START: &str = "[i]";
pub const TALK_DATA_MACHINE_I_NEARBY_END: &str = "[/i]";

pub const TALK_DATA_MACHINE_I_FURTHER_START: &str =
    concatcp!("[font=", FURTHER_MACHINE_ITALIC_FONT, "]");
pub const TALK_DATA_MACHINE_I_FURTHER_END: &str = "[/font]";

pub const TALK_DATA_MACHINE_I_FAR_START: &str = concatcp!("[font=", FAR_MACHINE_ITALIC_FONT, "]");
pub const TALK_DATA_MACHINE_I_FAR_END: &str = "[/font]";

pub const TALK_DATA_MACHINE_B_NEARBY_START: &str = "[b]";
pub const TALK_DATA_MACHINE_B_NEARBY_END: &str = "[/b]";

pub const TALK_DATA_MACHINE_B_FURTHER_START: &str =
    concatcp!("[font=", FURTHER_MACHINE_BOLD_FONT, "]");
pub const TALK_DATA_MACHINE_B_FURTHER_END: &str = "[/font]";

pub const TALK_DATA_MACHINE_B_FAR_START: &str = concatcp!("[font=", FAR_MACHINE_BOLD_FONT, "]");
pub const TALK_DATA_MACHINE_B_FAR_END: &str = "[/font]";

pub const TALK_DATA_MACHINE_NORMAL_NEARBY_START: &str = "";
pub const TALK_DATA_MACHINE_NORMAL_NEARBY_END: &str = "";

pub const TALK_DATA_MACHINE_NORMAL_FURTHER_START: &str =
    concatcp!("[font=", FURTHER_MACHINE_MEDIUM_FONT, "]");
pub const TALK_DATA_MACHINE_NORMAL_FURTHER_END: &str = "[/font]";

pub const TALK_DATA_MACHINE_NORMAL_FAR_START: &str =
    concatcp!("[font=", FAR_MACHINE_MEDIUM_FONT, "]");
pub const TALK_DATA_MACHINE_NORMAL_FAR_END: &str = "[/font]";

pub const SHOUT_DATA_STANDARD_NEARBY_I_START: &str =
    "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDyna.tres]";
pub const SHOUT_DATA_STANDARD_NEARBY_I_END: &str = "[/font]";

pub const SHOUT_DATA_STANDARD_FURTHER_I_START: &str =
    "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmall.tres]";
pub const SHOUT_DATA_STANDARD_FURTHER_I_END: &str = "[/font]";

pub const SHOUT_DATA_STANDARD_FAR_I_START: &str =
    "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmaller.tres]";
pub const SHOUT_DATA_STANDARD_FAR_I_END: &str = "[/font]";

pub const SHOUT_DATA_MACHINE_NEARBY_I_START: &str =
    "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDyna.tres]";
pub const SHOUT_DATA_MACHINE_NEARBY_I_END: &str = "[/font]";

pub const SHOUT_DATA_MACHINE_FURTHER_I_START: &str =
    "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmall.tres]";
pub const SHOUT_DATA_MACHINE_FURTHER_I_END: &str = "[/font]";

pub const SHOUT_DATA_MACHINE_FAR_I_START: &str =
    "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmaller.tres]";
pub const SHOUT_DATA_MACHINE_FAR_I_END: &str = "[/font]";

pub const TALK_TYPE_STANDARD_NEARBY_START: &str = "";
pub const TALK_TYPE_STANDARD_NEARBY_END: &str = "";
pub const _TALK_TYPE_STANDARD_FURTHER_START: &str = "";
pub const _TALK_TYPE_STANDARD_FURTHER_END: &str = "";
pub const _TALK_TYPE_STANDARD_FAR_START: &str = "";
pub const _TALK_TYPE_STANDARD_FAR_END: &str = "";

pub const TALK_TYPE_MACHINE_NEARBY_START: &str = "[i]";
pub const TALK_TYPE_MACHINE_NEARBY_END: &str = "[/i]";
pub const _TALK_TYPE_MACHINE_FURTHER_START: &str = "[i]";
pub const _TALK_TYPE_MACHINE_FURTHER_END: &str = "[/i]";
pub const _TALK_TYPE_MACHINE_FAR_START: &str = "[i]";
pub const _TALK_TYPE_MACHINE_FAR_END: &str = "[/i]";

pub const TALK_SPACE_GLOBAL_CHATPREFIX: &str = "/global";

pub const TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX: &str = "/me";
pub const TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBSTART: &str = "[color=#dbdbdb]";
pub const TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBEND: &str = "[/color]";
pub const TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBSTART: &str = "[color=#e6e6e6]";
pub const TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBEND: &str = "[/color]";

pub const _TALK_SPACE_PROXIMITY_CHATPREFIX: &str = "";
pub const TALK_SPACE_PROXIMITY_PREFIXBBSTART: &str = "[color=#dbdbdb]";
pub const TALK_SPACE_PROXIMITY_PREFIXBBEND: &str = "[/color]";
pub const TALK_SPACE_PROXIMITY_MESSAGEBBSTART: &str = "[color=#e6e6e6]";
pub const TALK_SPACE_PROXIMITY_MESSAGEBBEND: &str = "[/color]";

pub const TALK_SPACE_COMMON_CHATPREFIX: &str = ";";
pub const TALK_SPACE_COMMON_PREFIXBBSTART: &str = "[color=#6ce07a]";
pub const TALK_SPACE_COMMON_PREFIXBBEND: &str = "[/color]";
pub const TALK_SPACE_COMMON_MESSAGEBBSTART: &str = "[color=#68de77]";
pub const TALK_SPACE_COMMON_MESSAGEBBEND: &str = "[/color]";

pub const TALK_SPACE_SECURITY_CHATPREFIX: &str = ":s";
pub const TALK_SPACE_SECURITY_PREFIXBBSTART: &str = "[color=#f24141]";
pub const TALK_SPACE_SECURITY_PREFIXBBEND: &str = "[/color]";
pub const TALK_SPACE_SECURITY_MESSAGEBBSTART: &str = "[color=#fc3d3d]";
pub const TALK_SPACE_SECURITY_MESSAGEBBEND: &str = "[/color]";

pub const TALK_SPACE_SPECIALOPS_CHATPREFIX: &str = ".";
pub const TALK_SPACE_SPECIALOPS_PREFIXBBSTART: &str = "[color=#f24141]";
pub const TALK_SPACE_SPECIALOPS_PREFIXBBEND: &str = "[/color]";
pub const TALK_SPACE_SPECIALOPS_MESSAGEBBSTART: &str = "[color=#fc3d3d]";
pub const TALK_SPACE_SPECIALOPS_MESSAGEBBEND: &str = "[/color]";

pub const BILLBOARD_DATA_SECURITY_START: &str = "[center][color=#ff7070]";
pub const BILLBOARD_DATA_SECURITY_END: &str = "[/color][/center]";

pub const _BILLBOARD_DATA_SPECIALOPS_START: &str = "[center][color=#ff7070]";
pub const _BILLBOARD_DATA_SPECIALOPS_END: &str = "[/color][/center]";

pub const TALK_SPACE_COMMON_WORD: &str = "Common";
pub const TALK_SPACE_SECURITY_WORD: &str = "Security";
pub const TALK_SPACE_SPECIALOPS_WORD: &str = "Spec-op";

pub const JOB_SECURITY_WORD: &str = "Security";
pub const JOB_CONTROL_WORD: &str = "Control";

const COMMUNITY_HREF_COLOR: &str = "#5c4aff";

// Updates chat ButtonOption list for clients.
pub fn get_talk_spaces_setupui() -> Vec<(String, String)> {
    vec![(
        "Global".to_string(),
        TALK_SPACE_GLOBAL_CHATPREFIX.to_string(),
    )]
}
pub const EXAMINATION_EMPTY: &str = "You cannot see what is there.";

pub fn get_empty_cell_message() -> String {
    "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n" + EXAMINATION_EMPTY
}
pub fn get_space_message() -> String {
    let mut rng = rand::thread_rng();
    let random_pick: i32 = rng.gen_range(0..3);

    let mut msg = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
    msg = msg + "[font=" + FURTHER_ITALIC_FONT + "]" + "You examine the empty space.[/font]\n";

    if random_pick == 0 {
        msg = msg + "You are starstruck by the sight of space.";
    } else if random_pick == 1 {
        msg = msg + "That certainly looks like space.";
    } else {
        msg = msg + "Space.";
    }

    msg.to_string()
}
pub fn get_talk_spaces() -> Vec<(String, String)> {
    vec![
        ("Local".to_string(), "".to_string()),
        (
            "Me".to_string(),
            TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX.to_string(),
        ),
        (
            "Common".to_string(),
            TALK_SPACE_COMMON_CHATPREFIX.to_string(),
        ),
        (
            "Security".to_string(),
            TALK_SPACE_SECURITY_CHATPREFIX.to_string(),
        ),
        (
            "Global".to_string(),
            TALK_SPACE_GLOBAL_CHATPREFIX.to_string(),
        ),
    ]
}

pub fn escape_bb(string: String, partially: bool, escape_special_chars: bool) -> String {
    let mut new_string = string.escape_default().to_string();

    new_string = new_string.replace("[", "(");
    new_string = new_string.replace("]", ")");

    if partially {
        if string == "b"
            || string == "i"
            || string == "u"
            || string == "s"
            || string == "code"
            || string == "center"
            || string == "right"
            || string == "fill"
            || string == "indent"
            || string == "url"
            || string == "image"
            || string == "cell"
            || string.contains("url=")
            || string.contains("img=")
            || string.contains("font=")
            || string.contains("color=")
            || string.contains("table=")
        {
            new_string = "".to_string();
        }
    }

    if escape_special_chars {
        new_string = new_string
            .replace("`", "")
            .replace("~", "")
            .replace("!", "")
            .replace("@", "")
            .replace("#", "")
            .replace("$", "")
            .replace("%", "")
            .replace("^", "")
            .replace("&", "")
            .replace("*", "")
            .replace("(", "")
            .replace(")", "")
            .replace("-", "")
            .replace("+", "")
            .replace("_", "")
            .replace("{", "")
            .replace("}", "")
            .replace("\\", "")
            .replace("|", "");
    }

    new_string.trim().to_string()
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

impl Default for MOTD {
    fn default() -> Self {
        MOTD {
            message :  "[center]".to_string() +
            "[font=" + NEARBY_SHOUT_FONT + "][color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://github.com/starwolves/space\"}]Space Frontiers[/url][/color][/font]\n" +
            "Welcome to the official test server of Space Frontiers. (v" + VERSION + ")\n\n" +
            "You are about to board The Bullseye, a research & development ship.\n\n" +
            "The Space Frontiers community is thrilled to have you here, you are invited to connect with our new gaming community through our social platforms!\n" + 
            "[font=" + NEARBY_SHOUT_FONT + "][color=" + COMMUNITY_HREF_COLOR + "][url={\"type\": \"href\",\"data\":\"https://github.com/starwolves/space\"}]Github[/url][/color][/font]\n" +
            "[/center]",
        }
    }
}

pub struct MOTD {
    pub message: String,
}

pub const ATMOSPHERICS_TEXT_COLOR: &str = "#1797ff";
pub const ENGINEERING_TEXT_COLOR: &str = "#ff992b";
