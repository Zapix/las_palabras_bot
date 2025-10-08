use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRegularConjugacion {
    #[serde(rename(deserialize = "yo"))]
    pub first_person_singular: String,

    #[serde(rename(deserialize = "tú"))]
    pub second_person_singular: String,

    #[serde(rename(deserialize = "el"))]
    pub third_person_singular: String,

    #[serde(rename(deserialize = "nosotros"))]
    pub first_person_plural: String,

    #[serde(rename(deserialize = "vosotros"))]
    pub second_person_plural: String,

    #[serde(rename(deserialize = "ellos"))]
    pub third_person_plural: String,
}

#[cfg(test)]
mod tests {
    use super::RawRegularConjugacion;

    #[test]
    fn test_deserialize_regular_conjugation() {
        let json_data = r#"
        {
            "yo": "hablo",
            "tú": "hablas",
            "el": "habla",
            "nosotros": "hablamos",
            "vosotros": "habláis",
            "ellos": "hablan"
        }
        "#;
        let conjugation: RawRegularConjugacion = serde_json::from_str(json_data).unwrap();
        assert_eq!(conjugation.first_person_singular, "hablo");
        assert_eq!(conjugation.second_person_singular, "hablas");
        assert_eq!(conjugation.third_person_singular, "habla");
        assert_eq!(conjugation.first_person_plural, "hablamos");
        assert_eq!(conjugation.second_person_plural, "habláis");
        assert_eq!(conjugation.third_person_plural, "hablan");
    }
}
