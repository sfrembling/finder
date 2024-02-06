use clap::Parser;

mod directory;
mod index;
mod search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let results = if let Some(query) = app.search {
        match app.path {
            Some(path) => search::SearchResults::search_path(&query, &path)?,
            None => search::SearchResults::search(&query)?,
        }
    } else {
        return Ok(());
    };

    if let Some(path) = app.serialize {
        results.save(&path)?;
        println!("Results saved to {}!", path);
    } else {
        results.display();
    }

    Ok(())
}

#[derive(Parser)]
#[command(about, version)]
struct App {
    /// Finds every file on the system and caches it for later searches.
    #[arg(long, short)]
    index: bool,
    /// Searches your system for a file.
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
}
