extern crate reqwest;
extern crate serde_json;

use reqwest::Error;
use serde_json::{Value, Map};

/* Process the division data.
 *
 * In: Division - The division to be processed
 * Out: void, error
 */
fn division_processing(division: &Value) -> Result<(), Error> {
    /* We need the division URL to grab the actual division name.  The standings
     * JSON returns it
     */
    let div_url = format!("https://statsapi.mlb.com{}", division["division"]["link"].as_str().unwrap());
    let div_json:Map<String, Value> = reqwest::get(&div_url)?.json()?;

    /* Display the division name */
    println!("[b]{}[/b]", div_json["divisions"][0]["nameShort"].as_str().unwrap());

    /* Get the teams */
    let teams = division["teamRecords"].as_array().unwrap();

    /* Set up the total wins and losses for the division */
    let mut t_wins = 0;
    let mut t_loss = 0;

    /* Process the teams */
    for y in teams {
        /* Get relevant information from the team info itself */
        let name = y["team"]["name"].as_str().unwrap();
        let wins = y["wins"].as_i64().unwrap();
        let losses = y["losses"].as_i64().unwrap();
        let win_p = y["winningPercentage"].as_str().unwrap();
        let gbac = y["gamesBack"].as_str().unwrap();

        /* Increment the wins and losses for the divison */
        t_wins += wins;
        t_loss += losses;

        /* If the team is the division leader, ignore the games back */
        if gbac == "-" {
            println!("{0}: {1} - {2} ({3})", name, wins, losses, win_p);
        } else {
            println!("{0}: {1} - {2} ({3}) - {4} games back", name, wins, losses, win_p, gbac);
        }
    }

    /* Calculate the overall winning percentage of the division */
    let mut d_win_p:f64 = t_wins as f64 / (t_wins as f64 + t_loss as f64);
    println!("\nDivision win - loss: {0} - {1} ({2:.3})", t_wins, t_loss, d_win_p);

    /* Calculate the overall winning percentage of the division, without the
     * best team in the division
     */
    let mut fl_wins = t_wins - teams.first().unwrap()["wins"].as_i64().unwrap();
    let mut fl_loss = t_loss - teams.first().unwrap()["losses"].as_i64().unwrap();
    d_win_p = fl_wins as f64 / (fl_wins as f64 + fl_loss as f64);
    println!("Division win - loss (no 1st): {0} - {1} ({2:.3})", fl_wins, fl_loss, d_win_p);

    /* Calculate the overall winning percentage of the division, without the
     * worst team in the division
     */
    fl_wins = t_wins - teams.last().unwrap()["wins"].as_i64().unwrap();
    fl_loss = t_loss - teams.last().unwrap()["losses"].as_i64().unwrap();
    d_win_p = fl_wins as f64 / (fl_wins as f64 + fl_loss as f64);
    println!("Division win - loss (no last): {0} - {1} ({2:.3})\n\n", fl_wins, fl_loss, d_win_p);

    Ok(())
}

fn main() -> Result<(), Error> {
    /* Grab the league standings.  103 = AL, 104 = NL. */
    let resp:Map<String, Value> = reqwest::get(
        "https://statsapi.mlb.com/api/v1/standings?leagueId=103,104&season=2019&standingsTypes=regularSeason")?.json()?;

    let divisions = resp["records"].as_array().unwrap();

    /* Process each division */
    for x in divisions.iter() {
        division_processing(x)?;
    }

    Ok(())
}
