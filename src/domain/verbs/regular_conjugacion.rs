use super::raw_regular_conjugacion::RawRegularConjugacion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RegularConjugacion {
    pub first_person_singular: String,
    pub second_person_singular: String,
    pub third_person_singular: String,
    pub first_person_plural: String,
    pub second_person_plural: String,
    pub third_person_plural: String,
}

impl From<RegularConjugacion> for RawRegularConjugacion {
    fn from(conjugation: RegularConjugacion) -> Self {
        RawRegularConjugacion {
            first_person_singular: conjugation.first_person_singular,
            second_person_singular: conjugation.second_person_singular,
            third_person_singular: conjugation.third_person_singular,
            first_person_plural: conjugation.first_person_plural,
            second_person_plural: conjugation.second_person_plural,
            third_person_plural: conjugation.third_person_plural,
        }
    }
}

impl From<RawRegularConjugacion> for RegularConjugacion {
    fn from(raw: RawRegularConjugacion) -> Self {
        RegularConjugacion {
            first_person_singular: raw.first_person_singular,
            second_person_singular: raw.second_person_singular,
            third_person_singular: raw.third_person_singular,
            first_person_plural: raw.first_person_plural,
            second_person_plural: raw.second_person_plural,
            third_person_plural: raw.third_person_plural,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RegularConjugacion;
    use serde_json;
    #[test]
    fn test_deserialize_regular_conjugation() {
        let json_data = r#"
        {
            "first_person_singular": "hablo",
            "second_person_singular": "hablas",
            "third_person_singular": "habla",
            "first_person_plural": "hablamos",
            "second_person_plural": "habláis",
            "third_person_plural": "hablan"
        }
        "#;
        let conjugation: RegularConjugacion = serde_json::from_str(json_data).unwrap();
        assert_eq!(conjugation.first_person_singular, "hablo");
        assert_eq!(conjugation.second_person_singular, "hablas");
        assert_eq!(conjugation.third_person_singular, "habla");
        assert_eq!(conjugation.first_person_plural, "hablamos");
        assert_eq!(conjugation.second_person_plural, "habláis");
        assert_eq!(conjugation.third_person_plural, "hablan");
    }

    #[test]
    fn test_from_raw() {
        let raw = super::RawRegularConjugacion {
            first_person_singular: "hablo".to_string(),
            second_person_singular: "hablas".to_string(),
            third_person_singular: "habla".to_string(),
            first_person_plural: "hablamos".to_string(),
            second_person_plural: "habláis".to_string(),
            third_person_plural: "hablan".to_string(),
        };
        let conjugation: RegularConjugacion = raw.into();
        assert_eq!(conjugation.first_person_singular, "hablo");
        assert_eq!(conjugation.second_person_singular, "hablas");
        assert_eq!(conjugation.third_person_singular, "habla");
        assert_eq!(conjugation.first_person_plural, "hablamos");
        assert_eq!(conjugation.second_person_plural, "habláis");
        assert_eq!(conjugation.third_person_plural, "hablan");
    }

    #[test]
    fn test_to_raw() {
        let conjugation = RegularConjugacion {
            first_person_singular: "hablo".to_string(),
            second_person_singular: "hablas".to_string(),
            third_person_singular: "habla".to_string(),
            first_person_plural: "hablamos".to_string(),
            second_person_plural: "habláis".to_string(),
            third_person_plural: "hablan".to_string(),
        };
        let raw: super::RawRegularConjugacion = conjugation.into();
        assert_eq!(raw.first_person_singular, "hablo");
        assert_eq!(raw.second_person_singular, "hablas");
        assert_eq!(raw.third_person_singular, "habla");
        assert_eq!(raw.first_person_plural, "hablamos");
        assert_eq!(raw.second_person_plural, "habláis");
        assert_eq!(raw.third_person_plural, "hablan");
    }
}
