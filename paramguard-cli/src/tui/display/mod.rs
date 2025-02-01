use paramguard_core::archive::interface::display::DefaultFormatter;

pub(crate) fn formatter() -> &'static DefaultFormatter {
    static FORMATTER: DefaultFormatter = DefaultFormatter;

    &FORMATTER
}
