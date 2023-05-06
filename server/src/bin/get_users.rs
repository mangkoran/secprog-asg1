use server::db::*;

fn main() {
    let binding = get_users().expect("Error getting users");
    let users = binding.iter().enumerate();
    let users_count = users.len();

    println!("Displaying {} users", users_count);
    println!("");

    for (i, user) in users {
        println!("User {}", i + 1);
        println!("Username: {}", user.username);
        println!("Password Hash: {}", user.password);

        if i + 1 != users_count {
            println!();
        }
    }
}
