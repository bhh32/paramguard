use paramguard_core::archive::interface::display::DefaultFormatter;

// Create a singleton formatter for the CLI
pub(crate) fn formatter() -> &'static DefaultFormatter {
    static FORMATTER: DefaultFormatter = DefaultFormatter;

    &FORMATTER
}
