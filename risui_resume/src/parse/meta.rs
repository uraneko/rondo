use super::{Chapter, ChapterError};

#[derive(Debug, Default)]
pub(crate) struct Meta {
    name: String,
    github: String,
    discord: Option<String>,
    phone: Option<String>,
    email: String,
    country: Option<String>,
}

enum NameOrder {
    FirstLast,
    LastFirst,
}

pub(crate) fn meta<'a>(chap: Chapter) -> Result<Meta, ChapterError<'a>> {
    let mut meta = Meta::default();
    if !chap.contains("name") {
        return Err(ChapterError::FieldIsMandatory("meta", "name"));
    }
    meta.name(chap.read("name").unwrap(), NameOrder::FirstLast)
        .unwrap();

    if !chap.contains("github") {
        return Err(ChapterError::FieldIsMandatory("meta", "github"));
    }
    meta.github(chap.read("github").unwrap());

    if !chap.contains("email") {
        return Err(ChapterError::FieldIsMandatory("meta", "email"));
    }
    meta.email(chap.read("email").unwrap());

    meta.phone(chap.read("phone"));
    meta.discord(chap.read("discord"));
    meta.country(chap.read("country"));

    Ok(meta)
}

impl Meta {
    fn name(&mut self, s: &str, ord: NameOrder) -> Result<(), ChapterError> {
        let full_name = s
            .trim_start_matches("{ ")
            .trim_end_matches("\" }")
            .replacen('"', "", 3);
        let split_name = full_name.split(", ");
        let n = split_name.collect::<Vec<&str>>().try_into();
        if n.is_err() {
            return Err(ChapterError::BothFirstAndLastNamesAreMandatory);
        }
        let n: [&str; 2] = n.unwrap();

        let name: String = match ord {
            NameOrder::FirstLast => {
                if n[0].contains("family =") {
                    n[1].replace("given = ", "") + " " + &n[0].replace("family = ", "")
                } else {
                    n[0].replace("given = ", "") + " " + &n[1].replace("family = ", "")
                }
            }
            NameOrder::LastFirst => {
                if n[1].contains("given =") {
                    n[0].replace("family = ", "") + " " + &n[1].replace("given = ", "")
                } else {
                    n[0].replace("family = ", "") + " " + &n[1].replace("given = ", "")
                }
            }
        };
        self.name = name;

        Ok(())
    }

    fn github(&mut self, s: &str) {
        self.github = s.trim_matches('"').to_string();
    }

    fn email(&mut self, s: &str) {
        self.email = s.trim_matches('"').to_string();
    }

    fn phone(&mut self, s: Option<&str>) {
        self.phone = s.map(|s| s.trim_matches('"').to_string());
    }

    fn country(&mut self, s: Option<&str>) {
        self.country = s.map(|s| s.trim_matches('"').to_string());
    }

    fn discord(&mut self, s: Option<&str>) {
        self.discord = s.map(|s| s.trim_matches('"').to_string());
    }

    fn name_is(&self) -> &str {
        &self.name
    }

    pub(crate) fn phone_is(&self) -> &str {
        self.phone.as_ref().unwrap()
    }
    pub(crate) fn email_is(&self) -> &str {
        &self.email
    }
    pub(crate) fn country_is(&self) -> &str {
        self.country.as_ref().unwrap()
    }
    pub(crate) fn github_is(&self) -> &str {
        &self.github
    }
}
