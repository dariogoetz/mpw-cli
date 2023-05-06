use clap::{App, Arg};
use colored::*;
use env_logger;
use rpassword;

use mpw::identicon::{Color, Identicon};
use mpw::masterkey::MasterKey;
use mpw::password_type::PasswordType;

fn main() {
    let _ = env_logger::try_init();
    let matches = App::new("MPW password generator")
        .version("0.1.0")
        .author("Dario Goetz <dario.goetz@googlemail.com>")
        .about("Generates passwords using the masterpasswordapp algorithm")
        .arg(
            Arg::with_name("name")
                .index(1)
                .short("n")
                .long("full-name")
                .value_name("full_name")
                .help("Full name")
                .required(true),
        )
        .arg(
            Arg::with_name("site")
                .index(2)
                .short("s")
                .long("site-name")
                .value_name("site_name")
                .help("Name of site to generate password for"),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("password-type")
                .value_name("pw_type")
                .default_value("Maximum")
                .help("Type of password")
                .possible_values(&[
                    "Maximum", "Long", "Medium", "Short", "Basic", "PIN", "Name", "Phrase",
                ]),
        )
        .get_matches();

    let password_type: PasswordType = matches.value_of("type").unwrap().into();
    let full_name = matches.value_of("name").unwrap();
    let password = rpassword::prompt_password_stdout("Master Password: ").unwrap();

    let id = Identicon::new(&full_name, &password);
    let id_str = match id.color {
        Color::Red => id.to_string().red(),
        Color::Green => id.to_string().green(),
        Color::Yellow => id.to_string().yellow(),
        Color::Blue => id.to_string().blue(),
        Color::Magenta => id.to_string().magenta(),
        Color::Cyan => id.to_string().cyan(),
        Color::White => id.to_string().white(),
    };
    println!("Identity: {}", id_str);
    let masterkey = MasterKey::new_auth(&full_name, &password);

    let mut one_shot = false;
    loop {
        let site_name = match matches.value_of("site") {
            Some(site_name) => {
                one_shot = true;

                site_name.to_string()
            }
            None => {
                println!("Please enter site name:");
                let mut site_name = String::new();
                std::io::stdin()
                    .read_line(&mut site_name)
                    .expect("Could not read site key.");

                site_name
            }
        };
        let site_name = site_name.trim();
        let password = masterkey.generate_password(site_name, &password_type, 1);
        println!("The password is: {}", password.green().bold());

        if one_shot {
            break;
        } else {
            println!();
        }
    }
}
