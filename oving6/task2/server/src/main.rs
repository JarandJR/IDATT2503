use actix_web::{get, post, web::Json, App, HttpServer};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server ..");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(user)
            .service(auth)
            .service(get_status)
    })
    .bind(("0.0.0.0", 7878))?
    .run()
    .await
}

#[post("/user")]
async fn user(user_json: String) -> String {
    println!("Creating user");
    let mut user = serde_json::from_str::<User>(&user_json).expect("Could not parse user");
    let mut hash = [0u8; 20];
        ring::pbkdf2::derive(
            ring::pbkdf2::PBKDF2_HMAC_SHA1,
            std::num::NonZeroU32::new(2048).unwrap(),
            user.username.as_bytes(),
            user.password.as_bytes(),
            &mut hash,
        );
    user.password = hash_pass(&user);
    save_user(user);
    String::from("User created!")
}

#[post("/auth")]
async fn auth(user_json: String) -> String {
    println!("Authenticating user");
    let user_auth = serde_json::from_str::<User>(&user_json).expect("Could not parse user");

    let file_contents = std::fs::read_to_string("user.txt").expect("Could not read user");
    let saved_user = serde_json::from_str::<User>(&file_contents).expect("Could not parse user from file");

    let pass = hash_pass(&user_auth);
    if pass == saved_user.password {
        return String::from("Successfull login!");
    }
    String::from("Password or username is wrong!")
}

#[get("/status")]
async fn get_status()  -> Json<String> {
    Json::from(Json(format!("Server is running")))
}

fn hash_pass(user_hash: &User) -> String {
    println!("Hashing password..");
    let mut hash = [0u8; 20];
        ring::pbkdf2::derive(
            ring::pbkdf2::PBKDF2_HMAC_SHA1,
            std::num::NonZeroU32::new(2048).unwrap(),
            user_hash.username.as_bytes(),
            user_hash.password.as_bytes(),
            &mut hash,
        );
    hex::encode(hash)
}

fn save_user(new_user: User) {
    println!("Saving user..");
    let user_string = serde_json::to_string(&new_user).expect("Could not serialize user to JSON");
    std::fs::write("user.txt", user_string).expect("Could not save user");
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}