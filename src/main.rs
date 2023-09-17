use wfc::wfc::{
    parsing::{post_processing::merge, text_parse::generate_rules},
    wfc::{algorithm::iterate, rules::generate_wfc_vector},
};

static A_FAIRY_SONG_SHAKESPEARE: &str = "Over hill, over dale,\nThorough bush, thorough brier,\nOver park, over pale,\nThorough flood, thorough fire!\nI do wander everywhere,\nSwifter than the moon's sphere;\nAnd I serve the Fairy Queen,\nTo dew her orbs upon the green;\nThe cowslips tall her pensioners be;\nIn their gold coats spots you see;\nThose be rubies, fairy favours;\nIn those freckles live their savours;\nI must go seek some dewdrops here,\nAnd hang a pearl in every cowslip's ear.";

fn main() {
    let rules = generate_rules(A_FAIRY_SONG_SHAKESPEARE.to_string());
    let vector = generate_wfc_vector(&rules, 20);

    let result = iterate(vector, &rules);

    match result {
        Ok(v) => print!("{}", merge(v)),
        Err(e) => panic!("{}", e),
    }
}
