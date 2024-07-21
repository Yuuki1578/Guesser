mod util;

use util::{debug_utils::*, game_utils::*, STATUS, VERSION};

use std::io::{stdout, Error, ErrorKind, Write};

use std::cmp::Ordering;

pub fn main() -> Result<(), Error> {
    let mut args = Argument::new();

    args.get(2);
    args.remove(0);

    let limits = &args.expose(0)[..];
    let _sliced = if args.expose(0).len() < 7 {
        ""
    } else {
        &args.expose(0)[..7]
    };

    match &args.expose(0)[..] {
        "--help" | "-h" => {
            println!("{}", helper());
            Ok(())
        }

        "--version" | "-v" => {
            println!("Version: {VERSION}\nStatus:  {STATUS}");
            Ok(())
        }

        "--play" | "-p" => {
            println!("{}", start_default());
            let secret_number = random::<usize>(1..=100);

            loop {
                print!("stdin> ");
                stdout().flush()?;

                let user_input = input();
                let user_input = match user_input.trim().parse::<usize>() {
                    Ok(inputted) => inputted,
                    Err(_) => {
                        eprintln!("\n{}", parse_error(&user_input[..]));
                        continue;
                    }
                };

                match user_input.cmp(&secret_number) {
                    Ordering::Less => {
                        eprintln!(
                            "\n{}",
                            "Status: Too Small\n".to_string() + "Action: Continue\n"
                        );

                        continue;
                    }
                    Ordering::Greater => {
                        eprintln!(
                            "\n{}",
                            "Status: Too Big\n".to_string() + "Action: Continue\n"
                        );

                        continue;
                    }
                    Ordering::Equal => {
                        println!(
                            "\n{}",
                            "Status:  Equal\n".to_string()
                                + "Action:  Break\n"
                                + "Message: Congratulations! You Win!\n"
                        );

                        break;
                    }
                }
            }

            Ok(())
        }

        _sliced => {
            if _sliced.len() < 1{
                eprintln!("{}", helper());
                Ok(())
            } else {
                let parsed = parser_usize(limits);
                let secret_number = random::<usize>(1..=parsed);

                if parsed == 0 {
                    return Err(Error::new(ErrorKind::Other, "error: limit must greater than 0"));
                }

                println!("{}", start_usize(parsed));

                loop {
                    print!("stdin> ");
                    stdout().flush()?;

                    let user_input = input();
                    let user_input = match user_input.trim().parse::<usize>() {
                        Ok(inputted) => inputted,
                        Err(_) => {
                            eprintln!("\n{}", parse_error(&user_input[..]));
                            continue;
                        }
                    };

                    match user_input.cmp(&secret_number) {
                        Ordering::Less => {
                            eprintln!(
                                "\n{}",
                                "Status: Too Small\n".to_string() + "Action: Continue\n"
                            );

                            continue;
                        }
                        Ordering::Greater => {
                            eprintln!(
                                "\n{}",
                                "Status: Too Big\n".to_string() + "Action: Continue\n"
                            );

                            continue;
                        }
                        Ordering::Equal => {
                            println!(
                                "\n{}",
                                "Status:  Equal\n".to_string()
                                    + "Action:  Break\n"
                                    + "Message: Congratulations! You Win!\n"
                            );

                            break;
                        }
                    }
                }
                Ok(())
            }
        }
    }
}
