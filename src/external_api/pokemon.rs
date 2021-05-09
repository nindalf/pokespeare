use reqwest::StatusCode;
#[derive(serde::Deserialize)]
struct PokedexEntry {
    id: i32,
    name: String,
    flavor_text_entries: Vec<FlavourTextEntry>,
}

#[derive(serde::Deserialize)]
struct FlavourTextEntry {
    flavor_text: String,
    language: Language,
}
#[derive(serde::Deserialize)]
struct Language {
    name: String,
}

pub(crate) struct Pokemon {
    pub id: i32,
    pub name: String,
    pub description: String,
}
#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum PokeError {
    #[error("Not Found")]
    NotFound,
    #[error("No Flavour Text")]
    NoFlavourText,
    #[error("APIError")]
    APIError,
}

pub(crate) async fn get_pokemon_id_and_description(
    pokemon_name: &str,
) -> Result<Pokemon, PokeError> {
    #[cfg(not(test))]
    let url = "https://pokeapi.co/api/v2/pokemon-species";
    #[cfg(test)]
    let url = &mockito::server_url();

    let resp = reqwest::get(format!("{}/{}", url, pokemon_name))
        .await
        .map_err(|_| PokeError::APIError)?;

    if resp.status() == StatusCode::NOT_FOUND {
        return Err(PokeError::NotFound);
    }

    let entry = resp
        .json::<PokedexEntry>()
        .await
        .map_err(|_| PokeError::APIError)?;

    let flavour = entry
        .flavor_text_entries
        .iter()
        .find(|entry| entry.language.name == "en")
        .ok_or(PokeError::NoFlavourText)?;

    let extra_chars: &[_] = &['\x0C', '\n'];
    Ok(Pokemon {
        id: entry.id,
        name: entry.name,
        description: flavour.flavor_text.replace(extra_chars, " "),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn test_success_response() {
        let _m = mockito::mock("GET", "/charizard")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body_from_file("./src/external_api/test_responses/pokemon_success.json")
            .create();
        match get_pokemon_id_and_description("charizard").await {
            Ok(pokemon) => {
                assert_eq!(pokemon.id, 6);
                assert_eq!(pokemon.name, "charizard");
                assert_eq!(pokemon.description, "Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.");
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[actix_rt::test]
    async fn test_bad_input() {
        let _m = mockito::mock("GET", "/goku")
            .with_status(404)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_body("Not Found")
            .create();
        let result = get_pokemon_id_and_description("goku").await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), PokeError::NotFound);
    }

    #[actix_rt::test]
    async fn test_malformed_output() {
        let _m = mockito::mock("GET", "/charmander")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(r#"{"id": 3, "flavor_text_entries": [])"#)
            .create();
        let result = get_pokemon_id_and_description("charmander").await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), PokeError::APIError);
    }

    #[actix_rt::test]
    async fn test_empty_output() {
        let _m = mockito::mock("GET", "/charizard")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body_from_file("./src/external_api/test_responses/pokemon_failure.json")
            .create();
        let result = get_pokemon_id_and_description("charizard").await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), PokeError::NoFlavourText);
    }
}
