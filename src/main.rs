mod wfc {
    pub mod wfc {
        pub mod algorithm;
        pub mod post_processing;
        pub mod rules;
    }
}

use std::collections::HashMap;

use wfc::wfc::{
    algorithm::iterate,
    rules::{generate_wfc_vector, Allowed, Rules, END, START},
};

use crate::wfc::wfc::post_processing::merge;

fn get_rules() -> Rules {
    let mut rules = HashMap::<String, Allowed>::new();
    rules.insert(
        START.to_string(),
        Allowed::new(
            [END.to_string()].into_iter().collect(),
            ["hello".to_string()].into_iter().collect(),
        ),
    );
    rules.insert(
        "hello".to_string(),
        Allowed::new(
            [START.to_string()].into_iter().collect(),
            ["world".to_string(), "there".to_string()]
                .into_iter()
                .collect(),
        ),
    );

    rules.insert(
        "world".to_string(),
        Allowed::new(
            ["hello".to_string()].into_iter().collect(),
            ["!".to_string()].into_iter().collect(),
        ),
    );

    rules.insert(
        "there".to_string(),
        Allowed::new(
            ["hello".to_string()].into_iter().collect(),
            [END.to_string()].into_iter().collect(),
        ),
    );

    rules.insert(
        "!".to_string(),
        Allowed::new(
            ["world".to_string()].into_iter().collect(),
            [END.to_string()].into_iter().collect(),
        ),
    );

    rules.insert(
        END.to_string(),
        Allowed::new(
            ["there".to_string(), "!".to_string()].into_iter().collect(),
            [START.to_string()].into_iter().collect(),
        ),
    );

    rules
}

fn main() {
    let rules = get_rules();
    let vector = generate_wfc_vector(&rules, 3);

    let result = iterate(vector, &rules);

    match result {
        Ok(v) => print!("{}", merge(v)),
        Err(e) => panic!("{}", e),
    }
}
