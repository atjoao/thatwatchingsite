#[derive(Debug, Clone)]
pub struct Anime {
    pub group: String,
    pub name: String,
    pub quality: String,
    pub link: String,

    // batch is when the torrent contains all episodes and seasons
    pub batch: bool,
    
    // complete is when the torrent contains all episodes
    pub complete: bool,

    pub episode: String,
    pub season: String,

    // add seeders and etc..
}
