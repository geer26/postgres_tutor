extern crate argon2;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Queryable)]

pub struct User {
    id: u32,
    username: String,
    password_hash: String,
    salt: String,
    active: bool,
}

impl User {

    fn gen_salt(&self) -> String{
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(26)
            .map(char::from)
            .collect();
        s
    }

    pub fn set_password(&mut self, password: String){
        use argon2::{self, Config};
        let password = password.as_bytes();
        let s = self.gen_salt();
        self.salt = s.clone();
        let salt = s.as_bytes();
        let config = Config::default();
        self.password_hash = argon2::hash_encoded(password, salt, &config).unwrap();
        //argon2::hash_encoded(password, salt, &config).unwrap()
    }

    pub fn check_password(&self, password: String) -> bool{
        use argon2::{self, Config};
        let living_password = &self.password_hash;
        let salt = self.salt.as_bytes();
        let try_password = password.as_bytes();
        let config = Config::default();
        let try_password_hash = argon2::hash_encoded(try_password, salt, &config).unwrap();
        living_password == &try_password_hash
    }

    pub fn new(username: String) -> Self{
        User{
            id: 0,
            password_hash: "".to_string(),
            salt: "".to_string(),
            username,
            active: true,
        }
    }

}