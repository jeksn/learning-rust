
enum Living {
    Alive,
    Dead,
}

enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

struct Human {
    name: String,
    state: Living,
    home: Planet,
}

fn main() {
    let user = Human {
        name: "Johan".to_string(),
        state: Living::Alive,
        home: Planet::Earth,
    };

    println!("{}", user.name);
}
