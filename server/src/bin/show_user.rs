use server::db::*;

fn main() {
    let binding = get_user();
    let users = binding.iter().enumerate();
    let users_count = users.len();

    println!("Displaying {} users", users_count);
    println!("");

    for (i, user) in users {
        println!("User {}", i + 1);
        println!("Username: {}", user.username);
        println!("Hashed Password: {}", user.password);

        if i + 1 != users_count {
            println!();
        }
    }
}
