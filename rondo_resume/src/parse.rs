pub(crate) mod education;
pub(crate) mod meta;
pub(crate) mod side_projects;
pub(crate) mod skills;

pub(crate) use education::Education;
pub(crate) use meta::Meta;
pub(crate) use side_projects::SideProjects;
pub(crate) use skills::Skills;

use std::fs;

fn chapters(file: &str) -> Vec<Chapter> {
    let toml = fs::read_to_string(file).unwrap();
    let mut lines = toml
        .lines()
        .filter(|l| !(l.trim().is_empty() || l.trim().starts_with('#')));

    let mut chapters: Vec<Chapter> = Vec::with_capacity(9);
    let mut chapter = Chapter::default();
    while let Some(line) = lines.next() {
        if line.starts_with('[') && line.ends_with(']') {
            if !chapter.is_empty() {
                chapter.items_fields();
                chapters.push(chapter.clone());
            }
            chapter = Chapter::default();
            chapter.name(line);
        } else {
            chapter.push(line.to_string());
        }
    }
    chapter.items_fields();
    chapters.push(chapter);

    chapters
}

// this should return a result
pub(crate) fn parse(file: &str) -> Resume {
    let chaps = chapters(file);
    let mut res = Resume::default();
    let mut skills = vec![];
    chaps.into_iter().for_each(|chap| {
        match chap.name_is() {
            "meta" => res.meta = meta::meta(chap).unwrap(),
            val if val.starts_with("skills.") => skills.push(chap),
            "education" => res.edu = education::education(&chap).unwrap(),
            "side-projects" => res.side_projects = side_projects::side_projects(chap).unwrap(),
            _ => (),
        };
    });
    res.skills = skills::skills(skills).unwrap();

    res
}

#[derive(Debug, Default)]
pub(crate) struct Resume {
    meta: Meta,
    side_projects: SideProjects,
    skills: Skills,
    edu: Education,
}

impl Resume {
    pub(crate) fn meta(&self) -> &Meta {
        &self.meta
    }

    pub(crate) fn side_projects(&self) -> &SideProjects {
        &self.side_projects
    }

    pub(crate) fn skills(&self) -> &Skills {
        &self.skills
    }

    pub(crate) fn education(&self) -> &Education {
        &self.edu
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Chapter {
    name: String,
    items: Vec<String>,
}

#[derive(Debug)]
pub(crate) enum ChapterError<'a> {
    LineIsNotATomlEntry(&'a str),
    ChapterMismatch(&'a str, &'a str),
    FieldIsMandatory(&'a str, &'a str),
    BothFirstAndLastNamesAreMandatory,
    NotAProficiencyVariant(&'a str),
    UnrecognizedField(&'a str, &'a str),
}

impl std::fmt::Display for ChapterError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LineIsNotATomlEntry(l) => format!("badly formatted toml item line: {}", l),
                Self::ChapterMismatch(c, e) =>
                    format!("expected {}, found {}; {} != {}", e, c, e, c),
                Self::FieldIsMandatory(c, f) =>
                    format!("the {} field is mandatory under the {} table", f, c),
                Self::BothFirstAndLastNamesAreMandatory =>
                    format!("you need to input both first and last names"),
                Self::NotAProficiencyVariant(p) =>
                    format!("{} is not a recognized proficiency variant", p),
                Self::UnrecognizedField(t, f) =>
                    format!("{} field doesnt belong to {} table", f, t),
            }
        )
    }
}

impl Chapter {
    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.items.is_empty()
    }

    fn name_is(&self) -> &str {
        &self.name
    }

    fn items_fields(&mut self) {
        let iter = &mut self.items;
        iter.reverse();
        let mut items: Vec<String> = Vec::new();

        'w0: while let Some(item) = iter.pop() {
            // WARN very bad/brittle condition
            // but entire parser wannabe will be replaced with a proper parser
            // in issue 2; so no problem
            // WARN block below is really wasteful; i push then pop the push again the same item
            if !item.starts_with(char::is_alphabetic) {
                let mut head: String = items.pop().unwrap();
                head.push_str(&item);
                while let Some(item) = iter.pop() {
                    if item.starts_with(char::is_alphabetic) {
                        items.push(head);
                        items.push(item);
                        continue 'w0;
                    } else {
                        head.push_str(&item);
                    }
                }
                items.push(head);
            } else {
                items.push(item);
            }
        }
        items = items
            .into_iter()
            .map(|i| {
                i.splitn(2, " = ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        self.items = items;
    }

    fn contains(&self, field: &str) -> bool {
        self.items.iter().find(|i| *i == field).is_some()
    }

    fn push(&mut self, line: String) {
        self.items.push(line);
    }

    fn read(&self, field: &str) -> Option<&str> {
        if let Some(pos) = self.items.iter().position(|i| i == field) {
            return Some(self.items[pos + 1].as_str());
        }

        None
    }

    fn name(&mut self, s: &str) {
        self.name = s[1..s.len() - 1].to_owned();
    }

    // #[deprecated(note = "not using hashmap anymore")]
    // fn itemize<'a>(&mut self, i: &'a str) -> Result<(), ChapterError<'a>> {
    //     if !i.contains('=') {
    //         return Err(ChapterError::LineIsNotATomlEntry(i));
    //     }
    //     // BUG
    //     let mut kv = i
    //         .splitn(2, '=')
    //         .map(|i| i.trim().trim_matches('"').to_string());
    //
    //     let key = match &kv.next().unwrap()[..] {
    //         "], git" => format!("{}{}", "], git", self.items.len()),
    //         val => val.to_string(),
    //     };
    //
    //     self.items.push([key, kv.next().unwrap_or("".to_string())]);
    //
    //     Ok(())
    // }
}

#[derive(Debug, Default)]
struct PartitionedChapter {
    name: String,
    parts: Vec<Chapter>,
}

pub(crate) struct Experience {
    start: u16,
    end: u16,
    workplace: String,
    kind: JobKind,
    position: String,
}

pub(crate) enum JobKind {
    Remote,
    Hybrid,
    InOffice,
}
