use std::io::{self, Write};

use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Class;
use text_io::read;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .cookie_store(true)
        .build()?;

    print!("Username: ");
    io::stdout().flush().unwrap();
    let username: String = read!();
    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    let raw_login_html = login(&client, &username, &password).unwrap();
    let login_html = raw_login_html.as_str();
    let course_identifier = course_identifier(&login_html).unwrap();
    
    select_course(&client, &course_identifier);
    
    let subjects_identifiers = subjects_identifiers(&client).unwrap();
    let notes: Vec<f32> = subjects_identifiers.iter()
        .map(|subject_identifier| subject_note(&client, &subject_identifier, &course_identifier).unwrap())
        .map(|note| note.parse::<f32>().unwrap_or(0.0))
        .collect();    
    let subjects_names = subjects_names(&client).unwrap();
    
    subjects_names.iter().zip(notes.iter())
        .for_each(|subject_note| println!("{:#?}", subject_note));

    let total = notes.iter().sum::<f32>() as f32;
    let num_completed_subjects: f32 = notes.iter().filter(|x| **x != 0.0).fold(0.0, |count, _note| count + 1.0);
    println!("Average divided by all completed the subjects {:#?}", total / num_completed_subjects);

    let num_subjects = notes.len() as f32;
    println!("Average divided by all the subjects (incompleted included) {:#?}", total / num_subjects);

    Ok(())
}


fn login(client: &Client, username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(client
        .post("https://mytechspace.talent.upc.edu/index.php")
        .form(&[("username", &username), ("password", &password)])
        .send()?
        .text()?)
}

fn course_identifier(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let course_identifier_regex = regex::Regex::new(r"\d+").unwrap();

    Ok(Document::from(html)
        .find(Class("Arial11BlackBold"))
        .filter(|element| element.attr("onclick") != None)
        .map(|clickable| course_identifier_regex.captures(clickable.attr("onclick").unwrap()))
        .map(|identifier| identifier.unwrap()[0].to_string())
        .collect())
}

fn select_course(client: &Client, identifier: &str) -> Result<(), Box<dyn std::error::Error>> {
    let activity_page = "https://mytechspace.talent.upc.edu/activitat.php?id_activitat=";
    let activity_with_id_url = format!("{}{}", activity_page, identifier);
    client
        .get(activity_with_id_url)
        .send()?;

    Ok(())
}

fn subjects_identifiers(client: &Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = client
        .get("https://mytechspace.talent.upc.edu/aules.php")
        .send()?
        .text()?;

    let subject_identifier_regex = regex::Regex::new(r"T\d+").unwrap();
    let subjects_id: Vec<String> = Document::from(response.as_str())
        .find(Class("Arial11BlueBold"))
        .filter(|element| element.attr("onclick") != None)
        .map(|clickable| clickable.attr("onclick").unwrap())
        .filter(|clickable| subject_identifier_regex.is_match(clickable))
        .map(|identifier| subject_identifier_regex.captures(identifier).unwrap())
        .map(|identifier| identifier[0].to_string())
        .collect();

    Ok(subjects_id)
}

fn subjects_names(client: &Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = client
        .get("https://mytechspace.talent.upc.edu/aules.php")
        .send()?
        .text()?;

    let subject_identifier_regex = regex::Regex::new(r"T\d+").unwrap();
    let subjects_names: Vec<String> = Document::from(response.as_str())
        .find(Class("Arial11BlueBold"))
        .filter(|element| element.attr("onclick") != None)
        .filter(|clickable| subject_identifier_regex.is_match(clickable.attr("onclick").unwrap()))
        .map(|identifier| identifier.children().next().unwrap().text())
        .collect();


    Ok(subjects_names)
}

fn subject_note(client: &Client, subject_identifier: &str, course_identifier: &str) -> Result<String, Box<dyn std::error::Error>> {
    let subject_url = format!("{}{}{}{}", "https://mytechspace.talent.upc.edu/qualificacions.php?id_aula=", subject_identifier, "&id_activitat=", course_identifier);

    let response = client
        .get(subject_url)
        .send()?
        .text()?;

    Ok(Document::from(response.as_str())
        .find(Class("Arial10BlackBold"))
        .map(|element| element.children().next().unwrap().text())
        .filter(|row_value| row_value.chars().next().unwrap().is_digit(10))
        .collect())
}