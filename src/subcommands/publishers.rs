use std::collections::{BTreeMap, HashMap};

use crate::publishers::fetch_owners_of_crates;
use crate::{common::*, publishers::PublisherData};

pub fn publishers(args: Vec<String>, max_age: std::time::Duration) -> Result<(), std::io::Error> {
    let dependencies = sourced_dependencies(args);
    complain_about_non_crates_io_crates(&dependencies);
    let (publisher_users, publisher_teams) = fetch_owners_of_crates(&dependencies, max_age)?;

    if !publisher_users.is_empty() {
        println!("\nThe following individuals can publish updates for your dependencies:\n");
        let user_to_crate_map = transpose_publishers_map(&publisher_users);
        let map_for_display = sort_transposed_map_for_display(user_to_crate_map);
        for (i, (user, crates)) in map_for_display.iter().enumerate() {
            // We do not print usernames, since you can embed terminal control sequences in them
            // and erase yourself from the output that way.
            // TODO: check if it's possible to smuggle those into github/crates.io usernames
            let crate_list = comma_separated_list(&crates);
            println!(" {}. {} via crates: {}", i + 1, &user.login, crate_list);
        }
    }

    println!("\nNote: there may be outstanding publisher invitations. crates.io provides no way to list them.");
    println!("Invitations are also impossible to revoke, and they never expire.");
    println!("See https://github.com/rust-lang/crates.io/issues/2868 for more info.");

    if !publisher_teams.is_empty() {
        println!(
            "\nAll members of the following teams can publish updates for your dependencies:\n"
        );
        let team_to_crate_map = transpose_publishers_map(&publisher_teams);
        let map_for_display = sort_transposed_map_for_display(team_to_crate_map);
        for (i, (team, crates)) in map_for_display.iter().enumerate() {
            let crate_list = comma_separated_list(&crates);
            if let Some(url) = &team.url {
                println!(
                    " {}. \"{}\" ({}) via crates: {}",
                    i + 1,
                    &team.login,
                    url,
                    crate_list
                );
            } else {
                println!(" {}. \"{}\" via crates: {}", i + 1, &team.login, crate_list);
            }
        }
        println!("\nGithub teams are black boxes. It's impossible to get the member list without explicit permission.");
    }
    Ok(())
}

/// Turns a crate-to-publishers mapping into publisher-to-crates mapping.
/// BTreeMap is used because PublisherData doesn't implement Hash.
fn transpose_publishers_map(
    input: &HashMap<String, Vec<PublisherData>>,
) -> BTreeMap<PublisherData, Vec<String>> {
    let mut result: BTreeMap<PublisherData, Vec<String>> = BTreeMap::new();
    for (crate_name, publishers) in input.iter() {
        for publisher in publishers {
            result
                .entry(publisher.clone())
                .or_default()
                .push(crate_name.clone());
        }
    }
    result
}

/// Returns a Vec sorted so that publishers are sorted by the number of crates they control.
/// If that number is the same, sort by login.
fn sort_transposed_map_for_display(
    input: BTreeMap<PublisherData, Vec<String>>,
) -> Vec<(PublisherData, Vec<String>)> {
    let mut result: Vec<_> = input.into_iter().collect();
    result.sort_unstable_by_key(|(publisher, crates)| {
        (usize::MAX - crates.len(), publisher.login.clone())
    });
    result
}
