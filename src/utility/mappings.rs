#![allow(clippy::trivial_regex)]
use crate::structures::Word;
use rand::prelude::*;
use regex::Regex;

const RE_ERROR_MESSAGE: &str = "Failed to build regular expression.";

lazy_static! {
    static ref O_TO_OWO: Regex = Regex::new(r"o").expect(RE_ERROR_MESSAGE);
    static ref EW_TO_UWU: Regex = Regex::new(r"ew").expect(RE_ERROR_MESSAGE);
    static ref HEY_TO_HAY: Regex = Regex::new(r"([Hh])ey").expect(RE_ERROR_MESSAGE);
    static ref DEAD_TO_DED_UPPER: Regex = Regex::new(r"Dead").expect(RE_ERROR_MESSAGE);
    static ref DEAD_TO_DED_LOWER: Regex = Regex::new(r"dead").expect(RE_ERROR_MESSAGE);
    static ref N_VOWEL_T_TO_ND: Regex = Regex::new(r"n[aeiou]*t").expect(RE_ERROR_MESSAGE);
    static ref READ_TO_WEAD_UPPER: Regex = Regex::new(r"Read").expect(RE_ERROR_MESSAGE);
    static ref READ_TO_WEAD_LOWER: Regex = Regex::new(r"read").expect(RE_ERROR_MESSAGE);
    static ref BRACKETS_TO_STARTRAILS_FORE: Regex = Regex::new(r"[({<]").expect(RE_ERROR_MESSAGE);
    static ref BRACKETS_TO_STARTRAILS_REAR: Regex = Regex::new(r"[)}>]").expect(RE_ERROR_MESSAGE);
    //static ref PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_FIRST: Regex = Regex::new(r"[.,](?![0-9])").expect(RE_ERROR_MESSAGE);
    static ref PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_FIRST: Regex = Regex::new(r"[.,]").expect(RE_ERROR_MESSAGE);
    static ref PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_SECOND: Regex = Regex::new(r"[!;]+").expect(RE_ERROR_MESSAGE);
    static ref THAT_TO_DAT_UPPER: Regex = Regex::new(r"That").expect(RE_ERROR_MESSAGE);
    static ref THAT_TO_DAT_LOWER: Regex = Regex::new(r"that").expect(RE_ERROR_MESSAGE);
    //static ref TH_TO_F_UPPER: Regex = Regex::new(r"TH(?!E)").expect(RE_ERROR_MESSAGE);
    static ref TH_TO_F_UPPER: Regex = Regex::new(r"TH").expect(RE_ERROR_MESSAGE);
    //static ref TH_TO_F_LOWER: Regex = Regex::new(r"[Tt]h(?![Ee])").expect(RE_ERROR_MESSAGE);
    static ref TH_TO_F_LOWER: Regex = Regex::new(r"[Tt]h").expect(RE_ERROR_MESSAGE);
    static ref LE_TO_WAL: Regex = Regex::new(r"le$").expect(RE_ERROR_MESSAGE);
    static ref VE_TO_WE_UPPER: Regex = Regex::new(r"Ve").expect(RE_ERROR_MESSAGE);
    static ref VE_TO_WE_LOWER: Regex = Regex::new(r"ve").expect(RE_ERROR_MESSAGE);
    static ref RY_TO_WWY: Regex = Regex::new(r"ry").expect(RE_ERROR_MESSAGE);
    static ref RORL_TO_W_UPPER: Regex = Regex::new(r"(?:R|L)").expect(RE_ERROR_MESSAGE);
    static ref RORL_TO_W_LOWER: Regex = Regex::new(r"(?:r|l)").expect(RE_ERROR_MESSAGE);
    static ref LL_TO_WW: Regex = Regex::new(r"ll").expect(RE_ERROR_MESSAGE);
    static ref VOWEL_OR_R_EXCEPT_O_L_TO_WL_UPPER: Regex = Regex::new(r"[AEIUR]([lL])$").expect(RE_ERROR_MESSAGE);
    static ref VOWEL_OR_R_EXCEPT_O_L_TO_WL_LOWER: Regex = Regex::new(r"[aeiur]l$").expect(RE_ERROR_MESSAGE);
    static ref OLD_TO_OWLD_UPPER: Regex = Regex::new(r"OLD").expect(RE_ERROR_MESSAGE);
    static ref OLD_TO_OWLD_LOWER: Regex = Regex::new(r"([Oo])ld").expect(RE_ERROR_MESSAGE);
    static ref OL_TO_OWL_UPPER: Regex = Regex::new(r"OL").expect(RE_ERROR_MESSAGE);
    static ref OL_TO_OWL_LOWER: Regex = Regex::new(r"([Oo])l").expect(RE_ERROR_MESSAGE);
    static ref LORR_O_TO_WO_UPPER: Regex = Regex::new(r"[LR]([oO])").expect(RE_ERROR_MESSAGE);
    static ref LORR_O_TO_WO_LOWER: Regex = Regex::new(r"[lr]o").expect(RE_ERROR_MESSAGE);
    static ref SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_UPPER: Regex = Regex::new(r"([BCDFGHJKMNPQSTXYZ])([oO])").expect(RE_ERROR_MESSAGE);
    static ref SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_LOWER: Regex = Regex::new(r"([bcdfghjkmnpqstxyz])o").expect(RE_ERROR_MESSAGE);
    static ref VORW_LE_TO_WAL: Regex = Regex::new(r"[vw]le").expect(RE_ERROR_MESSAGE);
    static ref FI_TO_FWI_UPPER: Regex = Regex::new(r"FI").expect(RE_ERROR_MESSAGE);
    static ref FI_TO_FWI_LOWER: Regex = Regex::new(r"([Ff])i").expect(RE_ERROR_MESSAGE);
    static ref VER_TO_WER: Regex = Regex::new(r"([Vv])er").expect(RE_ERROR_MESSAGE);
    static ref POI_TO_PWOI: Regex = Regex::new(r"([Pp])oi").expect(RE_ERROR_MESSAGE);
    static ref SPECIFIC_CONSONANTS_LE_TO_LETTER_AND_WAL: Regex = Regex::new(r"([DdFfGgHhJjPpQqRrSsTtXxYyZz])le$").expect(RE_ERROR_MESSAGE);
    static ref CONSONANT_R_TO_CONSONANT_W: Regex = Regex::new(r"([BbCcDdFfGgKkPpQqSsTtWwXxZz])r").expect(RE_ERROR_MESSAGE);
    static ref LY_TO_WY_UPPER: Regex = Regex::new(r"Ly").expect(RE_ERROR_MESSAGE);
    static ref LY_TO_WY_LOWER: Regex = Regex::new(r"ly").expect(RE_ERROR_MESSAGE);
    static ref PLE_TO_PWE: Regex = Regex::new(r"([Pp])le").expect(RE_ERROR_MESSAGE);
    static ref NR_TO_NW_UPPER: Regex = Regex::new(r"NR").expect(RE_ERROR_MESSAGE);
    static ref NR_TO_NW_LOWER: Regex = Regex::new(r"([Nn])r").expect(RE_ERROR_MESSAGE);
    static ref MEM_TO_MWEM_UPPER: Regex = Regex::new(r"Mem").expect(RE_ERROR_MESSAGE);
    static ref MEM_TO_MWEM_LOWER: Regex = Regex::new(r"mem").expect(RE_ERROR_MESSAGE);
    static ref NYWO_TO_NYO: Regex = Regex::new("([Nn])ywo").expect(RE_ERROR_MESSAGE);
    static ref FUC_TO_FWUC: Regex = Regex::new(r"([Ff])uc").expect(RE_ERROR_MESSAGE);
    static ref MOM_TO_MWOM: Regex = Regex::new(r"([Mm])om").expect(RE_ERROR_MESSAGE);
    static ref ME_TO_MWE_UPPER: Regex = Regex::new(r"^Me$").expect(RE_ERROR_MESSAGE);
    static ref ME_TO_MWE_LOWER: Regex = Regex::new(r"^me$").expect(RE_ERROR_MESSAGE);
    static ref N_VOWEL_TO_NY_FIRST: Regex = Regex::new(r"n([aeiou])").expect(RE_ERROR_MESSAGE);
    static ref N_VOWEL_TO_NY_SECOND: Regex = Regex::new(r"N([aeiou])").expect(RE_ERROR_MESSAGE);
    static ref N_VOWEL_TO_NY_THIRD: Regex = Regex::new(r"N([AEIOU])").expect(RE_ERROR_MESSAGE);
    static ref OVE_TO_UV_UPPER: Regex = Regex::new(r"OVE").expect(RE_ERROR_MESSAGE);
    static ref OVE_TO_UV_LOWER: Regex = Regex::new(r"ove").expect(RE_ERROR_MESSAGE);
    static ref HAHA_TO_HEHE_XD: Regex = Regex::new(r"\b(ha|hah|heh|hehe)+\b").expect(RE_ERROR_MESSAGE);
    static ref THE_TO_TEH: Regex = Regex::new(r"\b([Tt])he\b").expect(RE_ERROR_MESSAGE);
    static ref YOU_TO_U_UPPER: Regex = Regex::new(r"\bYou\b").expect(RE_ERROR_MESSAGE);
    static ref YOU_TO_U_LOWER: Regex = Regex::new(r"\byou\b").expect(RE_ERROR_MESSAGE);
    static ref TIME_TO_TIM: Regex = Regex::new(r"\b([Tt])ime\b").expect(RE_ERROR_MESSAGE);
    static ref OVER_TO_OWOR: Regex = Regex::new(r"([Oo])ver").expect(RE_ERROR_MESSAGE);
    static ref WORSE_TO_WOSE: Regex = Regex::new(r"([Ww])orse").expect(RE_ERROR_MESSAGE);
    static ref GREAT_TO_GWATE: Regex = Regex::new(r"([Gg])reat").expect(RE_ERROR_MESSAGE);
    static ref AVIAT_TO_AWIAT: Regex = Regex::new(r"([Aa])viat").expect(RE_ERROR_MESSAGE);
    static ref DEDICAT_TO_DEDITAT: Regex = Regex::new(r"([Dd])edicat").expect(RE_ERROR_MESSAGE);
    static ref REMEMBER_TO_REMBER: Regex = Regex::new(r"([Rr])emember").expect(RE_ERROR_MESSAGE);
    static ref WHEN_TO_WEN: Regex = Regex::new(r"([Ww])hen").expect(RE_ERROR_MESSAGE);
    static ref FRIGHTENED_TO_FRIGTEN: Regex = Regex::new(r"([Ff])righten(ed)*").expect(RE_ERROR_MESSAGE);
    static ref MEME_TO_MEM_FIRST: Regex = Regex::new(r"Meme").expect(RE_ERROR_MESSAGE);
    static ref MEME_TO_MEM_SECOND: Regex = Regex::new(r"Mem").expect(RE_ERROR_MESSAGE);
    static ref FEEL_TO_FELL: Regex = Regex::new(r"^([Ff])eel$").expect(RE_ERROR_MESSAGE);
}

