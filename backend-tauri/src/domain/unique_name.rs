pub struct UniqueName(String);

impl UniqueName {
    pub async fn new(name: Option<String>, collection: &[String]) -> Self {
        let uniq_name = match name {
            Some(name) => UniqueName::generate_unique_name_from_string(name, collection).await,
            None => UniqueName::generate_unique_name(collection).await,
        };
        Self(uniq_name)
    }

    async fn generate_unique_name_from_string(mut name: String, collection: &[String]) -> String {
        if UniqueName::check_unique(&name, collection).await {
            return name;
        }
        name.push_str(&(collection.len() + 1).to_string());
        name
    }

    async fn generate_unique_name(collection: &[String]) -> String {
        if let Some(last_name) = collection.last() {
            let mut split_last_name: Vec<&str> = last_name.split("(").collect();
            split_last_name.pop();
            return format!("{} ({})", split_last_name.concat(), collection.len() + 1);
        }
        "New Name (1)".to_string()
    }

    async fn check_unique(name: &String, collection: &[String]) -> bool {
        collection
            .iter()
            .filter(|string| name == *string)
            .collect::<Vec<&String>>()
            .is_empty()
    }
}

impl From<UniqueName> for String {
    fn from(val: UniqueName) -> Self {
        val.0
    }
}
