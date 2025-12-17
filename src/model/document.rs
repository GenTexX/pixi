pub struct Document {
    pub path: Option<std::path::PathBuf>,
    pub title: String,
//  pub project: Project,
    pub dirty: bool,
}
impl Document {
    pub fn untitled() -> Document {
        Document { path: None, title: "untitled".to_string(), dirty: false }
    }
}