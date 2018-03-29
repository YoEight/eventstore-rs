#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StreamAcl {
    #[serde(rename = "$r")]
    pub read_roles: Vec<String>,

    #[serde(rename = "$w")]
    pub write_roles: Vec<String>,

    #[serde(rename = "$d")]
    pub delete_roles: Vec<String>,

    #[serde(rename = "$mr")]
    pub meta_read_roles: Vec<String>,

    #[serde(rename = "$mw")]
    pub meta_write_roles: Vec<String>,
}
