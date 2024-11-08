use std::io;
use std::fs::File;
use std::io::BufWriter;
use printpdf::*;
use std::collections::HashMap;

#[derive(Debug)]
struct User {
    name: String,
    role: String,
    pay: f64,
    strikes: u32,
    benched: bool,
}

fn main() {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("Luca".to_string(), User { name: "Luca".to_string(), role: "CEO".to_string(), pay: 100000.0, strikes: 0, benched: false });
    users.insert("Veer".to_string(), User { name: "Veer".to_string(), role: "Head of Operations".to_string(), pay: 6000.0, strikes: 0, benched: false });
    users.insert("Dre".to_string(), User { name: "Dre".to_string(), role: "Member".to_string(), pay: 5000.0, strikes: 0, benched: false });
    users.insert("Trixy".to_string(), User { name: "Arham".to_string(), role: "Member".to_string(), pay: 5000.0, strikes: 0, benched: false });
    users.insert("Dylan".to_string(), User { name: "Dylan".to_string(), role: "Coach".to_string(), pay: 5000.0, strikes: 0, benched: false });

    println!("Welcome to the user management system!");

    loop {
        println!("\nCommands:");
        println!("1. add strike - Add a strike to a user");
        println!("2. remove strike - Remove a strike from a user");
        println!("3. change role - Change a user’s role");
        println!("4. set pay - Set user pay");
        println!("5. set bench - Set bench status for a user");
        println!("6. view users - View all users");
        println!("7. export - Export data to PDF");
        println!("8. exit - Exit the terminal");

        let mut cmd = String::new();
        println!("\nEnter a command:");
        io::stdin().read_line(&mut cmd).expect("Failed to read line");
        let cmd = cmd.trim();

        match cmd {
            "add strike" => add_strike(&mut users),
            "remove strike" => remove_strike(&mut users),
            "change role" => change_role(&mut users),
            "set pay" => set_pay(&mut users),
            "set bench" => set_bench(&mut users),
            "view users" => view_users(&users),
            "export" => export_to_pdf(&users),
            "exit" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid command, please try again."),
        }
    }
}

fn add_strike(users: &mut HashMap<String, User>) {
    let mut user_name = String::new();
    println!("Enter the name of the user to add a strike: ");
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    let user_name = user_name.trim();

    if let Some(user) = users.get_mut(user_name) {
        user.strikes += 1;
        println!("Strike added to {}. Total strikes: {}", user.name, user.strikes);
    } else {
        println!("User not found.");
    }
}

fn remove_strike(users: &mut HashMap<String, User>) {
    let mut user_name = String::new();
    println!("Enter the name of the user to remove a strike: ");
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    let user_name = user_name.trim();

    if let Some(user) = users.get_mut(user_name) {
        if user.strikes > 0 {
            user.strikes -= 1;
            println!("Strike removed from {}. Total strikes: {}", user.name, user.strikes);
        } else {
            println!("No strikes to remove for {}.", user.name);
        }
    } else {
        println!("User not found.");
    }
}

fn change_role(users: &mut HashMap<String, User>) {
    let mut user_name = String::new();
    println!("Enter the name of the user to change role: ");
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    let user_name = user_name.trim();

    let mut new_role = String::new();
    println!("Enter new role: ");
    io::stdin().read_line(&mut new_role).expect("Failed to read line");
    let new_role = new_role.trim();

    if let Some(user) = users.get_mut(user_name) {
        user.role = new_role.to_string();
        println!("Role of {} updated to {}", user.name, user.role);
    } else {
        println!("User not found.");
    }
}

fn set_pay(users: &mut HashMap<String, User>) {
    let mut user_name = String::new();
    println!("Enter the name of the user to set pay: ");
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    let user_name = user_name.trim();

    let mut pay = String::new();
    println!("Enter pay amount: ");
    io::stdin().read_line(&mut pay).expect("Failed to read line");

    if let Some(user) = users.get_mut(user_name) {
        if let Ok(pay) = pay.trim().parse::<f64>() {
            user.pay = pay;
            println!("Pay of {} updated to ${}", user.name, user.pay);
        } else {
            println!("Invalid pay amount.");
        }
    } else {
        println!("User not found.");
    }
}

fn set_bench(users: &mut HashMap<String, User>) {
    let mut user_name = String::new();
    println!("Enter the name of the user to set bench status: ");
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    let user_name = user_name.trim();

    let mut status = String::new();
    println!("Enter bench status (yes/no): ");
    io::stdin().read_line(&mut status).expect("Failed to read line");
    let status = status.trim().eq_ignore_ascii_case("yes");

    if let Some(user) = users.get_mut(user_name) {
        user.benched = status;
        println!("Benched status of {} updated to {}", user.name, if user.benched { "Yes" } else { "No" });
    } else {
        println!("User not found.");
    }
}

fn view_users(users: &HashMap<String, User>) {
    println!("\nUser Data:");
    for (_name, user) in users {
        println!("{:?}", user);
    }
}

fn export_to_pdf(users: &HashMap<String, User>) {
    let (doc, page1, layer1) = PdfDocument::new("User Data", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).expect("Error adding font");

    current_layer.use_text("User Data Report", 20.0, Mm(10.0), Mm(280.0), &font);

    let mut y_position = 260.0;
    for user in users.values() {
        current_layer.use_text(
            format!("Name: {}, Role: {}, Pay: ${:.2}, Strikes: {}, Benched: {}",
                user.name, user.role, user.pay, user.strikes, if user.benched { "Yes" } else { "No" }
            ),
            12.0, Mm(10.0), Mm(y_position), &font,
        );
        y_position -= 10.0;
    }

    let file = File::create("user_data_report.pdf").expect("Error creating PDF file");
    doc.save(&mut BufWriter::new(file)).expect("Error saving PDF file");

    println!("Data exported to user_data_report.pdf");
}
