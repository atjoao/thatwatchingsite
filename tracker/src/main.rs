mod structs;
mod nyaa_si;

fn main() {
    let anime = "shikanoko";

    let nyaa_results = nyaa_si::fetch(anime).unwrap();

    println!("{:#?}", nyaa_results)
}