pub(crate) const FACES: [&str; 29] = [
    "(・`ω´・)",
    ";;w;;",
    "owo",
    "UwU",
    ">w<",
    "^w^",
    "(* ^ ω ^)",
    "(⌒ω⌒)",
    "ヽ(*・ω・)ﾉ",
    "(o´∀`o)",
    "(o･ω･o)",
    "＼(＾▽＾)／",
    "(*^ω^)",
    "(◕‿◕✿)",
    "(◕ᴥ◕)",
    "ʕ•ᴥ•ʔ",
    "ʕ￫ᴥ￩ʔ",
    "(*^.^*)",
    "(｡♥‿♥｡)",
    "OwO",
    "uwu",
    "uvu",
    "UvU",
    "(*￣з￣)",
    "(つ✧ω✧)つ",
    "(/ =ω=)/",
    "(╯°□°）╯︵ ┻━┻",
    "┬─┬ ノ( ゜-゜ノ)",
    "¯\\_(ツ)_/¯",
];

pub(crate) fn map_o_to_owo(input: Word) -> Word {
    let replacement = if thread_rng().gen_range(0..2) > 0 {
        "owo"
    } else {
        "o"
    };
    input.replace(&*O_TO_OWO, replacement, false)
}

pub(crate) fn map_ew_to_uwu(input: Word) -> Word {
    input.replace(&*EW_TO_UWU, "uwu", false)
}

