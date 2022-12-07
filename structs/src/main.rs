fn main() {
    let user1 = User {
        email: String::from("joe@joemail.com"),
        username: String::from("bananajoe123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("deher@denher.demher"),
        ..user1
    };

    //because of heap and stack datamanegement, after reusing the username String from user1 in user2, user1 perishes. 
    //Had i updated both Strings in the struct, both would still be usable.
}

struct User {
    active: bool, 
    username: String,
    email: String, 
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    return User {
            email,
            username,
            active: true,
            sign_in_count: 1,
    };
}