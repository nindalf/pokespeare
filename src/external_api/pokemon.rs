use anyhow::anyhow;
use reqwest::StatusCode;
#[derive(serde::Deserialize)]
struct PokedexEntry {
    id: i32,
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

// TODO change this fn to return thiserror instead of anyhow
// Check for specific errors in unit tests
pub(crate) async fn get_pokemon_id_and_description(
    pokemon_name: &str,
) -> anyhow::Result<(i32, String)> {
    #[cfg(not(test))]
    let url = "https://pokeapi.co/api/v2/pokemon-species";
    #[cfg(test)]
    let url = &mockito::server_url();

    let resp = reqwest::get(format!("{}/{}", url, pokemon_name)).await?;
    if resp.status() != StatusCode::OK {
        return Err(anyhow!("Received non-200 response"));
    }

    let entry = resp.json::<PokedexEntry>().await?;
    let flavour = entry
        .flavor_text_entries
        .iter()
        .filter(|entry| entry.language.name == "en")
        .next()
        .ok_or_else(|| anyhow!("Couldn't find any descriptions"))?;

    let extra_chars: &[_] = &['\x0C', '\n'];
    Ok((entry.id, flavour.flavor_text.replace(extra_chars, " ")))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn test_success_response() {
        let _m = mockito::mock("GET", "/charizard")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body_from_file("/Users/nindalf/Repos/pokespeare/src/external_api/test_responses/pokemon_success.json")
            .create();
        match get_pokemon_id_and_description("charizard").await {
            Ok((id, description)) => {
                assert_eq!(id, 6);
                assert_eq!(description, "Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.");
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
        assert!(result.is_err())
    }

    #[actix_rt::test]
    async fn test_malformed_output() {
        let _m = mockito::mock("GET", "/charmander")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(r#"{"id": 3, "flavor_text_entries": [])"#)
            .create();
        let result = get_pokemon_id_and_description("charmander").await;
        assert!(result.is_err())
    }
}