pub(crate) fn map_hey_to_hay(input: Word) -> Word {
    input.replace(&*HEY_TO_HAY, "${1}ay", false)
}

pub(crate) fn map_dead_to_ded(input: Word) -> Word {
    input
        .replace(&*DEAD_TO_DED_UPPER, "Ded", false)
        .replace(&*DEAD_TO_DED_LOWER, "ded", false)
}

pub(crate) fn map_n_vowel_t_to_nd(input: Word) -> Word {
    input.replace(&*N_VOWEL_T_TO_ND, "nd", false)
}

pub(crate) fn map_read_to_wead(input: Word) -> Word {
    input
        .replace(&*READ_TO_WEAD_UPPER, "Wead", false)
        .replace(&*READ_TO_WEAD_LOWER, "wead", false)
}

pub(crate) fn map_brackets_to_star_trails(input: Word) -> Word {
    input
        .replace(&*BRACKETS_TO_STARTRAILS_FORE, "｡･:*:･ﾟ★,｡･:*:･ﾟ☆", false)
        .replace(&*BRACKETS_TO_STARTRAILS_REAR, "☆ﾟ･:*:･｡,★ﾟ･:*:･｡", false)
}

pub(crate) fn map_period_comma_exclamation_semicolon_to_kaomojis(input: Word) -> Word {
    input
        .replace_with_func_single(
            &*PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_FIRST,
            || {
                let mut rng = thread_rng();
                " ".to_string()
                    + FACES
                        .choose(&mut rng)
                        .copied()
                        .expect("Failed to get a random face.")
            },
            false,
        )
        .replace_with_func_single(
            &*PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_SECOND,
            || {
                let mut rng = thread_rng();
                " ".to_string()
                    + FACES
                        .choose(&mut rng)
                        .copied()
                        .expect("Failed to get a random face.")
            },
            false,
        )
}

