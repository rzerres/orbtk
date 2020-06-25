use std::collections::HashMap;

use ron::Value;

use crate::{
    config::{ThemeConfig, RESOURCE_KEY},
    Selector, Style,
};

#[derive(Debug, Clone, Default)]
pub struct Theme {
    styles: HashMap<String, Style>,
}

impl Theme {
    pub fn from_config(theme: ThemeConfig) -> Self {
        let mut styles = HashMap::new();

        for style_key in theme.styles.keys() {
            let mut properties = HashMap::new();
            Theme::read_properties(style_key, &theme, &mut properties);

            let mut states = HashMap::new();

            for state_key in theme.styles.get(style_key).unwrap().states.keys() {
                let mut state = HashMap::new();
                Theme::read_states(style_key, state_key, &theme, &mut state);
                states.insert(state_key.clone(), state);
            }

            styles.insert(style_key.clone(), Style { properties, states });
        }

        Theme { styles }
    }

    pub fn style(&self, key: &str) -> Option<&Style> {
        self.styles.get(key)
    }

    pub fn properties<'a>(&'a self, selector: &Selector) -> Option<&'a HashMap<String, Value>> {
        if !selector.dirty {
            return None;
        }

        if let Some(style) = &selector.style {
            if let Some(state) = &selector.state {
                return self.styles.get(style)?.states.get(state);
            }

            return Some(&self.styles.get(style)?.properties);
        }

        return None;
    }

    fn read_properties(key: &String, theme: &ThemeConfig, properties: &mut HashMap<String, Value>) {
        if key.is_empty() {
            return;
        }

        if let Some(style) = theme.styles.get(key) {
            Theme::read_properties(&style.base, theme, properties);

            for (key, value) in &style.properties {
                Theme::read_property(key, value, theme, properties);
            }
        }
    }

    fn read_states(
        style_key: &String,
        state_key: &String,
        theme: &ThemeConfig,
        states: &mut HashMap<String, Value>,
    ) {
        if style_key.is_empty() || state_key.is_empty() {
            return;
        }

        if let Some(style) = theme.styles.get(style_key) {
            Theme::read_states(&style.base, state_key, theme, states);

            for (key, value) in &style.properties {
                Theme::read_property(key, value, theme, states);
            }

            if let Some(state) = style.states.get(state_key) {
                for (key, value) in state {
                    Theme::read_property(key, value, theme, states);
                }
            }
        }
    }

    fn read_property(
        key: &String,
        value: &Value,
        theme: &ThemeConfig,
        map: &mut HashMap<String, Value>,
    ) {
        if let Ok(value) = value.clone().into_rust::<String>() {
            if value.starts_with(RESOURCE_KEY) {
                if let Some(value) = theme.resources.get(&value.replace(RESOURCE_KEY, "")) {
                    map.insert(key.clone(), value.clone());
                }
            } else {
                map.insert(key.clone(), Value::String(value));
            }
        } else {
            map.insert(key.clone(), value.clone());
        }
    }
}
