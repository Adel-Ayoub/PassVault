use crate::password;
use crate::password::Password;
use md5;
use std::fs;
use std::io::stdin;
use std::io::Write;
use std::path::Path;

pub fn banner(version: f64) {
    println!(
        "MyPass password manager version {} \n\
        A simple and powerfull password manager \n\
        \nUsage: mypass [OPTIONS...] \n\
            \
        \t-k Add the key \n\
        \t-a Add a new password \n\
        \t-n Set name for your new password \n\
        \t-d Set description for your new password \n\
        \t-A Add a new password with TUI menu \n\
        \t-l List ALl of your password by their name \n\
        \t-L List all of your password by their name and description and date \n\
        \t-v get the version of this program \n\
        \t-m modify password information",
        version,
    )
}

// PSM Passwords Folder Struction:

//  .pass folder to store all the passwords that each password has
//  its own folder with the data and meta files.
//  For example:

//  ===================================
//  .pass/
//  |- instagram/
//  |  |- data - data file that stores things like password and etc.
//  |  |- meta - metadata file that stores the ascii base meta data.
//  |- TikTok/
//  |  |- data - ~
//  |  |- meta - ~
//  |- Spacehey/
//  |  |- data - ~
//  |  |- meta - ~
// ====================================

// The data file is the main file that stores the encrypted password
// And if there is any metadata file, the needed data will be store
// on the same file. The data file will store password and metadata
// by order.

// The metadata file is the information file that use to store the
// extra information about the password, like description and date.
// Notice that the name of the password will be the same as the name
// of the parent folder. with that said we don't need to store the name
// of the password in the meta file anymore.

pub fn get_hash(key: &str) -> String {
    // Do the hash logic
    format!("{:x}", md5::compute(key))
}

// Add password to the origin
pub fn origin_add(password: &password::Password) -> Result<(), String> {
    if !is_data_exists(&password.name) {
        if let Err(_) = fs::create_dir(format!("./pass/{}", &password.name)) {
            return Err(format!("[!] Can't find the password folder"));
        }
        // Make the meta file for saving metadata about the origin
    } else {
        // TODO: Ask the user to agree the existence of password
        let answer = Ask(format!(
            "You are overriting the current content of {} password, are you ok? [Y,n]? ",
            &password.name
        )
        .as_str())?;

        if !answer {
            return Err(format!("Stop process by user input"));
        }
        // println!("[!] notice you are updating the exists password");
    }

    let data: String = format!("{}", password.value.clone());
    let meta: String = format!("{}\n{}", password.description, password.date);

    let file_data = fs::File::create(format!("./pass/{}/data", &password.name));
    if let Err(_) = file_data {
        return Err(format!("Can't make data file_data"));
    }

    let res = file_data.unwrap().write_all(data.as_bytes());
    if let Err(_) = res {
        return Err(format!("Can't write to the file_data"));
    }

    let file_meta = fs::File::create(format!("./pass/{}/meta", &password.name));
    if let Err(_) = file_meta {
        return Err(format!("Can't make meta)) file"));
    }

    let res = file_meta.unwrap().write_all(meta.as_bytes());
    if let Err(_) = res {
        return Err(format!("Can't write to the file meta"));
    }

    Ok(())
}
// Show password from origin
pub fn origin_show(name: &String) -> Result<password::Password, String> {
    let mut u_pass = password::Password::default();

    // Check for existing of password
    if !is_data_exists(name) {
        // TODO: Ask the user if wants to make a new password with the same name
        return Err(format!("Can't find any password with that name."));
    }

    // Read the data and meta files and Check for data/meta readabliti
    let data = fs::read(format!("./pass/{}/data", name));
    if let Err(err) = data {
        return Err(err.to_string());
    }

    // Read the meta file
    let meta = fs::read(format!("./pass/{}/meta", name));
    if let Err(err) = meta {
        return Err(err.to_string());
    }

    // Shadowing the data and meta
    let data = String::from_utf8(data.unwrap());
    let meta = String::from_utf8(meta.unwrap());

    // TOOD: Better warning handeling
    if let Err(err) = data {
        return Err(err.to_string());
    } else if let Err(err1) = meta {
        return Err(err1.to_string());
    }

    let data = data.unwrap();
    let meta = meta.unwrap();

    // Parse them into u_pass

    let l_data = data.split("\n").collect::<Vec<&str>>();
    let d_pass = l_data[0]; // First line name

    let l_meta: Vec<&str> = meta.split("\n").collect();
    let description = l_meta[0];
    let date = l_meta[1];

    u_pass.name = String::from(name);
    u_pass.description = String::from(description);
    u_pass.value = String::from(d_pass);
    u_pass.date = String::from(date);

    // return the pass obj
    Ok(u_pass)
}

pub fn list_origin() -> Result<Vec<Password>, String> {
    let list_res: Vec<Password> = Vec::new();
    let mut counter = 0;

    // get the files from origin
    if let Ok(v) = fs::read_dir("./pass") {
        println!("This is the saved passwords list: \n");
        for items in v {
            if let Err(_) = items {
                return Err(format!("Error when reading origin passwords"));
            }
            let item = items.unwrap();
            // Read only the folders
            if item.path().is_dir() {
                println!("{}. {:?}", counter + 1, item.file_name());
            }
            // TODO: Show more information about the password
            counter += 1;
        }
    } else {
        return Err(format!("Can't read from origin"));
    }

    Ok(list_res)
}

pub fn Ask(text: &str) -> Result<bool, String> {
    // TODO: The print is not working. idk...
    println!("[?] {}", text);
    let mut input: String = String::new();
    let res = stdin().read_line(&mut input).ok();
    match res {
        Some(_) => {
            // TODO: Better way of removing new line
            if &input.to_lowercase().as_str()[0..input.len() - 2] == "y" {
                return Ok(true);
            }
            return Ok(false);
        }
        None => return Err(format!("Can't read the user input")),
    }
}

pub fn AskStr(text: &str) -> Result<String, String> {
    // TODO: The print is not working. idk...
    println!("[?] {}", text);
    let mut input: String = String::new();
    let res = stdin().read_line(&mut input).ok();
    input = String::from(&input.as_str()[0..input.len() - 2]);
    match res {
        Some(_) => {
            // TODO: Better way of removing new line
            return Ok(input);
        }
        None => return Err(format!("Can't read the user input")),
    }
}

pub fn is_data_exists(name: &str) -> bool {
    if Path::new(format!("./pass/{}", name).as_str()).exists() {
        return true;
    }
    return false;
}

// Check is there is any global .pass folder
pub fn is_origin_exists() -> bool {
    if Path::new("./pass").exists() {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample: String = String::from("This is the text\nThis is the new line");
        let l_data = sample.split("\n").collect::<Vec<&str>>();
        println!("This is the first line {}", l_data[0]);
    }
}
