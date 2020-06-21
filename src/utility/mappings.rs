use regex::Regex;
use crate::structures::word::Word;
use rand::{thread_rng, Rng};

lazy_static! {
    static ref O_TO_OWO: Regex = Regex::new(r"o").unwrap();
    static ref EW_TO_UWU: Regex = Regex::new(r"ew").unwrap();
    static ref HEY_TO_HAY: Regex = Regex::new(r"([Hh])ey").unwrap();
    static ref DEAD_TO_DED_UPPER: Regex = Regex::new(r"Dead").unwrap();
    static ref DEAD_TO_DED_LOWER: Regex = Regex::new(r"dead").unwrap();
    static ref N_VOWEL_T_TO_ND: Regex = Regex::new(r"n[aeiou]*t").unwrap();
    static ref READ_TO_WEAD_UPPER: Regex = Regex::new(r"Read").unwrap();
    static ref READ_TO_WEAD_LOWER: Regex = Regex::new(r"read").unwrap();
    static ref BRACKETS_TO_STARTRAILS_FORE: Regex = Regex::new(r"[({<]").unwrap();
    static ref BRACKETS_TO_STARTRAILS_REAR: Regex = Regex::new(r"[)}>]").unwrap();
    //static ref PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_FIRST: Regex = Regex::new(r"[.,](?![0-9])").unwrap();
    static ref PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_FIRST: Regex = Regex::new(r"[.,]").unwrap();
    static ref PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_SECOND: Regex = Regex::new(r"[!;]+").unwrap();
    static ref THAT_TO_DAT_UPPER: Regex = Regex::new(r"That").unwrap();
    static ref THAT_TO_DAT_LOWER: Regex = Regex::new(r"that").unwrap();
    //static ref TH_TO_F_UPPER: Regex = Regex::new(r"TH(?!E)").unwrap();
    static ref TH_TO_F_UPPER: Regex = Regex::new(r"TH").unwrap();
    //static ref TH_TO_F_LOWER: Regex = Regex::new(r"[Tt]h(?![Ee])").unwrap();
    static ref TH_TO_F_LOWER: Regex = Regex::new(r"[Tt]h").unwrap();
    static ref LE_TO_WAL: Regex = Regex::new(r"le$").unwrap();
    static ref VE_TO_WE_UPPER: Regex = Regex::new(r"Ve").unwrap();
    static ref VE_TO_WE_LOWER: Regex = Regex::new(r"ve").unwrap();
    static ref RY_TO_WWY: Regex = Regex::new(r"ry").unwrap();
    static ref RORL_TO_W_UPPER: Regex = Regex::new(r"(?:R|L)").unwrap();
    static ref RORL_TO_W_LOWER: Regex = Regex::new(r"(?:r|l)").unwrap();
    static ref LL_TO_WW: Regex = Regex::new(r"ll").unwrap();
    static ref VOWEL_OR_R_EXCEPT_O_L_TO_WL_UPPER: Regex = Regex::new(r"[AEIUR]([lL])$").unwrap();
    static ref VOWEL_OR_R_EXCEPT_O_L_TO_WL_LOWER: Regex = Regex::new(r"[aeiur]l$").unwrap();
    static ref OLD_TO_OWLD_UPPER: Regex = Regex::new(r"OLD").unwrap();
    static ref OLD_TO_OWLD_LOWER: Regex = Regex::new(r"([Oo])ld").unwrap();
    static ref OL_TO_OWL_UPPER: Regex = Regex::new(r"OL").unwrap();
    static ref OL_TO_OWL_LOWER: Regex = Regex::new(r"([Oo])l").unwrap();
    static ref LORR_O_TO_WO_UPPER: Regex = Regex::new(r"[LR]([oO])").unwrap();
    static ref LORR_O_TO_WO_LOWER: Regex = Regex::new(r"[lr]o").unwrap();
    static ref SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_UPPER: Regex = Regex::new(r"([BCDFGHJKMNPQSTXYZ])([oO])").unwrap();
    static ref SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_LOWER: Regex = Regex::new(r"([bcdfghjkmnpqstxyz])o").unwrap();
    static ref VORW_LE_TO_WAL: Regex = Regex::new(r"[vw]le").unwrap();
    static ref FI_TO_FWI_UPPER: Regex = Regex::new(r"FI").unwrap();
    static ref FI_TO_FWI_LOWER: Regex = Regex::new(r"([Ff])i").unwrap();
    static ref VER_TO_WER: Regex = Regex::new(r"([Vv])er").unwrap();
    static ref POI_TO_PWOI: Regex = Regex::new(r"([Pp])oi").unwrap();
    static ref SPECIFIC_CONSONANTS_LE_TO_LETTER_AND_WAL: Regex = Regex::new(r"([DdFfGgHhJjPpQqRrSsTtXxYyZz])le$").unwrap();
    static ref CONSONANT_R_TO_CONSONANT_W: Regex = Regex::new(r"([BbCcDdFfGgKkPpQqSsTtWwXxZz])r").unwrap();
    static ref LY_TO_WY_UPPER: Regex = Regex::new(r"Ly").unwrap();
    static ref LY_TO_WY_LOWER: Regex = Regex::new(r"ly").unwrap();
    static ref PLE_TO_PWE: Regex = Regex::new(r"([Pp])le").unwrap();
    static ref NR_TO_NW_UPPER: Regex = Regex::new(r"NR").unwrap();
    static ref NR_TO_NW_LOWER: Regex = Regex::new(r"nr").unwrap();
    static ref FUC_TO_FWUC: Regex = Regex::new(r"([Ff])uc").unwrap();
    static ref MOM_TO_MWOM: Regex = Regex::new(r"([Mm])om").unwrap();
    static ref ME_TO_MWE: Regex = Regex::new(r"([Mm])e").unwrap();
    static ref N_VOWEL_TO_NY_FIRST: Regex = Regex::new(r"n([aeiou])").unwrap();
    static ref N_VOWEL_TO_NY_SECOND: Regex = Regex::new(r"N([aeiou])").unwrap();
    static ref N_VOWEL_TO_NY_THIRD: Regex = Regex::new(r"N([AEIOU])").unwrap();
    static ref OVE_TO_UV_UPPER: Regex = Regex::new(r"OVE").unwrap();
    static ref OVE_TO_UV_LOWER: Regex = Regex::new(r"ove").unwrap();
    static ref HAHA_TO_HEHE_XD: Regex = Regex::new(r"\b(ha|hah|heh|hehe)+\b").unwrap();
    static ref THE_TO_TEH: Regex = Regex::new(r"\b([Tt])he\b").unwrap();
    static ref YOU_TO_U_UPPER: Regex = Regex::new(r"\bYou\b").unwrap();
    static ref YOU_TO_U_LOWER: Regex = Regex::new(r"\byou\b").unwrap();
    static ref TIME_TO_TIM: Regex = Regex::new(r"\b([Tt])ime\b").unwrap();
    static ref OVER_TO_OWOR: Regex = Regex::new(r"([Oo])ver").unwrap();
    static ref WORSE_TO_WOSE: Regex = Regex::new(r"([Ww])orse").unwrap();
}

