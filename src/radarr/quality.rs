use serde::{Serialize, Deserialize};

use super::quality_spec::QualitySpec;
use super::revision::Revision;

#[derive(Serialize, Deserialize, Debug)]
pub struct Quality {
    pub quality: QualitySpec,

    #[serde(rename = "customFormats")]
    pub custom_formats: Vec<String>,

    pub revision: Revision,
}
