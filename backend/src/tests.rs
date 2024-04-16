use crate::{database::Database, models, utils};
use fake::{
    faker::{address, internet},
    Fake,
};
use futures::future::join_all;
use rand::Rng;
use std::ops::Range;

#[actix_rt::test]
async fn create_users() {
    let mut futures = Vec::new();
    let database = Database::new();

    for _ in 0..100 {
        let database = database.clone();
        futures.push(async move {
            let mut new_player = models::Player::default();
            new_player.time_in_game = (0.0..=1000.0).fake();
            let player_id = database.create_player(new_player).await.unwrap();
            let password: String = internet::en::Password(Range { start: 8, end: 16 }).fake();

            let mut rng = rand::thread_rng();
            let user_type: models::UserType = rng.gen();

            let mut rng = rand::thread_rng();
            let gender: models::Gender = rng.gen();

            let user = models::User {
                id: None,
                user_type,
                username: internet::en::Username().fake(),
                email: internet::en::SafeEmail().fake(),
                password: utils::hash_password(&password).unwrap().to_string(),
                gender,
                os: utils::get_os(internet::en::UserAgent().fake()),
                player_id,
                latitude: address::en::Latitude().fake(),
                longitude: address::en::Longitude().fake(),
                year_of_birth: (1920..=2003).fake(),
                role_id: models::RoleType::User.to_string(),
            };

            database.create_user(user).await.unwrap();
        });
    }

    join_all(futures).await;
}
