use rspotify::model::{PlayableItem, FullTrack};

fn main() {
    let json_str = r#"
    {
        "album": {
            "album_type": "album",
            "artists": [
                {
                    "external_urls": { "spotify": "https://open.spotify.com/artist/3h100hRlLZ7QDz8GRt5QsD" },
                    "href": "https://api.spotify.com/v1/artists/3h100hRlLZ7QDz8GRt5QsD",
                    "id": "3h100hRlLZ7QDz8GRt5QsD",
                    "name": "Lullatone",
                    "type": "artist",
                    "uri": "spotify:artist:3h100hRlLZ7QDz8GRt5QsD"
                }
            ],
            "external_urls": { "spotify": "https://open.spotify.com/album/3AWATaNWvlDlTOmlemNQDw" },
            "href": "https://api.spotify.com/v1/albums/3AWATaNWvlDlTOmlemNQDw",
            "id": "3AWATaNWvlDlTOmlemNQDw",
            "images": [
                { "height": 640, "url": "https://i.scdn.co/image/ab67616d0000b2733793d98ceea14ba5648a70f9", "width": 640 },
                { "height": 300, "url": "https://i.scdn.co/image/ab67616d00001e023793d98ceea14ba5648a70f9", "width": 300 },
                { "height": 64, "url": "https://i.scdn.co/image/ab67616d000048513793d98ceea14ba5648a70f9", "width": 64 }
            ],
            "name": "Thinking About Thursdays",
            "release_date": "2017-02-21",
            "release_date_precision": "day",
            "total_tracks": 52,
            "type": "album",
            "uri": "spotify:album:3AWATaNWvlDlTOmlemNQDw"
        },
        "artists": [
            {
                "external_urls": { "spotify": "https://open.spotify.com/artist/3h100hRlLZ7QDz8GRt5QsD" },
                "href": "https://api.spotify.com/v1/artists/3h100hRlLZ7QDz8GRt5QsD",
                "id": "3h100hRlLZ7QDz8GRt5QsD",
                "name": "Lullatone",
                "type": "artist",
                "uri": "spotify:artist:3h100hRlLZ7QDz8GRt5QsD"
            }
        ],
        "disc_number": 1,
        "duration_ms": 186000,
        "explicit": false,
        "external_urls": { "spotify": "https://open.spotify.com/track/6TKd6LlPgJEJErUS0I3iMv" },
        "href": "https://api.spotify.com/v1/tracks/6TKd6LlPgJEJErUS0I3iMv",
        "id": "6TKd6LlPgJEJErUS0I3iMv",
        "is_local": false,
        "name": "Studying a Pinecone",
        "preview_url": null,
        "track_number": 47,
        "type": "track",
        "uri": "spotify:track:6TKd6LlPgJEJErUS0I3iMv"
    }
    "#;

    match serde_json::from_str::<FullTrack>(json_str) {
        Ok(_) => println!("FullTrack: OK"),
        Err(e) => println!("FullTrack: ERR: {}", e),
    }

    match serde_json::from_str::<PlayableItem>(json_str) {
        Ok(PlayableItem::Track(_)) => println!("PlayableItem: Track"),
        Ok(PlayableItem::Episode(_)) => println!("PlayableItem: Episode"),
        Ok(PlayableItem::Unknown(_)) => println!("PlayableItem: Unknown"),
        Err(e) => println!("PlayableItem: ERR: {}", e),
    }
}
