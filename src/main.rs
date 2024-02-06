use clap::Parser;

mod directory;
mod index;
mod search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    directory::init_fs()?;

    let app = App::parse();

    if app.clear {
        std::fs::remove_file(directory::get_data_dir().join("index.json"))?;
        println!("Index cleared!");
        return Ok(());
    }

    if app.index {
        let mut index = index::Index::new();

        index.build()?;
        index.save(&directory::get_data_dir().join("index.json"))?;

        println!("Index built!");
    }

    let options = search::SearchOptions {
        query: app.search,
        path: {
            if let Some(path) = app.path {
                let p = std::path::Path::new(&path);
                let p = p.canonicalize()?;
                Some(p)
            } else {
                None
            }
        },
        filetype: app.filetype,
    };

    let query = options
        .query
        .as_ref()
        .unwrap_or(&".".to_string())
        .to_owned();

    let search = if options.query.is_some() {
        search::SearchResults::search(options)?
    } else {
        return Ok(());
    };

    if app.count {
        println!("{} files found!", search.results.len());
    } else if let Some(path) = app.serialize {
        search.save(&path)?;
        println!("Results saved to {}!", path);
    } else {
        search.display(&query);
    }

    if app.stats {
        println!("{}", "-".repeat(50));
        println!(
            "{} files found in {} seconds!",
            search.results.len(),
            start.elapsed().as_secs_f64()
        );
    }

    Ok(())
}

#[derive(Parser)]
#[command(about, version)]
struct App {
    /// Finds every file on the system and caches it for later searches.
    #[arg(long, short)]
    index: bool,
    /// Searches your system for a file. Accepts a RegEx pattern.
    #[arg(long, short)]
    search: Option<String>,
    /// An optional path to search under.
    #[arg(long, short)]
    path: Option<String>,
    /// Clears the index.
    #[arg(long, short)]
    clear: bool,
    /// Saves the search results to a file.
    #[arg(long, short = 'w')]
    serialize: Option<String>,
    /// Counts the number of files found.
    #[arg(long, short = 'l')]
    count: bool,
    /// Filters the search results by file type.
    #[arg(long, short)]
    filetype: Option<String>,
    /// Displays statistics about the search results.
    #[arg(long, short = 'x')]
    stats: bool,
}
