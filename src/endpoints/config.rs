

use aide::axum::ApiRouter;

use crate::endpoints::tracks;
use crate::endpoints::players;
use crate::endpoints::leaderboard;
use crate::endpoints::health;


pub fn create_endpoints() -> ApiRouter {

    let api = ApiRouter::new()
    .merge(tracks::create_endpoints())
    .merge(players::create_endpoints())
    .merge(leaderboard::create_endpoints())
    .merge(health::create_endpoints());

    api
}