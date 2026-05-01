use super::{Chapter, ChapterError};

pub(crate) fn skills<'a>(chaps: Vec<Chapter>) -> Result<Skills, ChapterError<'a>> {
    let mut skills = Skills::default();
    chaps.into_iter().for_each(|chap| {
        let name = chap.name_is().to_string();
        let mut chap = chap.items.into_iter();
        while let [Some(k), Some(v)] = [chap.next(), chap.next()] {
            skills.push(&name, Skill::from([k, v]));
        }
    });

    Ok(skills)
}

#[derive(Debug, Default)]
pub(crate) struct Skills {
    languages: Vec<Skill>,
    stacks: Vec<Skill>,
    protocols: Vec<Skill>,
}

impl Skills {
    fn push(&mut self, name: &str, skill: Skill) {
        match name.trim_start_matches("skills.") {
            "languages" => self.language(skill),
            "stacks" => self.stack(skill),
            "protocols" => self.protocol(skill),
            val => panic!("I dont know how to handle table {}", val),
        }
    }

    fn language(&mut self, skill: Skill) {
        self.languages.push(skill);
    }

    fn stack(&mut self, skill: Skill) {
        self.stacks.push(skill);
    }

    fn protocol(&mut self, skill: Skill) {
        self.protocols.push(skill);
    }

    pub(crate) fn langs_are(&self) -> &[Skill] {
        self.languages.as_slice()
    }

    pub(crate) fn protos_are(&self) -> &[Skill] {
        self.protocols.as_slice()
    }

    pub(crate) fn stacks_are(&self) -> &[Skill] {
        self.stacks.as_slice()
    }
}

#[derive(Debug)]
pub(crate) struct Skill {
    name: String,
    proficiency: Proficiency,
}

impl Skill {
    pub(crate) fn name_is(&self) -> &str {
        &self.name
    }

    pub(crate) fn proficiency_is(&self) -> &str {
        (&self.proficiency).try_into().unwrap()
    }
}

impl From<[String; 2]> for Skill {
    fn from(value: [String; 2]) -> Self {
        let name = value[0].trim_matches('"').to_string();
        let proficiency = (value[1].trim_matches('"').to_lowercase())
            .as_str()
            .try_into()
            .unwrap();

        Self { name, proficiency }
    }
}

#[derive(Debug)]
pub(crate) enum Proficiency {
    Beginner,
    AdvancedBeginner,
    Intermediate,
    Advanced,
    Expert,
}

impl<'a> TryFrom<&'a str> for Proficiency {
    type Error = ChapterError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "beginner" => Ok(Self::Beginner),
            "advanced_beginner" => Ok(Self::AdvancedBeginner),
            "intermediate" => Ok(Self::Intermediate),
            "advanced" => Ok(Self::Advanced),
            "expert" => Ok(Self::Expert),
            val => Err(ChapterError::NotAProficiencyVariant(val)),
        }
    }
}

impl<'a> TryFrom<&'a Proficiency> for &'a str {
    type Error = ChapterError<'a>;

    fn try_from(value: &'a Proficiency) -> Result<Self, Self::Error> {
        Ok(match value {
            &Proficiency::Beginner => "beginner",
            &Proficiency::AdvancedBeginner => "advanced_beginner",
            &Proficiency::Intermediate => "intermediate",
            &Proficiency::Advanced => "advanced",
            &Proficiency::Expert => "expert",
        })
    }
}
