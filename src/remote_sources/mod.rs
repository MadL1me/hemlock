pub enum SourceOrigin {
    Git,
    LocalGlob,
    Unknown,
}

pub struct LocalFilesSource {
    pub glob_path: String
}