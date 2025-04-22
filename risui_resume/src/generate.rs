use super::parse::{Education, Meta, Resume, SideProjects, Skills};

pub(crate) fn write_html(res: Resume, path: &str) -> Result<String, std::io::Error> {
    let mut html = String::from(BOILERPLATE);
    generate_meta(&res.meta(), &mut html);
    generate_side_projects(&res.side_projects(), &mut html);
    generate_skills(&res.skills(), &mut html);
    generate_education(&res.education(), &mut html);
    html.push_str("</body></html>");

    std::fs::write(path, html.clone());

    Ok(html)
}

pub(crate) fn generate_html(res: Resume) -> String {
    let mut html = String::from(BOILERPLATE);
    generate_meta(&res.meta(), &mut html);
    generate_side_projects(&res.side_projects(), &mut html);
    generate_skills(&res.skills(), &mut html);
    generate_education(&res.education(), &mut html);
    html.push_str("</body></html>");

    html
}

const BOILERPLATE: &str = "<!DOCTYPE html>
<html>
  <head>
    <link
      rel='stylesheet'
      type='text/css'
      href='build.css'
    />
  </head>
  <body>
    <div class='header'>";

const META_BP: &str = "
 <span class='header-child name'>Iadh Makhlouf</span>
 <span class='header-child contacts'>
   <a class='github' href='{github}'>github</a>
   <span class='email'>{email}</span>
   <span class='phone'>{phone}</span>
   <span class='country'>{country}</span>
 </span>
</div>
<div class='content'>
<div class='content-section projects'>
<span class='title'>Projects</span>
";

fn generate_meta(meta: &Meta, bp: &mut String) {
    bp.push_str(
        &META_BP
            .replace("{github}", meta.github_is())
            .replace("{email}", meta.email_is())
            .replace("{phone}", meta.phone_is())
            .replace("{country}", meta.country_is()),
    )
}

const SP_BP: &str = "
<div class='entry'>
  <a class='title' href = '{git}'>{name}</a>
  <span class='skills'>{technologies}</span>
  <span class='content'>{desc}</span>
</div>
";

const TECH_BP: &str = "<span class='tech'>{tech}</span>";

const DIV_CLOSE_BP: &str = "</div>";

fn generate_side_projects(side_projects: &SideProjects, bp: &mut String) {
    bp.push_str(
        &side_projects
            .iter()
            .map(|p| {
                let mut t = SP_BP
                    .replace("{git}", p.git_is())
                    .replace("{name}", p.name_is())
                    .replace("{desc}", p.desc_is());
                let techs = p
                    .technologies_are()
                    .into_iter()
                    .map(|t| TECH_BP.replace("{tech}", t))
                    .collect::<String>();
                t = t.replace("{technologies}", &techs);

                t
            })
            .collect::<String>(),
    );

    bp.push_str(DIV_CLOSE_BP);
}

const SKILLS_BP: &str = "
<div class='content-section skills'>
<span class='title'>Skills</span>
";

const SKILL_BP: &str = "
<div class='entry'>
  <span class='title'>{name}</span>
    <span class='sep'> | </span>
  <span class='content'>{proficiency}</span>
</div>
";

fn generate_skills(skills: &Skills, bp: &mut String) {
    bp.push_str(SKILLS_BP);
    // languages
    bp.push_str(
        "<div class='skills-langs skills-sub-section'>
            <span class='title'>Languages</span>",
    );
    bp.push_str(
        &skills
            .langs_are()
            .into_iter()
            .map(|s| {
                SKILL_BP
                    .replace("{name}", s.name_is())
                    .replace("{proficiency}", s.proficiency_is())
            })
            .collect::<String>(),
    );
    bp.push_str(DIV_CLOSE_BP);

    // stacks
    bp.push_str(
        "<div class='skills-stacks skills-sub-section'>
            <span class='title'>Stacks</span>",
    );
    bp.push_str(
        &skills
            .stacks_are()
            .into_iter()
            .map(|s| {
                SKILL_BP
                    .replace("{name}", s.name_is())
                    .replace("{proficiency}", s.proficiency_is())
            })
            .collect::<String>(),
    );
    bp.push_str(DIV_CLOSE_BP);

    // languages
    bp.push_str(
        "<div class='skills-protos skills-sub-section'>
            <span class='title'>Protocols</span>",
    );
    bp.push_str(
        &skills
            .protos_are()
            .into_iter()
            .map(|s| {
                SKILL_BP
                    .replace("{name}", s.name_is())
                    .replace("{proficiency}", s.proficiency_is())
            })
            .collect::<String>(),
    );
    bp.push_str(DIV_CLOSE_BP);
    bp.push_str(DIV_CLOSE_BP);
}

const EDU_BP: &str = "
<div class='content-section education'>
<span class='title'>Education</span>
<span class='uni'>{uni}</span>
<span class='degree'>{degree}</span>
</div>
</div>
";

fn generate_education(edu: &Education, bp: &mut String) {
    bp.push_str(
        &EDU_BP
            .replace("{uni}", edu.uni_is())
            .replace("{degree}", edu.degree_is()),
    );
}
