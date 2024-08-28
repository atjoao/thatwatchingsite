mod structs;
mod nyaa_si;

fn main() {
    let nyaa_results = nyaa_si::fetch().unwrap();

    println!("{:#?}", nyaa_results)
}

