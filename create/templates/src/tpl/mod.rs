pub mod just;

pub fn generate(writer: &crate::TemplateWriter, config: &crate::Config) {
    just::generate(writer, config);
}