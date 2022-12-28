
pub fn main(){

    let connection = sqlite::open("test.sqlite").unwrap();

let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
";
connection.execute(query).unwrap();
}