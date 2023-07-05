use serde::{Deserialize, Serialize};

use crate::post::LanguageId;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// A language.
pub struct Language {
    pub id: LanguageId,
    pub code: String,
    pub name: String,
}
