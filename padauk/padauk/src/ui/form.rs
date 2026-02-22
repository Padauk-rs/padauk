use crate::ui::render_callback::request_redraw;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use uuid::Uuid;

pub(crate) type FieldValidator = Arc<dyn Fn(&str) -> Option<String> + Send + Sync>;

#[derive(Clone)]
pub struct FormKey {
    id: String,
}

#[derive(Default)]
struct FormState {
    field_values: HashMap<String, String>,
    validators: HashMap<String, FieldValidator>,
    field_errors: HashMap<String, String>,
}

type FormsMap = Mutex<HashMap<String, FormState>>;

static FORMS: OnceLock<FormsMap> = OnceLock::new();

fn forms() -> &'static FormsMap {
    FORMS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn form_key() -> FormKey {
    FormKey {
        id: Uuid::new_v4().to_string(),
    }
}

impl FormKey {
    pub fn validate(&self) -> bool {
        validate_form(&self.id)
    }

    pub fn clear_validation(&self) {
        let mut all_forms = forms().lock().unwrap();
        if let Some(state) = all_forms.get_mut(&self.id) {
            state.field_errors.clear();
        }
        request_redraw();
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }
}

pub(crate) fn sync_field(
    form_id: &str,
    field_id: &str,
    value: &str,
    validator: Option<FieldValidator>,
) {
    let mut all_forms = forms().lock().unwrap();
    let state = all_forms.entry(form_id.to_string()).or_default();

    let previous = state
        .field_values
        .insert(field_id.to_string(), value.to_string());
    if previous.as_deref() != Some(value) {
        state.field_errors.remove(field_id);
    }

    if let Some(v) = validator {
        state.validators.insert(field_id.to_string(), v);
    }
}

pub(crate) fn field_error(form_id: &str, field_id: &str) -> Option<String> {
    forms()
        .lock()
        .unwrap()
        .get(form_id)
        .and_then(|state| state.field_errors.get(field_id).cloned())
}

pub(crate) fn validate_field(form_id: &str, field_id: &str) -> bool {
    let validator_and_value = {
        let all_forms = forms().lock().unwrap();
        let Some(state) = all_forms.get(form_id) else {
            return true;
        };

        let validator = state.validators.get(field_id).cloned();
        let value = state
            .field_values
            .get(field_id)
            .cloned()
            .unwrap_or_default();
        (validator, value)
    };

    let Some(validator) = validator_and_value.0 else {
        let mut all_forms = forms().lock().unwrap();
        if let Some(state) = all_forms.get_mut(form_id) {
            state.field_errors.remove(field_id);
        }
        request_redraw();
        return true;
    };

    let error = validator(&validator_and_value.1);

    let mut all_forms = forms().lock().unwrap();
    let state = all_forms.entry(form_id.to_string()).or_default();
    match error {
        Some(err) => {
            state.field_errors.insert(field_id.to_string(), err);
            request_redraw();
            false
        }
        None => {
            state.field_errors.remove(field_id);
            request_redraw();
            true
        }
    }
}

fn validate_form(form_id: &str) -> bool {
    let validators_and_values = {
        let all_forms = forms().lock().unwrap();
        let Some(state) = all_forms.get(form_id) else {
            return true;
        };

        state
            .validators
            .iter()
            .map(|(field_id, validator)| {
                let value = state
                    .field_values
                    .get(field_id)
                    .cloned()
                    .unwrap_or_default();
                (field_id.clone(), validator.clone(), value)
            })
            .collect::<Vec<_>>()
    };

    let mut next_errors: HashMap<String, String> = HashMap::new();
    for (field_id, validator, value) in validators_and_values {
        if let Some(error) = validator(&value) {
            next_errors.insert(field_id, error);
        }
    }

    let is_valid = next_errors.is_empty();

    let mut all_forms = forms().lock().unwrap();
    let state = all_forms.entry(form_id.to_string()).or_default();
    state.field_errors = next_errors;

    request_redraw();
    is_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_validate_tracks_errors() {
        let key = form_key();
        let form_id = key.id().to_string();
        let field_id = "email";

        sync_field(
            &form_id,
            field_id,
            "",
            Some(Arc::new(|v| {
                if v.trim().is_empty() {
                    Some("Required".to_string())
                } else {
                    None
                }
            })),
        );

        assert!(!key.validate());
        assert_eq!(field_error(&form_id, field_id).as_deref(), Some("Required"));

        sync_field(&form_id, field_id, "x@example.com", None);
        assert_eq!(field_error(&form_id, field_id), None);
    }
}
