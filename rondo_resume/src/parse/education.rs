use super::{Chapter, ChapterError};

pub(crate) fn education<'a>(chap: &'a Chapter) -> Result<Education, ChapterError<'a>> {
    Education::try_from(chap.items.as_slice())
}

#[derive(Debug, Default)]
pub(crate) struct Education {
    university: String,
    year: Option<u16>,
    major: Option<String>,
    degree: String,
}

impl Education {
    pub(crate) fn uni_is(&self) -> &str {
        &self.university
    }

    pub(crate) fn degree_is(&self) -> &str {
        &self.degree
    }

    fn year(&mut self, year: &str) {
        self.year = Some(year.parse().unwrap());
    }

    fn degree(&mut self, degree: &str) {
        self.degree = degree.trim_matches('"').to_string();
    }

    fn university(&mut self, university: &str) {
        self.university = university.trim_matches('"').to_string();
    }

    fn major(&mut self, major: &str) {
        self.major = Some(major.trim_matches('"').to_string());
    }
}

impl<'a> TryFrom<&'a [String]> for Education {
    type Error = ChapterError<'a>;

    fn try_from(value: &'a [String]) -> Result<Self, Self::Error> {
        let mut edu = Education::default();
        let mut err: Option<Result<Education, ChapterError>> = None;
        value.chunks(2).for_each(|c| match c[0].as_str() {
            "year" => edu.year(&c[1]),
            "university" => edu.university(&c[1]),
            "major" => edu.major(&c[1]),
            "degree" => edu.degree(&c[1]),
            val => {
                err = Some(Err(ChapterError::UnrecognizedField(
                    "education",
                    val.clone(),
                )))
            }
        });
        if err.is_some() {
            return err.unwrap();
        }

        Ok(edu)
    }
}
