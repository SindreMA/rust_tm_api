use paperclip::actix::web;

use crate::endpoints::tracks;
use crate::endpoints::players;
use crate::endpoints::leaderboard;
use crate::endpoints::health;


pub fn configure(cfg: &mut web::ServiceConfig) {
    tracks::configure(cfg);
    players::configure(cfg);
    leaderboard::configure(cfg);
    health::configure(cfg);
}