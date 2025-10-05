use serde::{Deserialize, Serialize};

use super::raw_regular_conjucagion::RawRegularConjucagion;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perfecto {
    pub futuro: RawRegularConjucagion,
    pub pasado: RawRegularConjucagion,
    pub presente: RawRegularConjucagion,
    pub preterito: RawRegularConjucagion,
    pub condicional: RawRegularConjucagion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Imperativo {
    pub negativo: RawRegularConjucagion,
    pub afirmativo: RawRegularConjucagion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicativo {
    pub futuro: RawRegularConjucagion,
    pub presente: RawRegularConjucagion,
    pub preterito: RawRegularConjucagion,
    pub imperfecto: RawRegularConjucagion,
    pub condicional: RawRegularConjucagion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progresivo {
    pub futuro: RawRegularConjucagion,
    pub presente: RawRegularConjucagion,
    pub preterito: RawRegularConjucagion,
    pub imperfecto: RawRegularConjucagion,
    pub condicional: RawRegularConjucagion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subjuntivo {
    pub futuro: RawRegularConjucagion,
    pub presente: RawRegularConjucagion,
    pub imperfecto: RawRegularConjucagion,
    pub imperfecto2: RawRegularConjucagion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfectoSubjuntivo {
    pub futuro: RawRegularConjucagion,
    pub presente: RawRegularConjucagion,
    pub pasado: RawRegularConjucagion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawVerb {
    #[serde(rename = "verbo")]
    pub verb: String,

    pub perfecto: Option<Perfecto>,
    pub imperativo: Option<Imperativo>,
    pub indicativo: Option<Indicativo>,
    pub progresivo: Option<Progresivo>,
    pub subjuntivo: Option<Subjuntivo>,
    pub perfecto_subjuntivo: Option<PerfectoSubjuntivo>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_raw_verb() {
        let json_data = r#"
        {
            "verbo": "hablar",
            "perfecto": {
                "futuro": {
                    "yo": "me habré hablado",
                    "tú": "te habrás hablado",
                    "nosotros": "nos habremos hablado",
                    "vosotros": "os habréis hablado",
                    "el": "se habrá hablado",
                    "ellos": "se habrán hablado"
                },
                "pasado": {
                    "yo": "me había hablado",
                    "tú": "te habías hablado",
                    "nosotros": "nos habíamos hablado",
                    "vosotros": "os habíais hablado",
                    "el": "se había hablado",
                    "ellos": "se habían hablado"
                },
                "presente": {
                    "yo": "me he hablado",
                    "tú": "te has hablado",
                    "nosotros": "nos hemos hablado",
                    "vosotros": "os habéis hablado",
                    "el": "se ha hablado",
                    "ellos": "se han hablado"
                },
                "preterito": {
                    "yo": "me hube hablado",
                    "tú": "te hubiste hablado",
                    "nosotros": "nos hubimos hablado",
                    "vosotros": "os hubisteis hablado",
                    "el": "se hubo hablado",
                    "ellos": "se hubieron hablado"
                },
                "condicional": {
                    "yo": "me habría hablado",
                    "tú": "te habrías hablado",
                    "nosotros": "nos habríamos hablado",
                    "vosotros": "os habríais hablado",
                    "el": "se habría hablado",
                    "ellos": "se habrían hablado"
                }
            },
            "imperativo": {
                "negativo": {
                    "yo": "-",
                    "el": "no se hable",
                    "tú": "no te hables",
                    "ellos": "no se hablen",
                    "nosotros": "no nos hablemos",
                    "vosotros": "no os habléis"
                },
                "afirmativo": {
                    "yo": "-",
                    "el": "háblese",
                    "tú": "háblate",
                    "ellos": "háblense",
                    "nosotros": "hablémonos",
                    "vosotros": "hablaos"
                }
            },
            "indicativo": {
                "futuro": {
                    "yo": "me hablaré",
                    "tú": "te hablarás",
                    "nosotros": "nos hablaremos",
                    "vosotros": "os hablaréis",
                    "el": "se hablará",
                    "ellos": "se hablarán"
                },
                "presente": {
                    "yo": "me hablo",
                    "tú": "te hablas",
                    "nosotros": "nos hablamos",
                    "vosotros": "os habláis",
                    "el": "se habla",
                    "ellos": "se hablan"
                },
                "preterito": {
                    "yo": "me hablé",
                    "tú": "te hablaste",
                    "nosotros": "nos hablamos",
                    "vosotros": "os hablasteis",
                    "el": "se habló",
                    "ellos": "se hablaron"
                },
                "imperfecto": {
                    "yo": "me hablaba",
                    "tú": "te hablabas",
                    "nosotros": "nos hablábamos",
                    "vosotros": "os hablabais",
                    "el": "se hablaba",
                    "ellos": "se hablaban"
                },
                "condicional": {
                    "yo": "me hablaría",
                    "tú": "te hablarías",
                    "nosotros": "nos hablaríamos",
                    "vosotros": "os hablaríais",
                    "el": "se hablaría",
                    "ellos": "se hablarían"
                }
            },
            "progresivo": {
                "futuro": {
                    "yo": "me estaré hablando",
                    "tú": "te estarás hablando",
                    "nosotros": "nos estaremos hablando",
                    "vosotros": "os estaréis hablando",
                    "el": "se estará hablando",
                    "ellos": "se estarán hablando"
                },
                "presente": {
                    "yo": "me estoy hablando",
                    "tú": "te estás hablando",
                    "nosotros": "nos estamos hablando",
                    "vosotros": "os estáis hablando",
                    "el": "se está hablando",
                    "ellos": "se están hablando"
                },
                "preterito": {
                    "yo": "me estuve hablando",
                    "tú": "te estuviste hablando",
                    "nosotros": "nos estuvimos hablando",
                    "vosotros": "os estuvisteis hablando",
                    "el": "se estuvo hablando",
                    "ellos": "se estuvieron hablando"
                },
                "imperfecto": {
                    "yo": "me estaba hablando",
                    "tú": "te estabas hablando",
                    "nosotros": "nos estábamos hablando",
                    "vosotros": "os estabais hablando",
                    "el": "se estaba hablando",
                    "ellos": "se estaban hablando"
                },
                "condicional": {
                    "yo": "me estaría hablando",
                    "tú": "te estarías hablando",
                    "nosotros": "nos estaríamos hablando",
                    "vosotros": "os estaríais hablando",
                    "el": "se estaría hablando",
                    "ellos": "se estarían hablando"
                }
            },
            "subjuntivo": {
                "futuro": {
                    "yo": "me hablare",
                    "tú": "te hablares",
                    "nosotros": "nos habláremos",
                    "vosotros": "os hablareis",
                    "el": "se hablare",
                    "ellos": "se hablaren"
                },
                "presente": {
                    "yo": "me hable",
                    "tú": "te hables",
                    "nosotros": "nos hablemos",
                    "vosotros": "os habléis",
                    "el": "se hable",
                    "ellos": "se hablen"
                },
                "imperfecto": {
                    "yo": "me hablara",
                    "tú": "te hablaras",
                    "nosotros": "nos habláramos",
                    "vosotros": "os hablarais",
                    "el": "se hablara",
                    "ellos": "se hablaran"
                },
                "imperfecto2": {
                    "yo": "me hablase",
                    "tú": "te hablases",
                    "nosotros": "nos hablásemos",
                    "vosotros": "os hablaseis",
                    "el": "se hablase",
                    "ellos": "se hablasen"
                }
            },
            "perfecto_subjuntivo": {
                "futuro": {
                    "yo": "me hubiere hablado",
                    "tú": "te hubieres hablado",
                    "nosotros": "nos hubiéremos hablado",
                    "vosotros": "os hubiereis hablado",
                    "el": "se hubiere hablado",
                    "ellos": "se hubieren hablado"
                },
                "pasado": {
                    "yo": "me hubiera hablado",
                    "tú": "te hubieras hablado",
                    "nosotros": "nos hubiéramos hablado",
                    "vosotros": "os hubierais hablado",
                    "el": "se hubiera hablado",
                    "ellos": "se hubieran hablado"
                },
                "presente": {
                    "yo": "me haya hablado",
                    "tú": "te hayas hablado",
                    "nosotros": "nos hayamos hablado",
                    "vosotros": "os hayáis hablado",
                    "el": "se haya hablado",
                    "ellos": "se hayan hablado"
                }
            }
        }
        "#;
        let verb: RawVerb = serde_json::from_str(json_data).unwrap();
        assert_eq!(verb.verb, "hablar");
    }
}
