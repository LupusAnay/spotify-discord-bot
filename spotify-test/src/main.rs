use std::env;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::player::Player;

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    let username = env::var("SPOTIFY_USERNAME").unwrap();
    let password = env::var("SPOTIFY_PASSWORD").unwrap();
    let track = env::var("SPOTIFY_TRACK").unwrap();

    let credentials = Credentials::with_password(username, password);

    let track = SpotifyId::from_base62(&*track).unwrap();

    let backend = audio_backend::find(Some(String::from("gstreamer"))).unwrap();

    println!("Connecting ..");
    let session = Session::connect(session_config, credentials, None)
        .await
        .unwrap();

    let (mut player, _) = Player::new(player_config, session, None, move || {
        backend(Some(String::from(r#" ! audioconvert ! vorbisenc ! oggmux ! filesink location=sine.ogg"#)), audio_format)
    });

    player.load(track, true, 0);

    println!("Playing...");

    player.await_end_of_track().await;

    println!("Done");
}