pub const FACES: [&'static str; 12] = [
    "(・`ω´・)", ";;w;;", "owo", "UwU", ">w<", "^w^", "(* ^ ω ^)",
    "(⌒ω⌒)", "ヽ(*・ω・)ﾉ", "(o´∀`o)", "(o･ω･o)", "＼(＾▽＾)／"
];

pub fn map_o_to_owo(input: &mut Word) -> &Word {
    let replacement = if thread_rng().gen_range(0, 2) > 0 {
        "owo"
    }
    else {
        "o"
    };
    let reg: &Regex = &*O_TO_OWO;
    input.replace(reg, replacement, false)
}

pub fn map_ew_to_uwu(input: &mut Word) -> &Word {
    let reg: &Regex = &*EW_TO_UWU;
    input.replace(reg, "uwu", false)
}

pub fn map_hey_to_hay(input: &mut Word) -> &Word {
    let reg: &Regex = &*HEY_TO_HAY;
    input.replace(reg, "${1}ay", false)
}

pub fn map_dead_to_ded(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*DEAD_TO_DED_UPPER;
    let reg_lower: &Regex = &*DEAD_TO_DED_LOWER;
    input.replace(reg_upper, "Ded", false);
    input.replace(reg_lower, "ded", false);
    input
}

pub fn map_n_vowel_t_to_nd(input: &mut Word) -> &Word {
    let reg: &Regex = &*N_VOWEL_T_TO_ND;
    input.replace(reg, "nd", false)
}

pub fn map_read_to_wead(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*READ_TO_WEAD_UPPER;
    let reg_lower: &Regex = &*READ_TO_WEAD_LOWER;
    input.replace(reg_upper, "Wead", false);
    input.replace(reg_lower, "wead", false);
    input
}

