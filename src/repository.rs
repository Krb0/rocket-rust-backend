
pub fn initial_setup(){

    let connection = sqlite::open("database/users.sqlite").unwrap();
    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Juan', 10);
        INSERT INTO users VALUES ('Pedro', 12);
        INSERT INTO users VALUES ('Pepe', 92);
        INSERT INTO users VALUES ('Roberto', 50);
    ";
    connection.execute(query).unwrap();
}