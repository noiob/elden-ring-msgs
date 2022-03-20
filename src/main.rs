// extern crate mammut;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate elefren;

use std::error::Error;
use elefren::prelude::*;
use elefren::helpers::toml;
use elefren::helpers::cli;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use rand::prelude::IteratorRandom;
use rand::{thread_rng, Rng};

fn main() -> Result<(), Box<dyn Error>> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register()?
    };

    let phrase_path = Path::new("ermessages.txt");
    let phrase_file = BufReader::new(File::open(&phrase_path).unwrap()).lines();

    let conj_path = Path::new("erconjunctions.txt");
    let conj_file = BufReader::new(File::open(&conj_path).unwrap()).lines();

    let bmsg_path = Path::new("bmsg.txt");
    let bmsg_file = BufReader::new(File::open(&bmsg_path).unwrap()).lines();

    let mut rng = thread_rng();

    let post;

    if rng.gen() { // single-line message
        let bmsg = bmsg_file.choose(&mut rng).unwrap().unwrap();
        let phrase = phrase_file.choose(&mut rng).unwrap().unwrap();
        post = bmsg.replace("<?bmsg?>", &phrase);
    }
    else { // two-line message
        let conj = conj_file.choose(&mut rng).unwrap().unwrap();
        let bmsgs = bmsg_file.choose_multiple(&mut rng, 2);
        let phrases = phrase_file.choose_multiple(&mut rng, 2);
        let line1 = bmsgs[0].as_ref().unwrap().replace("<?bmsg?>", &phrases[0].as_ref().unwrap());
        let line2 = bmsgs[1].as_ref().unwrap().replace("<?bmsg?>", &phrases[1].as_ref().unwrap());

        if conj == "," {
            post = format!("{}{}\n{}", line1, conj, line2);
        }
        else {
            post = format!("{}\n{}{}", line1, conj, line2);
        }
    }

    println!("{}", post);

    let the_status = elefren::status_builder::StatusBuilder::new()
        .status(post.clone())
        .visibility(elefren::status_builder::Visibility::Public)
        .build()?;
    let post = mastodon.new_status(the_status);
    println!("{}", post.unwrap().url.unwrap());

    Ok(())
}

fn register() -> Result<Mastodon, Box<dyn Error>> {
    let registration = Registration::new("https://botsin.space")
                                    .client_name("Tarnished's Wizened Finger")
                                    .scopes(Scopes::all())
                                    .build()?;
    let mastodon = cli::authenticate(registration)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}
