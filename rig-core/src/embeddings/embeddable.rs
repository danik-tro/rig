//! The module defines the [Embeddable] trait, which must be implemented for types that can be embedded.

/// The associated type `Kind` on the trait `Embeddable` must implement this trait.
pub trait EmbeddingKind {}

/// Used for structs that contain a single embedding target.
pub struct SingleEmbedding;
impl EmbeddingKind for SingleEmbedding {}

/// Used for structs that contain many embedding targets.
pub struct ManyEmbedding;
impl EmbeddingKind for ManyEmbedding {}

#[derive(Debug, thiserror::Error)]
pub enum EmbeddableError {
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Trait for types that can be embedded.
/// The `embeddable` method returns a list of strings for which embeddings will be generated by the embeddings builder.
/// If the type `Kind` is `SingleEmbedding`, the list of strings contains a single item, otherwise, the list can contain many items.
pub trait Embeddable {
    type Kind: EmbeddingKind;
    type Error: std::error::Error;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error>;
}

//////////////////////////////////////////////////////
/// Implementations of Embeddable for common types ///
//////////////////////////////////////////////////////
impl Embeddable for String {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.clone()])
    }
}

impl Embeddable for i8 {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for i16 {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for i32 {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for i64 {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for i128 {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for f32 {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for f64 {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for bool {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl Embeddable for char {
    type Kind = SingleEmbedding;
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(vec![self.to_string()])
    }
}

impl<T: Embeddable> Embeddable for Vec<T> {
    type Kind = ManyEmbedding;
    type Error = T::Error;

    fn embeddable(&self) -> Result<Vec<String>, Self::Error> {
        Ok(self
            .iter()
            .map(|i| i.embeddable())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate as rig;
    use rig::embeddings::embeddable::{Embeddable, EmbeddableError};
    use rig_derive::Embeddable;
    use serde::Serialize;

    fn serialize(definition: Definition) -> Result<Vec<String>, EmbeddableError> {
        Ok(vec![
            serde_json::to_string(&definition).map_err(EmbeddableError::SerdeError)?
        ])
    }

    #[derive(Embeddable)]
    struct FakeDefinition {
        id: String,
        word: String,
        #[embed(embed_with = "serialize")]
        definition: Definition,
    }

    #[derive(Serialize, Clone)]
    struct Definition {
        word: String,
        link: String,
        speech: String,
    }

    #[test]
    fn test_custom_embed() {
        let fake_definition = FakeDefinition {
            id: "doc1".to_string(),
            word: "house".to_string(),
            definition: Definition {
                speech: "noun".to_string(),
                word: "a building in which people live; residence for human beings.".to_string(),
                link: "https://www.dictionary.com/browse/house".to_string(),
            },
        };

        assert_eq!(
            fake_definition.embeddable().unwrap(),
            vec!["{\"word\":\"a building in which people live; residence for human beings.\",\"link\":\"https://www.dictionary.com/browse/house\",\"speech\":\"noun\"}".to_string()]
        )
    }

    #[derive(Embeddable)]
    struct FakeDefinition2 {
        id: String,
        word: String,
        #[embed]
        definition: String,
    }

    #[test]
    fn test_simple_embed() {
        let fake_definition = FakeDefinition2 {
            id: "doc1".to_string(),
            word: "house".to_string(),
            definition: "a building in which people live; residence for human beings.".to_string(),
        };

        assert_eq!(
            fake_definition.embeddable().unwrap(),
            vec!["a building in which people live; residence for human beings.".to_string()]
        );

        assert!(false)
    }

    #[derive(Embeddable)]
    struct Company {
        id: String,
        company: String,
        #[embed]
        employee_ages: Vec<i32>,
    }

    #[test]
    fn test_multiple_embed() {
        let company = Company {
            id: "doc1".to_string(),
            company: "Google".to_string(),
            employee_ages: vec![25, 30, 35, 40],
        };

        assert_eq!(
            company.embeddable().unwrap(),
            vec![
                "25".to_string(),
                "30".to_string(),
                "35".to_string(),
                "40".to_string()
            ]
        );
    }

    #[derive(Embeddable)]
    struct Company2 {
        id: String,
        #[embed]
        company: String,
        #[embed]
        employee_ages: Vec<i32>,
    }

    #[test]
    fn test_many_embed() {
        let company = Company2 {
            id: "doc1".to_string(),
            company: "Google".to_string(),
            employee_ages: vec![25, 30, 35, 40],
        };

        assert_eq!(
            company.embeddable().unwrap(),
            vec![
                "Google".to_string(),
                "25".to_string(),
                "30".to_string(),
                "35".to_string(),
                "40".to_string()
            ]
        );
    }
}