pub fn map_brackets_to_star_trails(input: &mut Word) -> &Word {
    let reg_fore: &Regex = &*BRACKETS_TO_STARTRAILS_FORE;
    let reg_rear: &Regex = &*BRACKETS_TO_STARTRAILS_REAR;
    input.replace(reg_fore, "｡･:*:･ﾟ★,｡･:*:･ﾟ☆", false);
    input.replace(reg_rear, "☆ﾟ･:*:･｡,★ﾟ･:*:･｡", false);
    input
}

pub fn map_period_comma_exclamation_semicolon_to_kaomojis(input: &mut Word) -> &Word {
    let mut num = thread_rng().gen_range(0.0, 1.0) * (FACES.len() as f64);
    let mut index = num.floor() as usize;
    let reg_first: &Regex = &*PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_FIRST;
    let reg_second: &Regex = &*PERIOD_COMMA_EXCLAMATION_SEMICOLON_TO_KAOMOJIS_SECOND;
    input.replace_with_func_single(reg_first, || " ".to_string() + FACES[index], false);

    num = thread_rng().gen_range(0.0, 1.0) * (FACES.len() as f64);
    index = num.floor() as usize;
    input.replace_with_func_single(reg_second, || " ".to_string() + FACES[index], false);
    input
}

pub fn map_that_to_dat(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*THAT_TO_DAT_UPPER;
    let reg_lower: &Regex = &*THAT_TO_DAT_LOWER;
    input.replace(reg_lower, "dat", false);
    input.replace(reg_upper, "Dat", false);
    input
}

pub fn map_th_to_f(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*TH_TO_F_UPPER;
    let reg_lower: &Regex = &*TH_TO_F_LOWER;
    input.replace(reg_lower, "f", false);
    input.replace(reg_upper, "F", false);
    input
}

pub fn map_le_to_wal(input: &mut Word) -> &Word {
    let reg: &Regex = &*LE_TO_WAL;
    input.replace(reg, "wal", false)
}

pub fn map_ve_to_we(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*VE_TO_WE_UPPER;
    let reg_lower: &Regex = &*VE_TO_WE_LOWER;
    input.replace(reg_lower, "we", false);
    input.replace(reg_upper, "We", false);
    input
}

pub fn map_ry_to_wwy(input: &mut Word) -> &Word {
    let reg: &Regex = &*RY_TO_WWY;
    input.replace(reg, "wwy", false)
}

pub fn map_r_or_l_to_w(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*RORL_TO_W_UPPER;
    let reg_lower: &Regex = &*RORL_TO_W_LOWER;
    input.replace(reg_lower, "w", false);
    input.replace(reg_upper, "W", false);
    input
}

pub fn map_ll_to_ww(input: &mut Word) -> &Word {
    let reg: &Regex = &*LL_TO_WW;
    input.replace(reg, "ww", false)
}

pub fn map_vowel_or_r_except_o_l_to_wl(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*VOWEL_OR_R_EXCEPT_O_L_TO_WL_UPPER;
    let reg_lower: &Regex = &*VOWEL_OR_R_EXCEPT_O_L_TO_WL_LOWER;
    input.replace(reg_lower, "wl", false);
    input.replace(reg_upper, "W${1}", false);
    input
}

pub fn map_old_to_owld(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*OLD_TO_OWLD_UPPER;
    let reg_lower: &Regex = &*OLD_TO_OWLD_LOWER;
    input.replace(reg_lower, "${1}wld", false);
    input.replace(reg_upper, "OWLD", false);
    input
}

pub fn map_ol_to_owl(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*OL_TO_OWL_UPPER;
    let reg_lower: &Regex = &*OL_TO_OWL_LOWER;
    input.replace(reg_lower, "${1}wl", false);
    input.replace(reg_upper, "OWL", false);
    input
}

pub fn map_l_or_r_o_to_wo(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*LORR_O_TO_WO_UPPER;
    let reg_lower: &Regex = &*LORR_O_TO_WO_LOWER;
    input.replace(reg_lower, "wo", false);
    input.replace(reg_upper, "W${1}", false);
    input
}

pub fn map_specific_consonants_o_to_letter_and_wo(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_UPPER;
    let reg_lower: &Regex = &*SPECIFIC_CONSONANTS_O_TO_LETTER_AND_WO_LOWER;
    input.replace(reg_lower, "${1}wo", false);
    input.replace_with_func_multiple(reg_upper, |s1, s2| {
        let mut msg = s1.to_string();
        msg += if s2.to_uppercase() == s2.to_string() {
            "W"
        }
        else {
            "w"
        };
        msg += s2;
        msg
    }, false)
}

