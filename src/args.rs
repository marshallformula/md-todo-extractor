use clap::{App, Arg, ArgGroup};

pub fn determine_vault() -> String {
    // cli args
    let matches = App::new("todoer")
        .author("marshallformula")
        .arg(
            Arg::with_name("work")
                .short("w")
                .long("work")
                .help("use the `work` vault"),
        )
        .arg(
            Arg::with_name("marshallformula")
                .short("m")
                .long("marshallformula")
                .help("use the `marshallformula` vault"),
        )
        .group(
            ArgGroup::with_name("vault")
                .args(&["work", "marshallformula"])
                .required(true),
        )
        .get_matches();

    return if matches.is_present("work") {
        String::from("/Users/nate/obsidian/work")
    } else {
        String::from("/Users/nate/obsidian/marshallformula")
    };
}
