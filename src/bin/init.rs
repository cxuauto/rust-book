use std::io;
use std::io::Write;
use std::process::Command;
use clap::{App, ArgMatches, SubCommand};
use mdbook::MDBook;
use mdbook::errors::Result;
use mdbook::config;
use get_book_dir;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init")
        .about("Create boilerplate structure and files in the directory")
        // the {n} denotes a newline which will properly aligned in all help messages
        .arg_from_usage("[dir] 'A directory for your book{n}(Defaults to Current Directory \
                         when omitted)'")
        .arg_from_usage("--theme 'Copies the default theme into your source folder'")
        .arg_from_usage("--force 'skip confirmation prompts'")
}

// Init command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut builder = MDBook::init(&book_dir);
    let mut config = config::Config::default();

    // If flag `--theme` is present, copy theme to src
    if args.is_present("theme") {
        config.set("output.html.theme", "src/theme")?;
        // Skip this if `--force` is present
        if !args.is_present("force") {
            // Print warning
            println!();
            println!(
                "Copying the default theme to {}",
                builder.config().book.src.display()
            );
            println!("This could potentially overwrite files already present in that directory.");
            print!("\nAre you sure you want to continue? (y/n) ");

            // Read answer from user and exit if it's not 'yes'
            if confirm() {
                builder.copy_theme(true);
            }
        } else {
            builder.copy_theme(true);
        }
    }

    println!("\nDo you want a .gitignore to be created? (y/n)");

    if confirm() {
        builder.create_gitignore(true);
    }

    config.book.title = request_book_title();

    if let Some(author) = get_author_name() {
        debug!("Obtained user name from gitconfig: {:?}", author);
        config.book.authors.push(author);
        builder.with_config(config);
    }

    builder.build()?;
    println!("\nAll done, no errors...");

    Ok(())
}

/// Obtains author name from git config file by running the `git config` command.
fn get_author_name() -> Option<String> {
    let output = Command::new("git")
        .args(&["config", "--get", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        None
    }
}

/// Request book title from user and return if provided.
fn request_book_title() -> Option<String> {
    println!("What title would you like to give the book? ");
    io::stdout().flush().unwrap();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    let resp = resp.trim();
    if resp.is_empty() {
        None
    } else {
        Some(resp.into())
    }
}

// Simple function for user confirmation
fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    match &*s.trim() {
        "Y" | "y" | "yes" | "Yes" => true,
        _ => false,
    }
}
