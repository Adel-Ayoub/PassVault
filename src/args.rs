use std::process::exit;

use crate::{util, VERSION};

#[derive(Default, Debug)]
pub struct CfgField {
    pub name: String,
    pub value: String,
    pub rank: usize,
}

impl CfgField {
    pub fn IsExists(&self) -> bool {
        return self.name != "";
    }
}

#[derive(Default, Debug)]
pub struct Config {
    init_origin: CfgField,
    new_password: CfgField,
    key: CfgField,
    show_password: CfgField,
    generate_password: CfgField,
    pub new_name: CfgField,
    new_description: CfgField,
    new_password_tui: CfgField,
    pub list_passwords: CfgField,
    resset_origin: CfgField,
}

// const ARGS: &'static [&'static str] = &["-a"];

impl Config {
    fn new() -> Self {
        Config::default()
    }
    pub fn IsNewPassword(&self) -> Option<&CfgField> {
        match self.new_password.IsExists() {
            true => Some(&self.new_password),
            false => None,
        }
    }

    pub fn IsInitOrigin(&self) -> Option<&CfgField> {
        match self.init_origin.IsExists() {
            true => Some(&self.init_origin),
            false => None,
        }
    }

    pub fn IsNewName(&self) -> Option<&CfgField> {
        match self.new_name.IsExists() {
            true => Some(&self.new_name),
            false => None,
        }
    }

    pub fn IsListPassword(&self) -> Option<&CfgField> {
        match self.list_passwords.IsExists() {
            true => Some(&self.list_passwords),
            false => None,
        }
    }

    pub fn IsRessetOrigin(&self) -> Option<&CfgField> {
        match self.resset_origin.IsExists() {
            true => Some(&self.resset_origin),
            false => None,
        }
    }

    pub fn IsShowPassword(&self) -> Option<&CfgField> {
        match self.show_password.IsExists() {
            true => Some(&self.show_password),
            false => None,
        }
    }

    pub fn IsGeneratePassword(&self) -> Option<&CfgField> {
        match self.generate_password.IsExists() {
            true => Some(&self.generate_password),
            false => None,
        }
    }

    pub fn IsKeyExists(&self) -> Option<&CfgField> {
        match self.key.IsExists() {
            true => Some(&self.key),
            false => None,
        }
    }

    pub fn IsNewDescription(&self) -> &CfgField {
        return &self.new_description;
    }

    pub fn IsNewPasswordTui(&self) -> &CfgField {
        return &self.new_password_tui;
    }
}

pub fn parse_arguments(args: &Vec<String>) -> Result<Config, String> {
    let mut cfg: Config = Config::new();
    let mut two: bool = false;
    let mut args_iter_obj = args[1..].iter().enumerate();

    if args.len() <= 1 {
        if let Ok(r) = util::AskStr("Find> ") {
            // Check the input for validity
            cfg.new_name = CfgField {
                name: String::from("-n"),
                value: String::from(r),
                rank: 0,
            };
        } else {
            return Err(format!(
                "Can't read the name of your password. please enter a valid name"
            ));
        }

        if let Ok(r) = util::AskStr("Key> ") {
            cfg.key = CfgField {
                name: String::from("-k"),
                value: String::from(r),
                rank: 1,
            };
        } else {
            return Err(format!(
                "Can't read the key of your password. please enter a valid key"
            ));
            // return None;
        }
        return Ok(cfg);
    }
    loop {
        if args_iter_obj.len() <= 0 {
            // println!("Ran out of index");
            break;
        }
        if (args_iter_obj.len() >= 2) {
            two = true;
        } else {
            two = false;
        }
        // let mut i: usize;
        let mut v: &String = &String::default();
        let (index, mut value) = args_iter_obj.next().expect("The Point 1");
        if value == "version" {
            println!("{}", VERSION);
            exit(0);
        }
        if !value.starts_with("-") {
            cfg.new_name = CfgField {
                name: String::from("-n"),
                value: String::from(value),
                rank: index,
            };
            if let Some((i3, v3)) = args_iter_obj.next() {
                cfg.key = CfgField {
                    name: String::from("-k"),
                    value: String::from(v3),
                    rank: i3,
                };
            } else {
                return Err(format!("[!] Please provid the key as the second option"));
            }
            break;
        }
        if two {
            loop {
                // println!("RIGHT");
                (_, v) = args_iter_obj.next().expect("The point 2");
                // TODO: Make chars of v's unwarap better
                if v.chars().nth(0).unwrap() == '-' {
                    value = v;
                    continue;
                } else {
                    break;
                }
            }
        }
        //println!("{} and {}", value, v);
        match value.as_str() {
            "-i" | "--init" => {
                println!("You provide the -a option");
                cfg.init_origin = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-a" | "--append" => {
                println!("You provide the -a option");
                cfg.new_password = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-n" | "--name" => {
                println!("You provide the -n option");
                cfg.new_name = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-d" | "--description" => {
                println!("You provide the -d option");
                cfg.new_description = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-l" | "--list" => {
                println!("You provide the -l option");
                cfg.list_passwords = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-k" | "--key" => {
                println!("You provide the -k option");
                cfg.key = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }

            "-g" | "--generate" => {
                println!("You provide the -g option");
                cfg.generate_password = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-r" | "--resset" => {
                println!("You provide the -r option");
                cfg.resset_origin = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            &_ => {
                return Err(format!(
                    "Please enter a valid command. You can see command list with `psm --help`"
                ));
            }
        }
    }
    Ok(cfg)
}
