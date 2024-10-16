mod structs;
mod nyaa_si;

fn main() {
    let anime = "code geass lelouch of the rebellion";

    let nyaa_results = nyaa_si::fetch(anime).unwrap();

    println!("{:#?}", nyaa_results)
}

