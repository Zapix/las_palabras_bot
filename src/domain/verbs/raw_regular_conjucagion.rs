use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRegularConjucagion {
    #[serde(rename = "yo")]
    pub first_person_singular: String,

    #[serde(rename = "tú")]
    pub second_person_singular: String,

    #[serde(rename = "el")]
    pub third_person_singular: String,

    #[serde(rename = "nosotros")]
    pub first_person_plural: String,

    #[serde(rename = "vosotros")]
    pub second_person_plural: String,

    #[serde(rename = "ellos")]
    pub third_person_plural: String,
}

#[cfg(test)]
mod tests {
    use super::RawRegularConjucagion;

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
        let conjugation: RawRegularConjucagion = serde_json::from_str(json_data).unwrap();
        assert_eq!(conjugation.first_person_singular, "hablo");
        assert_eq!(conjugation.second_person_singular, "hablas");
        assert_eq!(conjugation.third_person_singular, "habla");
        assert_eq!(conjugation.first_person_plural, "hablamos");
        assert_eq!(conjugation.second_person_plural, "habláis");
        assert_eq!(conjugation.third_person_plural, "hablan");
    }
}