pub(crate) fn map_that_to_dat(input: Word) -> Word {
    input
        .replace(&*THAT_TO_DAT_LOWER, "dat", false)
        .replace(&*THAT_TO_DAT_UPPER, "Dat", false)
}

pub(crate) fn map_th_to_f(input: Word) -> Word {
    input
        .replace(&*TH_TO_F_LOWER, "f", false)
        .replace(&*TH_TO_F_UPPER, "F", false)
}

pub(crate) fn map_le_to_wal(input: Word) -> Word {
    input.replace(&*LE_TO_WAL, "wal", false)
}

pub(crate) fn map_ve_to_we(input: Word) -> Word {
    input
        .replace(&*VE_TO_WE_LOWER, "we", false)
        .replace(&*VE_TO_WE_UPPER, "We", false)
}

pub(crate) fn map_ry_to_wwy(input: Word) -> Word {
    input.replace(&*RY_TO_WWY, "wwy", false)
}

pub(crate) fn map_r_or_l_to_w(input: Word) -> Word {
    input
        .replace(&*RORL_TO_W_LOWER, "w", false)
        .replace(&*RORL_TO_W_UPPER, "W", false)
}

pub(crate) fn map_ll_to_ww(input: Word) -> Word {
    input.replace(&*LL_TO_WW, "ww", false)
}

pub(crate) fn map_vowel_or_r_except_o_l_to_wl(input: Word) -> Word {
    input
        .replace(&*VOWEL_OR_R_EXCEPT_O_L_TO_WL_LOWER, "wl", false)
        .replace(&*VOWEL_OR_R_EXCEPT_O_L_TO_WL_UPPER, "W${1}", false)
}

pub(crate) fn map_old_to_owld(input: Word) -> Word {
    input
        .replace(&*OLD_TO_OWLD_LOWER, "${1}wld", false)
        .replace(&*OLD_TO_OWLD_UPPER, "OWLD", false)
}

pub(crate) fn map_ol_to_owl(input: Word) -> Word {
    input
        .replace(&*OL_TO_OWL_LOWER, "${1}wl", false)
        .replace(&*OL_TO_OWL_UPPER, "OWL", false)
}

pub(crate) fn map_l_or_r_o_to_wo(input: Word) -> Word {
    input
        .replace(&*LORR_O_TO_WO_LOWER, "wo", false)
        .replace(&*LORR_O_TO_WO_UPPER, "W${1}", false)
}

pub(crate) fn map_specific_consonants_o_to_letter_and_wo(input: Word) -> Word {
    input
        .replace(
            &*SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_LOWER,
            "${1}wo",
            false,
        )
        .replace_with_func_multiple(
            &*SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_UPPER,
            |s1, s2| {
                let mut msg = s1.to_string();
                msg += if s2.to_uppercase() == *s2 { "W" } else { "w" };
                msg += s2;
                msg
            },
            false,
        )
}

pub(crate) fn map_v_or_w_le_to_wal(input: Word) -> Word {
    input.replace(&*LE_TO_WAL, "wal", false)
}

pub(crate) fn map_fi_to_fwi(input: Word) -> Word {
    input
        .replace(&*FI_TO_FWI_LOWER, "${1}wi", false)
        .replace(&*FI_TO_FWI_UPPER, "FWI", false)
}

pub(crate) fn map_ver_to_wer(input: Word) -> Word {
    input.replace(&*VER_TO_WER, "wer", false)
}

pub(crate) fn map_poi_to_pwoi(input: Word) -> Word {
    input.replace(&*POI_TO_PWOI, "${1}woi", false)
}

pub(crate) fn map_specific_consonants_le_to_letter_and_wal(input: Word) -> Word {
    input.replace(&*SPECIFIC_CONSONANTS_LE_TO_LETTER_AND_WAL, "${1}wal", false)
}

pub(crate) fn map_consonant_r_to_consonant_w(input: Word) -> Word {
    input.replace(&*CONSONANT_R_TO_CONSONANT_W, "${1}w", false)
}

