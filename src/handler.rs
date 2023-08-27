use tabled::settings::Style;

pub async fn add(){
    bunt::println!("Executing add command...");
    let key = inquire::Text::new("Enter Key: ").with_help_message("Enter any identifier").prompt().unwrap();
    let value = inquire::Text::new("Enter Value: ").with_help_message("Enter any value").prompt().unwrap();
    let hash = super::utils::random_hash();
    super::db::add(key.clone(), value.clone(), hash).await;
    bunt::println!("Added entry: {$green}{}{/$}", key);
    bunt::println!("Value: {$yellow}{}{/$}", value);
}

pub async fn list(){
    bunt::println!("Executing list command...");
    let entries = super::db::list().await.unwrap();
    let mut builder = super::utils::get_table();
    for entry in entries {
        builder.push_record(vec![
            entry.key,
            entry.value,
            entry.hash,
            // format datetime to human readable format
            chrono::DateTime::parse_from_rfc3339(&entry.created_at)
                .unwrap()
                .format("%a %b %e %T %Y")
                .to_string(),
        ]);
    }
    let table = builder.build().with(Style::rounded()).to_string();
    bunt::println!("{}", table);
}

pub async fn delete(){
    bunt::println!("Executing delete command...");
    let key = inquire::Text::new("Enter Key: ").with_help_message("Enter any identifier").prompt().unwrap();
    super::db::delete(key.clone()).await.unwrap();
    bunt::println!("Deleted entry: {$red}{}{/$}", key);
}

pub async fn get(){
    bunt::println!("Executing get command...");
    let key = inquire::Text::new("Enter Key: ").with_help_message("Enter any identifier").prompt().unwrap();
    let entry = super::db::get(key.clone()).await.unwrap();
    bunt::println!("Entry: {$green}{}{/$}", entry.key);
    bunt::println!("Value: {$yellow}{}{/$}", entry.value);
}

pub async fn search(){
    bunt::println!("Executing search command...");
    let keys = super::db::list_keys().await.unwrap();
    let key = inquire::Select::new("Select Key: ", keys).with_page_size(10).prompt().unwrap();
    let entry = super::db::get(key.clone()).await.unwrap();
    bunt::println!("Entry: {$green}{}{/$}", entry.key);
    bunt::println!("Value: {$yellow}{}{/$}", entry.value);
}