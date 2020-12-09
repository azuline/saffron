use crate::commands::{Create, Reset};
use crate::config::Config;
use crate::models::{users, User};

pub async fn create_user(opts: Create, config: Config) {
    let nickname = opts.nickname;
    let token = users::generate_token();

    User::create(&config.db_pool, &nickname, &token)
        .await
        .expect("Database error--failed to create user.");

    println!(
        "Created user {} with token: {}",
        &nickname,
        hex::encode(&token)
    );
}

pub async fn reset_user(opts: Reset, config: Config) {
    let user = match User::from_id(&config.db_pool, opts.user_id).await {
        Some(user) => user,
        None => {
            println!("No user found with ID {}.", opts.user_id);
            return;
        }
    };

    let new_token = users::generate_token();

    let user = user
        .update_token(&config.db_pool, &new_token)
        .await
        .expect("Database error--failed to reset token.");

    println!(
        "Reset token of user {} ({}) to: {}",
        user.id,
        &user.nickname,
        hex::encode(&new_token)
    );
}

pub async fn list_users(config: Config) {
    let users = User::all(&config.db_pool)
        .await
        .expect("Database error--failed querying users.");

    for user in users.iter() {
        println!("User {}: {}", user.id, &user.nickname);
    }
}
