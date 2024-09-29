#[derive(Default, Debug, Clone)]
pub struct OptionFlags {
    debug: bool,
    help: bool,
    files: Vec<String>
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
}

pub fn parse_args(args: Vec<String>) -> OptionFlags {
    let mut options: OptionFlags = OptionFlags::new();

    // starting from 1 since it's executable itself
    for i in 1..args.len() {
        match args.get(i).unwrap().as_str() {
            "-d" | "--debug" => options.debug = true,
            "-h" | "--help" => {
                options.help = true;
                break;
            }
            "-f" | "--files" => {
                let mut files: Vec<String> = Vec::new();
                for i in i..args.len() {
                    let file = args.get(i).unwrap();
                    // if we encounter other command such as 
                    if file.starts_with("-") || file.starts_with("--") {
                        break;
                    }

                    files.push(file.clone());
                }
            }
            _ => {}
        }
    }

    options
}
