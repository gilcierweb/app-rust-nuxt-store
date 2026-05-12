use validator::{ValidationError, ValidationErrors};

fn resolve_field_label(field: &str) -> String {
    let key = format!("validation.fields.{}", field);
    let locale = rust_i18n::locale();

    if let Some(translated) = crate::_rust_i18n_try_translate(&locale, key.as_str()) {
        return translated.into_owned();
    }

    field.to_owned()
}

fn resolve_field_error_message(field: &str, error: &ValidationError) -> String {
    let field_label = resolve_field_label(field);

    if let Some(message) = &error.message {
        let key_or_message = message.as_ref();
        let translated = t!(key_or_message, field = field_label.as_str()).into_owned();
        if translated != key_or_message {
            return translated;
        }
        return key_or_message
            .replace("%{field}", field_label.as_str())
            .replace("{field}", field_label.as_str());
    }

    if error.code.as_ref() == "required" {
        return t!("validation.field_required", field = field_label.as_str()).into_owned();
    }

    t!("validation.field_invalid", field = field_label.as_str()).into_owned()
}

pub fn first_validation_error_message(errors: &ValidationErrors) -> String {
    for (field, list) in errors.field_errors() {
        if let Some(error) = list.first() {
            return resolve_field_error_message(field.as_ref(), error);
        }
    }

    t!("errors.validation").into_owned()
}

#[cfg(test)]
mod tests {
    use super::resolve_field_label;

    #[test]
    fn resolves_translated_label_for_legal_name_in_pt_br() {
        rust_i18n::set_locale("pt-BR");
        assert_eq!(resolve_field_label("legal_name"), "Razão social");
    }
}
