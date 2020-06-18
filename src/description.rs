use pokerust::{FromName, PokemonSpecies};

use crate::types::{ServiceResult, PokemonError};

pub fn fetch_description(name: &str) -> ServiceResult<String> {
    let pokemon = match PokemonSpecies::from_name(&name) {
        Ok(pokemon) => pokemon,
        Err(_err) => return Err(PokemonError {
            status: String::from("error"),
            reason: String::from("Pokemon was not found")
        }),
    };

    let description_en = find_flavor_text(&pokemon);
    Ok(description_en)
}

fn find_flavor_text(pokemon: &PokemonSpecies) -> String {
    let flavor_texts = &pokemon.flavor_text_entries;
    let flavor_text_en = flavor_texts.iter().find(|&flavor_text| {
        flavor_text.language.name == "en" && 
        flavor_text.version.as_ref().unwrap().name == "omega-ruby"
    });
    let text: String = flavor_text_en.unwrap()
        .flavor_text
        .to_string()
        .replace("\n", "");
    text
}