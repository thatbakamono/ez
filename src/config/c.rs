use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CConfiguration {
    pub(crate) standard: Option<CStandard>,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum CStandard {
    #[serde(rename = "89")]
    EightyNine,
    #[serde(rename = "99")]
    NinetyNine,
    #[serde(rename = "11")]
    Eleven,
    #[serde(rename = "17")]
    Seventeen,
}
