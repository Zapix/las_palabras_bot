use crate::helpers::spawn_app;
use las_palabras_bot::domain::verbs::raw_verb::RawVerb;
use las_palabras_bot::domain::verbs::repository::{VerbsDb, VerbsRepository};

const BLANK_VERB: &str = r#"
    {
        "verbo": "zurriagar",
        "perfecto": {
            "futuro": {
                "yo": "habré zurriagado",
                "tú": "habrás zurriagado",
                "nosotros": "habremos zurriagado",
                "vosotros": "habréis zurriagado",
                "el": "habrá zurriagado",
                "ellos": "habrán zurriagado"
            },
            "pasado": {
                "yo": "había zurriagado",
                "tú": "habías zurriagado",
                "nosotros": "habíamos zurriagado",
                

                "vosotros": "habíais zurriagado",
                "el": "había zurriagado",
                "ellos": "habían zurriagado"
            },
            "presente": {
                "yo": "he zurriagado",
                "tú": "has zurriagado",
                "nosotros": "hemos zurriagado",
                "vosotros": "habéis zurriagado",
                "el": "ha zurriagado",
                "ellos": "han zurriagado"
            },
            "preterito": {
                "yo": "hube zurriagado",
                "tú": "hubiste zurriagado",
                "nosotros": "hubimos zurriagado",
                "vosotros": "hubisteis zurriagado",
                "el": "hubo zurriagado",
                "ellos": "hubieron zurriagado"
            },
            "condicional": {
                "yo": "habría zurriagado",
                "tú": "habrías zurriagado",
                "nosotros": "habríamos zurriagado",
                "vosotros": "habríais zurriagado",
                "el": "habría zurriagado",
                "ellos": "habrían zurriagado"
            }
        },
        "imperativo": {
            "negativo": {
                "yo": "-",
                "el": "no zurriague",
                "tú": "no zurriagues",
                "ellos": "no zurriaguen",
                "nosotros": "no zurriaguemos",
                "vosotros": "no zurriaguéis"
            },
            "afirmativo": {
                "yo": "-",
                "el": "zurriague",
                "tú": "zurriaga",
                "ellos": "zurriaguen",
                "nosotros": "zurriaguemos",
                "vosotros": "zurriagad"
            }
        },
        "indicativo": {
            "futuro": {
                "yo": "zurriagaré",
                "tú": "zurriagarás",
                "nosotros": "zurriagaremos",
                "vosotros": "zurriagaréis",
                "el": "zurriagará",
                "ellos": "zurriagarán"
            },
            "presente": {
                "yo": "zurriago",
                "tú": "zurriagas",
                "nosotros": "zurriagamos",
                "vosotros": "zurriagáis",
                "el": "zurriaga",
                "ellos": "zurriagan"
            },
            "preterito": {
                "yo": "zurriagué",
                "tú": "zurriagaste",
                "nosotros": "zurriagamos",
                "vosotros": "zurriagasteis",
                "el": "zurriagó",
                "ellos": "zurriagaron"
            },
            "imperfecto": {
                "yo": "zurriagaba",
                "tú": "zurriagabas",
                "nosotros": "zurriagábamos",
                "vosotros": "zurriagabais",
                "el": "zurriagaba",
                "ellos": "zurriagaban"
            },
            "condicional": {
                "yo": "zurriagaría",
                "tú": "zurriagarías",
                "nosotros": "zurriagaríamos",
                "vosotros": "zurriagaríais",
                "el": "zurriagaría",
                "ellos": "zurriagarían"
            }
        },
        "progresivo": {
            "futuro": {
                "yo": "estaré zurriagando",
                "tú": "estarás zurriagando",
                "nosotros": "estaremos zurriagando",
                "vosotros": "estaréis zurriagando",
                "el": "estará zurriagando",
                "ellos": "estarán zurriagando"
            },
            "presente": {
                "yo": "estoy zurriagando",
                "tú": "estás zurriagando",
                "nosotros": "estamos zurriagando",
                "vosotros": "estáis zurriagando",
                "el": "está zurriagando",
                "ellos": "están zurriagando"
            },
            "preterito": {
                "yo": "estuve zurriagando",
                "tú": "estuviste zurriagando",
                "nosotros": "estuvimos zurriagando",
                "vosotros": "estuvisteis zurriagando",
                "el": "estuvo zurriagando",
                "ellos": "estuvieron zurriagando"
            },
            "imperfecto": {
                "yo": "estaba zurriagando",
                "tú": "estabas zurriagando",
                "nosotros": "estábamos zurriagando",
                "vosotros": "estabais zurriagando",
                "el": "estaba zurriagando",
                "ellos": "estaban zurriagando"
            },
            "condicional": {
                "yo": "estaría zurriagando",
                "tú": "estarías zurriagando",
                "nosotros": "estaríamos zurriagando",
                "vosotros": "estaríais zurriagando",
                "el": "estaría zurriagando",
                "ellos": "estarían zurriagando"
            }
        },
        "subjuntivo": {
            "futuro": {
                "yo": "zurriagare",
                "tú": "zurriagares",
                "nosotros": "zurriagáremos",
                "vosotros": "zurriagareis",
                "el": "zurriagare",
                "ellos": "zurriagaren"
            },
            "presente": {
                "yo": "zurriague",
                "tú": "zurriagues",
                "nosotros": "zurriaguemos",
                "vosotros": "zurriaguéis",
                "el": "zurriague",
                "ellos": "zurriaguen"
            },
            "imperfecto": {
                "yo": "zurriagara",
                "tú": "zurriagaras",
                "nosotros": "zurriagáramos",
                "vosotros": "zurriagarais",
                "el": "zurriagara",
                "ellos": "zurriagaran"
            },
            "imperfecto2": {
                "yo": "zurriagase",
                "tú": "zurriagases",
                "nosotros": "zurriagásemos",
                "vosotros": "zurriagaseis",
                "el": "zurriagase",
                "ellos": "zurriagasen"
            }
        },
        "perfecto_subjuntivo": {
            "futuro": {
                "yo": "hubiere zurriagado",
                "tú": "hubieres zurriagado",
                "nosotros": "hubiéremos zurriagado",
                "vosotros": "hubiereis zurriagado",
                "el": "hubiere zurriagado",
                "ellos": "hubieren zurriagado"
            },
            "pasado": {
                "yo": "hubiera zurriagado",
                "tú": "hubieras zurriagado",
                "nosotros": "hubiéramos zurriagado",
                "vosotros": "hubierais zurriagado",
                "el": "hubiera zurriagado",
                "ellos": "hubieran zurriagado"
            },
            "presente": {
                "yo": "haya zurriagado",
                "tú": "hayas zurriagado",
                "nosotros": "hayamos zurriagado",
                "vosotros": "hayáis zurriagado",
                "el": "haya zurriagado",
                "ellos": "hayan zurriagado"
            }
        }
    }
"#;

#[tokio::test]
async fn test_get_verb_404() {
    let app = spawn_app().await.expect("Failed to spawn app");
    let non_existent_id = uuid::Uuid::new_v4();
    let response = app
        .api_client()
        .get(format!(
            "{}/api/v1/verbs/{}",
            app.address(),
            non_existent_id
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(
        response.status().as_u16(),
        404,
        "Expected a 404 Not Found response"
    );
}

#[tokio::test]
async fn test_get_verb_success() {
    let mut app = spawn_app().await.expect("Failed to spawn app");

    let db_pool = app.db_pool();
    let verbs_repo = VerbsDb::new(db_pool);
    let raw_verb = serde_json::from_str::<RawVerb>(BLANK_VERB).expect("Failed to parse raw verb");
    let verb = verbs_repo
        .add_verb(raw_verb)
        .await
        .expect("Failed to create verb");

    let response = app
        .api_client()
        .get(format!("{}/api/v1/verbs/{}", app.address(), verb.id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(
        response.status().as_u16(),
        200,
        "Expected a 200 OK response"
    );
}
