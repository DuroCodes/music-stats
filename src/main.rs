use demand::{DemandOption, Input, Select};
use lastfm::{types::TimePeriod, user::LastFmUser};

mod lastfm;
mod magick;
mod themes;

#[tokio::main]
async fn main() {
    let user = Input::new("What's your Last.fm username?")
        .validation(|u| match u.is_empty() {
            true => Err("Username cannot be empty"),
            false => Ok(()),
        })
        .run()
        .expect("Failed to get username");

    let api_key = Input::new("What's your Last.fm API key?")
        .password(true)
        .description("You can get one at https://www.last.fm/api/account/create")
        .validation(|u| match u.is_empty() {
            true => Err("API key cannot be empty"),
            false => Ok(()),
        })
        .run()
        .expect("Failed to get API key");

    let period = Select::new("Select a time period")
        .option(DemandOption::new(TimePeriod::Week))
        .option(DemandOption::new(TimePeriod::Month))
        .option(DemandOption::new(TimePeriod::Quarter))
        .option(DemandOption::new(TimePeriod::Half))
        .option(DemandOption::new(TimePeriod::Year))
        .option(DemandOption::new(TimePeriod::Overall))
        .filterable(true)
        .run()
        .expect("Failed to get time period");

    let theme = Select::new("Select a theme")
        .option(DemandOption::new("Midnight"))
        .option(DemandOption::new("Forest"))
        .option(DemandOption::new("Ocean"))
        .option(DemandOption::new("Strawberry"))
        .option(DemandOption::new("Bumblebee"))
        .option(DemandOption::new("Crimson"))
        .option(DemandOption::new("Aqua"))
        .option(DemandOption::new("Lavender"))
        .option(DemandOption::new("Emerald"))
        .option(DemandOption::new("Cherry"))
        .option(DemandOption::new("Twilight"))
        .option(DemandOption::new("Flame"))
        .option(DemandOption::new("Moss"))
        .option(DemandOption::new("Catppuccin"))
        .option(DemandOption::new("Horizon"))
        .option(DemandOption::new("Auto (from your profile picture)"))
        .filterable(true)
        .run()
        .expect("Failed to get theme");

    let lastfm = LastFmUser::new(api_key, user, 6, period);

    println!("Generating summary...");

    let summary = magick::generate_summary(lastfm, theme, "images/output.jpg").await;
    let relative_dir = std::env::current_dir().unwrap();

    match summary {
        Ok(s) => println!(
            "Summary generated successfully: {}",
            relative_dir.join(s).display()
        ),
        Err(e) => eprintln!("Error: {e}"),
    }
}
