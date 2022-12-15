fn main() {
    let user1 = User {
        _email: String::from("joe@joemail.com"),
        _username: String::from("bananajoe123"),
        _active: true,
        _sign_in_count: 1,
    };

    let _user2 = User {
        _email: String::from("deher@denher.demher"),
        ..user1
    };

    let _black = Color (0, 0, 0);
    let _origin = Point (0, 0, 0);

    let _subject = AlwaysEqual;
    //because of heap and stack datamanegement, after reusing the username String from user1 in user2, user1 perishes. 
    //Had i updated both Strings in the struct, both would still be usable.
}

struct User {
    _active: bool, 
    _username: String,
    _email: String, 
    _sign_in_count: u64,
}

struct Color (i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn _build_user(_email: String, _username: String) -> User {
    return User {
            _email,
            _username,
            _active: true,
            _sign_in_count: 1,
    };
}