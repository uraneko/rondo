use pandoc::{InputFormat, OutputFormat, Pandoc, PandocError, PandocOutput};
use pandoc::{InputKind, OutputKind, PandocOption};

pub(crate) fn pandoc_build_resume(
    html: String,
    output: Option<&str>,
) -> Result<PandocOutput, PandocError> {
    let output = if let Some(path) = output {
        OutputKind::File(path.into())
    } else {
        OutputKind::Pipe
    };
    let mut pandoc = Pandoc::new();
    pandoc
        .set_input(InputKind::Pipe(html))
        .add_options(&[
            PandocOption::Standalone,
            // PandocOption::PdfEngine("weasyprint".into()),
            PandocOption::PdfEngine("weasyprint".into()),
            PandocOption::Css("build.css".into()),
        ])
        .set_input_format(InputFormat::Html, vec![])
        .set_output_format(OutputFormat::Pdf, vec![])
        .set_output(output);

    pandoc.execute()
}