pub fn map_v_or_w_le_to_wal(input: &mut Word) -> &Word {
    let reg: &Regex = &*LE_TO_WAL;
    input.replace(reg, "wal", false)
}

pub fn map_fi_to_fwi(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*FI_TO_FWI_UPPER;
    let reg_lower: &Regex = &*FI_TO_FWI_LOWER;
    input.replace(reg_lower, "${1}wi", false);
    input.replace(reg_upper, "FWI", false);
    input
}

pub fn map_ver_to_wer(input: &mut Word) -> &Word {
    let reg: &Regex = &*VER_TO_WER;
    input.replace(reg, "wer", false)
}

pub fn map_poi_to_pwoi(input: &mut Word) -> &Word {
    let reg: &Regex = &*POI_TO_PWOI;
    input.replace(reg, "${1}woi", false)
}

pub fn map_specific_consonants_le_to_letter_and_wal(input: &mut Word) -> &Word {
    let reg: &Regex = &*SPECIFIC_CONSONANTS_LE_TO_LETTER_AND_WAL;
    input.replace(reg, "${1}wal", false)
}

pub fn map_consonant_r_to_consonant_w(input: &mut Word) -> &Word {
    let reg: &Regex = &*CONSONANT_R_TO_CONSONANT_W;
    input.replace(reg, "${1}w", false)
}

pub fn map_ly_to_wy(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*LY_TO_WY_UPPER;
    let reg_lower: &Regex = &*LY_TO_WY_LOWER;
    input.replace(reg_lower, "wy", false);
    input.replace(reg_upper, "Wy", false);
    input
}

pub fn map_ple_to_pwe(input: &mut Word) -> &Word {
    let reg: &Regex = &*PLE_TO_PWE;
    input.replace(reg, "1we", false)
}

pub fn map_nr_to_nw(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*NR_TO_NW_UPPER;
    let reg_lower: &Regex = &*NR_TO_NW_LOWER;
    input.replace(reg_lower, "nw", false);
    input.replace(reg_upper, "NW", false);
    input
}

pub fn map_fuc_to_fwuc(input: &mut Word) -> &Word {
    let reg: &Regex = &*FUC_TO_FWUC;
    input.replace(reg, "${1}wuc", false)
}

pub fn map_mom_to_mwom(input: &mut Word) -> &Word {
    let reg: &Regex = &*MOM_TO_MWOM;
    input.replace(reg, "${1}wom", false)
}

pub fn map_me_to_mwe(input: &mut Word) -> &Word {
    let reg: &Regex = &*ME_TO_MWE;
    input.replace(reg, "${1}we", false)
}

pub fn map_n_vowel_to_ny(input: &mut Word) -> &Word {
    let reg_first: &Regex = &*N_VOWEL_TO_NY_FIRST;
    let reg_second: &Regex = &*N_VOWEL_TO_NY_SECOND;
    let reg_third: &Regex = &*N_VOWEL_TO_NY_THIRD;
    input.replace(reg_first, "ny${1}", false);
    input.replace(reg_second, "Ny${1}", false);
    input.replace(reg_third, "NY${1}", false);
    input
}

pub fn map_ove_to_uv(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*OVE_TO_UV_UPPER;
    let reg_lower: &Regex = &*OVE_TO_UV_LOWER;
    input.replace(reg_lower, "uv", false);
    input.replace(reg_upper, "UV", false);
    input
}

pub fn map_haha_to_hehe_xd(input: &mut Word) -> &Word {
    let reg: &Regex = &*HAHA_TO_HEHE_XD;
    input.replace(reg, "hehe xD", false)
}

pub fn map_the_to_teh(input: &mut Word) -> &Word {
    let reg: &Regex = &*THE_TO_TEH;
    input.replace(reg, "${1}eh", false)
}

pub fn map_you_to_u(input: &mut Word) -> &Word {
    let reg_upper: &Regex = &*YOU_TO_U_UPPER;
    let reg_lower: &Regex = &*YOU_TO_U_LOWER;
    input.replace(reg_upper, "U", false);
    input.replace(reg_lower, "u", false);
    input
}

pub fn map_time_to_tim(input: &mut Word) -> &Word {
    let reg: &Regex = &*TIME_TO_TIM;
    input.replace(reg, "${1}im", false)
}

pub fn map_over_to_owor(input: &mut Word) -> &Word {
    let reg: &Regex = &*OVER_TO_OWOR;
    input.replace(reg, "${1}wor", false)
}

pub fn map_worse_to_wose(input: &mut Word) -> &Word {
    let reg: &Regex = &*WORSE_TO_WOSE;
    input.replace(reg, "${1}ose", false)
}