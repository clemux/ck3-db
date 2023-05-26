use std::env;
use ck3save::{models::Gamestate, Ck3File, EnvTokens};
use diesel::RunQueryDsl;
use ck3::{establish_connection};
use ck3::models::{NewCharacter, NewFaith, NewHouse};
use ck3::schema::{characters, faiths, houses};

fn localize_name(data: &serde_yaml::Value, name: &str) -> String {
    match data["l_english"][name].as_str() {
        Some(n) => n.to_owned(),
        None => name.to_owned()
    }
}

fn localize_dynasty(data: &serde_yaml::Value, dynasty: &str) -> String {
    match data["l_english"][dynasty].as_str() {
        Some(s) => s.to_owned(),
        None => dynasty.to_owned()
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let data = std::fs::read(&args[1])?;

    // read save
    let file = Ck3File::from_slice(&data)?;
    let mut zip_sink = Vec::new();
    let file = file.parse(&mut zip_sink)?;
    let save: Gamestate = file.deserializer(&EnvTokens).deserialize()?;

    // read localization
    let f = std::fs::File::open("test.yaml")?;
    let localization_data: serde_yaml::Value = serde_yaml::from_reader(f)?;
    let dynasty_file = std::fs::File::open("dynasties.yaml")?;
    let dynasty_data: serde_yaml::Value = serde_yaml::from_reader(dynasty_file)?;


    println!("{:?}", save.living[&11104]);
    let name = save.living[&11104].first_name.as_ref().unwrap();
    println!("{}", localize_name(&localization_data, name));

    // feed data to db
    let connection = &mut establish_connection();
    let characters: Vec<NewCharacter> = save.living.into_iter().map(
        |(x,y)| NewCharacter {
            id: x as i32,
            first_name: y.first_name.as_ref().map(|name| localize_name(&localization_data, name)),
            prowess: y.skill[5] as i32,
            faith_id: y.faith.map(|f| f as i32),
            house_id: y.dynasty_house.map(|id| id as i32),
        }
    ).collect();
    diesel::insert_into(characters::table)
        .values(characters)
        .execute(connection)
        .expect("Error");

    let faiths: Vec<NewFaith> = save.religion.faiths.into_iter().map(
        |(x,y)| NewFaith {
            id: x as i32,
            tag: y.tag,
        }
    ).collect();
    diesel::insert_into(faiths::table)
        .values(faiths)
        .execute(connection)
        .expect("Error");

    let houses: Vec<NewHouse> = save.dynasties.dynasty_house.into_iter().map(
        |(x,y)| NewHouse {
            id: x as i32,
            name: y.name.map(|d| localize_dynasty(&dynasty_data, d.as_ref()))
        }
    ).collect();
    diesel::insert_into(houses::table)
        .values(houses)
        .execute(connection)
        .expect("Error");


    Ok(())
}
