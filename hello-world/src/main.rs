mod collections;
mod control_flow;
mod game;
mod ownership;
mod strings;
mod structs;
mod types;

pub mod examples {
    pub fn types() {
        super::types::basics();
        super::types::slices();
    }

    pub fn strings() {
        crate::strings::basics();
    }

    pub fn control_flow() {
        crate::control_flow::basics();
    }

    pub fn ownership() {
        crate::ownership::basics();
        super::ownership::references();
    }

    pub fn structs() {
        crate::structs::creation_and_mutation();
        super::structs::enums();
    }

    pub fn random_number_game() {
        crate::game::random_number(String::from("Arjun"));
    }

    pub fn collections() {
        crate::collections::basics();
        crate::collections::enum_vectors();
    }
}

fn main() {
    examples::types();
    examples::strings();
    examples::control_flow();
    examples::ownership();
    examples::structs();
    examples::collections();
    //_run_random_number_game("Arjun".to_string());
}