pub(crate) fn map_ly_to_wy(input: Word) -> Word {
    input
        .replace(&*LY_TO_WY_LOWER, "wy", false)
        .replace(&*LY_TO_WY_UPPER, "Wy", false)
}

pub(crate) fn map_ple_to_pwe(input: Word) -> Word {
    input.replace(&*PLE_TO_PWE, "${1}we", false)
}

pub(crate) fn map_nr_to_nw(input: Word) -> Word {
    input
        .replace(&*NR_TO_NW_LOWER, "${1}w", false)
        .replace(&*NR_TO_NW_UPPER, "NW", false)
}

pub(crate) fn map_mem_to_mwem(input: Word) -> Word {
    input
        .replace(&*MEM_TO_MWEM_UPPER, "mwem", false)
        .replace(&*MEM_TO_MWEM_LOWER, "Mwem", false)
}

pub(crate) fn unmap_nywo_to_nyo(input: Word) -> Word {
    input.replace(&*NYWO_TO_NYO, "${1}yo", false)
}

pub(crate) fn map_fuc_to_fwuc(input: Word) -> Word {
    input.replace(&*FUC_TO_FWUC, "${1}wuc", false)
}

pub(crate) fn map_mom_to_mwom(input: Word) -> Word {
    input.replace(&*MOM_TO_MWOM, "${1}wom", false)
}

pub(crate) fn map_me_to_mwe(input: Word) -> Word {
    input
        .replace(&*ME_TO_MWE_UPPER, "Mwe", false)
        .replace(&*ME_TO_MWE_LOWER, "mwe", false)
}

pub(crate) fn map_n_vowel_to_ny(input: Word) -> Word {
    input
        .replace(&*N_VOWEL_TO_NY_FIRST, "ny${1}", false)
        .replace(&*N_VOWEL_TO_NY_SECOND, "Ny${1}", false)
        .replace(&*N_VOWEL_TO_NY_THIRD, "NY${1}", false)
}

pub(crate) fn map_ove_to_uv(input: Word) -> Word {
    input
        .replace(&*OVE_TO_UV_LOWER, "uv", false)
        .replace(&*OVE_TO_UV_UPPER, "UV", false)
}

pub(crate) fn map_haha_to_hehe_xd(input: Word) -> Word {
    input.replace(&*HAHA_TO_HEHE_XD, "hehe xD", false)
}

pub(crate) fn map_the_to_teh(input: Word) -> Word {
    input.replace(&*THE_TO_TEH, "${1}eh", false)
}

pub(crate) fn map_you_to_u(input: Word) -> Word {
    input
        .replace(&*YOU_TO_U_UPPER, "U", false)
        .replace(&*YOU_TO_U_LOWER, "u", false)
}

pub(crate) fn map_time_to_tim(input: Word) -> Word {
    input.replace(&*TIME_TO_TIM, "${1}im", false)
}

pub(crate) fn map_over_to_owor(input: Word) -> Word {
    input.replace(&*OVER_TO_OWOR, "${1}wor", false)
}

pub(crate) fn map_worse_to_wose(input: Word) -> Word {
    input.replace(&*WORSE_TO_WOSE, "${1}ose", false)
}

pub(crate) fn map_great_to_gwate(input: Word) -> Word {
    input.replace(&*GREAT_TO_GWATE, "${1}wate", false)
}

pub(crate) fn map_aviat_to_awiat(input: Word) -> Word {
    input.replace(&*AVIAT_TO_AWIAT, "${1}wiat", false)
}

pub(crate) fn map_dedicat_to_deditat(input: Word) -> Word {
    input.replace(&*DEDICAT_TO_DEDITAT, "${1}editat", false)
}

pub(crate) fn map_remember_to_rember(input: Word) -> Word {
    input.replace(&*REMEMBER_TO_REMBER, "${1}ember", false)
}

pub(crate) fn map_when_to_wen(input: Word) -> Word {
    input.replace(&*WHEN_TO_WEN, "${1}en", false)
}

pub(crate) fn map_frightened_to_frigten(input: Word) -> Word {
    input.replace(&*FRIGHTENED_TO_FRIGTEN, "${1}rigten", false)
}

pub(crate) fn map_meme_to_mem(input: Word) -> Word {
    input
        .replace(&*MEME_TO_MEM_FIRST, "mem", false)
        .replace(&*MEME_TO_MEM_SECOND, "Mem", false)
}

pub(crate) fn map_feel_to_fell(input: Word) -> Word {
    input.replace(&*FEEL_TO_FELL, "${1}ell", false)
}
