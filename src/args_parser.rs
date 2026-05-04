const DEFAULT_MEDIA_EXTENSIONS: [&str; 11] = [
    "jpg", "jpeg", "gif", "bmp", "png", "webp", "webm", "tiff", "mp4", "mpg", "mov",
];

#[derive(Debug, Clone)]
pub struct OptionFlags {
    debug: bool,
    help: bool,
    files: Vec<String>,
    media_extensions: Vec<String>,
}

impl Default for OptionFlags {
    fn default() -> Self {
        OptionFlags {
            debug: false,
            help: false,
            files: Vec::new(),
            media_extensions: DEFAULT_MEDIA_EXTENSIONS
                .iter()
                .map(|extension| extension.to_string())
                .collect(),
        }
    }
}

impl OptionFlags {
    fn new() -> Self {
        Default::default()
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn is_help(&self) -> bool {
        self.help
    }

    pub fn get_files(&self) -> &Vec<String> {
        &self.files
    }

    pub fn get_media_extensions(&self) -> &Vec<String> {
        &self.media_extensions
    }
}

pub fn parse_args(args: Vec<String>) -> OptionFlags {
    let mut options: OptionFlags = OptionFlags::new();
    println!("ARGS: {:?}", args);

    // starting from 1 since it's executable itself
    let mut i = 1;
    while i < args.len() {
        match args.get(i).unwrap().as_str() {
            "-v" | "--verbose" => options.debug = true,
            "-h" | "--help" => {
                options.help = true;
                break;
            }
            "-f" | "--files" => {
                i += 1;
                while i < args.len() {
                    let file = args.get(i).unwrap();
                    if file.starts_with("-") {
                        i -= 1;
                        break;
                    }
                    options.files.push(file.clone());
                    i += 1;
                }
            }
            "-e" | "--extensions" => {
                let mut media_extensions = Vec::new();
                i += 1;
                while i < args.len() {
                    let extension = args.get(i).unwrap();
                    if extension.starts_with("-") {
                        i -= 1;
                        break;
                    }
                    media_extensions.push(extension.trim_start_matches(".").to_lowercase());
                    i += 1;
                }
                if !media_extensions.is_empty() {
                    options.media_extensions = media_extensions;
                }
            }
            _ => {}
        }
        i += 1;
    }

    options
}

#[cfg(test)]
mod args_parser_tests {
    use super::*;

    fn args(values: Vec<&str>) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn uses_default_media_extensions() {
        let options = parse_args(args(vec!["ih"]));

        assert_eq!(
            options.get_media_extensions(),
            &vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "gif".to_string(),
                "bmp".to_string(),
                "png".to_string(),
                "webp".to_string(),
                "webm".to_string(),
                "tiff".to_string(),
                "mp4".to_string(),
                "mpg".to_string(),
                "mov".to_string(),
            ]
        );
    }

    #[test]
    fn parses_media_extensions() {
        let options = parse_args(args(vec!["ih", "--extensions", ".heic", "avif"]));

        assert_eq!(
            options.get_media_extensions(),
            &vec!["heic".to_string(), "avif".to_string()]
        );
    }
}
