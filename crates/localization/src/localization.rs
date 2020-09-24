///
pub trait Localization {
    /// Gets the current language identified by a language key (e.g. `en_US`, `de_DE`).
    fn language(&self) -> &String;

    /// Sets the current language identified by its language key (e.g. `en_US`, `de_DE`).
    fn set_language(&mut self, key: &str);

    /// Gets the translation string matched by the active language key. The function will retrun properties default string, if a translation string is missing.
    fn text(&self, key: String) -> String;
}
