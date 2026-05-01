use super::{Chapter, ChapterError};

pub(crate) fn side_projects<'a>(chap: Chapter) -> Result<SideProjects, ChapterError<'a>> {
    let mut sps = SideProjects::default();
    let mut chap = chap.items.into_iter();
    while let [Some(k), Some(v)] = [chap.next(), chap.next()] {
        sps.push(SideProject::from([k, v]));
    }

    Ok(sps)
}

#[derive(Debug, Default)]
pub(crate) struct SideProject {
    git: String,
    desc: String,
    technologies: Vec<String>,
    // name as in the build.toml file
    item_name: String,
    // real name of the project as in the repo
    // name: String,
}

impl SideProject {
    pub(crate) fn name_is(&self) -> &str {
        &self.item_name
    }

    pub(crate) fn desc_is(&self) -> &str {
        &self.desc
    }

    pub(crate) fn technologies_are(&self) -> &[String] {
        self.technologies.as_slice()
    }

    pub(crate) fn git_is(&self) -> &str {
        &self.git
    }
}

#[derive(Debug, Default)]
pub(crate) struct SideProjects {
    items: Vec<SideProject>,
}

impl SideProjects {
    fn push(&mut self, sp: SideProject) {
        self.items.push(sp);
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<SideProject> {
        self.items.iter()
    }
}

impl From<[String; 2]> for SideProject {
    fn from(value: [String; 2]) -> Self {
        let fields = &value[1];
        let fields = &fields[2..fields.len() - 2];
        let technologies = {
            let start = fields.find("technologies = ").unwrap();
            let end = fields.find("\",], ").unwrap();
            if start >= end {
                panic!("need technologies field")
            }

            fields[start + "technologies = ".len()..end + 3].to_string()
        };
        let technologies = technologies[1..technologies.len() - 1]
            .replacen("\t", "", technologies.len())
            .split(",")
            .filter(|t| !t.is_empty())
            .map(|t| t.trim().trim_matches('"').to_string())
            .collect();
        let git = {
            let start = fields.find("git = ").unwrap();
            let mut end = fields.find("\"");
            while start >= end.unwrap() + 1 {
                end = fields[end.unwrap()..]
                    .find("\"")
                    .map(|p| p + end.unwrap() + 1);
                if end.is_none() {
                    panic!("need git field")
                }
            }
            end = fields[end.unwrap()..].find("\"").map(|p| p + end.unwrap());

            fields[start + "git = ".len()..end.unwrap() + 1]
                .trim_matches('"')
                .to_string()
        };
        let desc = {
            let start = fields.find("description = ").unwrap();
            let mut end = fields.find("\"");
            while start >= end.unwrap() {
                end = fields[end.unwrap()..]
                    .find("\"")
                    .map(|p| p + end.unwrap() + 1);
                if end.is_none() {
                    panic!("need description field")
                }
            }
            end = fields[end.unwrap()..].find("\"").map(|p| p + end.unwrap());

            fields[start + "description = ".len()..end.unwrap() + 1]
                .trim_matches('"')
                .to_string()
        };

        Self {
            technologies,
            git,
            desc,
            item_name: value[0].clone(),
        }
    }
}
