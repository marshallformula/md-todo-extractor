use colored::Colorize;

mod args;
mod vault_files;

fn main() {
    let vault = args::determine_vault();
    let md_files = vault_files::find_md_files(vault);
    let parser = vault_files::Parser::init();

    md_files
        .into_iter()
        .map(|file| parser.process_todos(file))
        .filter(|tf| tf.has_todos() && tf.filename != "index.md")
        .for_each(|tf| {
            println!("{}", tf.filename.bold().bright_cyan());
            for item in tf.items {
                println!("    [ ] {}", item);
            }
            println!();
        });
}
