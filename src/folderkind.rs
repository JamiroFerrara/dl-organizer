use std::*;

pub struct FolderKind {
    kinds: Vec<FolderKindEnum>,
}

pub enum FolderKindEnum {
    Pictures(Vec<String>),
    Documents(Vec<String>),
    Executables(Vec<String>),
    Audio(Vec<String>),
    Video(Vec<String>),
    Zip(Vec<String>),
    Other
}

impl fmt::Display for FolderKindEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FolderKindEnum::Pictures(_) => write!(f, "Pictures"),
            FolderKindEnum::Documents(_) => write!(f, "Documents"),
            FolderKindEnum::Executables(_) => write!(f, "Executables"),
            FolderKindEnum::Audio(_) => write!(f, "Audio"),
            FolderKindEnum::Video(_) => write!(f, "Video"),
            FolderKindEnum::Zip(_) => write!(f, "Zip"),
            FolderKindEnum::Other => write!(f, "Other"),
        }
    }
}

impl FolderKind {
    pub fn new() -> Self {
        let audio = FolderKindEnum::Audio(vec!["mp3".to_string(), "wav".to_string(), "flac".to_string(), "aac".to_string(), "m4a".to_string(), "ogg".to_string(), "wma".to_string()]);
        let pictures = FolderKindEnum::Pictures(vec!["jpg".to_string(), "png".to_string(), "gif".to_string(), "bmp".to_string(), "jpeg".to_string()]);
        let video = FolderKindEnum::Video(vec!["mp4".to_string(), "avi".to_string(), "mkv".to_string(), "flv".to_string(), "wmv".to_string(), "mov".to_string(), "mpg".to_string(), "mpeg".to_string()]);
        let exe = FolderKindEnum::Executables(vec![".exe".to_string()]);
        let docs = FolderKindEnum::Documents(vec!["doc".to_string(), "docx".to_string(), "pdf".to_string(), "txt".to_string(), "odt".to_string(), "ods".to_string(), "odp".to_string(), "odg".to_string(), "odf".to_string(), "odb".to_string(), "odc".to_string(), "odm".to_string(), "ott".to_string(), "ots".to_string(), "otp".to_string(), "otg".to_string(), "oth".to_string(), "ots".to_string(), "otf".to_string(), "otc".to_string(), "odb".to_string(), "odm".to_string(), "ott".to_string(), "ots".to_string(), "otp".to_string(), "otg".to_string(), "oth".to_string(), "ots".to_string(), "otf".to_string(), "otc".to_string(), "odb".to_string(), "odm".to_string(), "ott".to_string(), "ots".to_string(), "otp".to_string(), "otg".to_string(), "oth".to_string(), "ots".to_string(), "otf".to_string(), "otc".to_string(), "odb".to_string(), "odm".to_string(), "ott".to_string(), "ots".to_string(), "otp".to_string(), "otg".to_string(), "oth".to_string(), "ots".to_string(), "otf".to_string(), "otc".to_string(), "odb".to_string(), "odm".to_string(), "ott".to_string(), "ots".to_string(), "otp".to_string(), "otg".to_string(), "oth".to_string(), "ots".to_string(), "otf".to_string(), "otc".to_string(), "odb".to_string(), "odm".to_string(), "ott".to_string(), "ots".to_string(), "otp".to_string()]);
        let zip = FolderKindEnum::Zip(vec!["zip".to_string(), "rar".to_string(), "7z".to_string(), "tar".to_string(), "gz".to_string(), "xz".to_string(), "bz2".to_string(), "lz".to_string(), "lzma".to_string(), "zst".to_string(), "iso".to_string()]);
        let other = FolderKindEnum::Other;

        let kinds = vec![audio, pictures, video, exe, docs, other, zip];

        Self {
            kinds: kinds
        }
    }

    pub fn get_kind(&self, file_name: &str) -> Option<&FolderKindEnum> {
        self.kinds.iter().find(|kind| {
            match kind {
                FolderKindEnum::Pictures(files) => {
                    files.iter().any(|file| {
                        file_name.ends_with(file)
                    })
                }
                FolderKindEnum::Documents(files) => {
                    files.iter().any(|file| {
                        file_name.ends_with(file)
                    })
                }
                FolderKindEnum::Executables(files) => {
                    files.iter().any(|file| {
                        file_name.ends_with(file)
                    })
                }
                FolderKindEnum::Audio(files) => {
                    files.iter().any(|file| {
                        file_name.ends_with(file)
                    })
                }
                FolderKindEnum::Video(files) => {
                    files.iter().any(|file| {
                        file_name.ends_with(file)
                    })
                }
                FolderKindEnum::Zip(files) => {
                    files.iter().any(|file| {
                        file_name.ends_with(file)
                    })
                }
                FolderKindEnum::Other => {
                    false
                }
            }
        })
    }
}
