use crate::{build, generate, parse};

#[derive(Debug)]
pub(super) struct Command {
    // toml input file is always set to build.toml
    // unless changed by the user option '--toml' | 'input' | '--input-toml'
    // (all 3 are the same option)
    toml: String,
    // pdf output is enabled by default
    // default output format is 'auto_resume.pdf' file
    pdf: Option<String>,
    // html output is disabled by default
    html: Option<String>,
    // disables pdf output
    no_pdf: bool,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            toml: String::from("build.toml"),
            pdf: Some(String::from("auto_resume.pdf")),
            html: None,
            no_pdf: false,
        }
    }
}

impl Command {
    pub(super) fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
        let mut cmd = Self::default();
        while let Some(arg) = args.next() {
            match arg.trim() {
                "--toml-input" | "--toml" | "--input" => {
                    let next = args.next();
                    if next.is_none() {
                        return Err(
                            "--toml option requires an input file argument right after it.",
                        );
                    }
                    let next = next.unwrap();
                    cmd.toml(next);
                }
                "--no-pdf" => cmd.disable_pdf_output(),
                "--pdf" => {
                    if !cmd.no_pdf() {
                        let next = args.next();
                        if next.is_none() {
                            return Err(
                                "--pdf option requires an output file argument right after it.",
                            );
                        }
                        let next = next.unwrap();
                        cmd.pdf(next);
                    }
                }
                "--html" => {
                    let next = args.next();
                    if next.is_none() {
                        return Err(
                            "--html option requires an output file argument right after it.",
                        );
                    }
                    let next = next.unwrap();
                    cmd.html(next);
                }
                val => {
                    print!("{}", val);
                    return Err(" option is unrecognized.");
                }
            }
        }

        Ok(cmd)
    }

    fn toml(&mut self, value: String) {
        self.toml = value;
    }

    fn disable_pdf_output(&mut self) {
        self.no_pdf = true;
    }

    fn no_pdf(&self) -> bool {
        self.no_pdf
    }

    fn pdf(&mut self, value: String) {
        self.pdf = Some(value);
    }

    fn html(&mut self, value: String) {
        self.html = Some(value);
    }

    fn pdf_file(&self) -> Option<&str> {
        self.pdf.as_ref().map(|s| s.as_str())
    }

    fn html_file(&self) -> Option<&str> {
        self.html.as_ref().map(|s| s.as_str())
    }

    fn toml_file(&self) -> &str {
        &self.toml
    }

    pub(super) fn cmd(self) -> Result<(), &'static str> {
        let resume = parse::parse(self.toml_file());
        let html = if let Some(html_file) = self.html_file() {
            generate::write_html(resume, html_file).unwrap()
        } else {
            generate::generate_html(resume)
        };
        if self.no_pdf() {
            return Ok(());
        }
        build::pandoc_build_resume(html, self.pdf_file()).unwrap();

        Ok(())
    }
}
