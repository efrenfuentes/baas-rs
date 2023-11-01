use collections::field::{Field, FieldOptions, FieldType, SystemField};

#[test]
fn test_field_type_integer() {
    let field_type = FieldType::Integer;

    assert_eq!(field_type.to_string(), "BIGINT");
}

#[test]
fn test_field_type_double() {
    let field_type = FieldType::Double;

    assert_eq!(field_type.to_string(), "DOUBLE PRECISION");
}

#[test]
fn test_field_type_boolean() {
    let field_type = FieldType::Boolean;

    assert_eq!(field_type.to_string(), "BOOLEAN");
}

#[test]
fn test_field_type_serial() {
    let field_type = FieldType::Serial;

    assert_eq!(field_type.to_string(), "BIGSERIAL");
}

#[test]
fn test_field_type_char() {
    let field_type = FieldType::Char;

    assert_eq!(field_type.to_string(), "VARCHAR(255)");
}

#[test]
fn test_field_type_text() {
    let field_type = FieldType::Text;

    assert_eq!(field_type.to_string(), "TEXT");
}

#[test]
fn test_field_type_timestamp() {
    let field_type = FieldType::Timestamp;

    assert_eq!(field_type.to_string(), "TIMESTAMP WITHOUT TIME ZONE");
}

#[test]
fn test_field_type_date() {
    let field_type = FieldType::Date;

    assert_eq!(field_type.to_string(), "DATE");
}

#[test]
fn test_field_type_time() {
    let field_type = FieldType::Time;

    assert_eq!(field_type.to_string(), "TIME");
}

#[test]
fn test_field_type_json() {
    let field_type = FieldType::Json;

    assert_eq!(field_type.to_string(), "JSON");
}

#[test]
fn test_field_type_uuid() {
    let field_type = FieldType::UUID;

    assert_eq!(field_type.to_string(), "UUID");
}

#[test]
fn test_system_field_id() {
    let system_field = SystemField::Id;

    assert_eq!(system_field.to_string(), "id");
}

#[test]
fn test_system_field_inserted_at() {
    let system_field = SystemField::InsertedAt;

    assert_eq!(system_field.to_string(), "inserted_at");
}

#[test]
fn test_system_field_updated_at() {
    let system_field = SystemField::UpdatedAt;

    assert_eq!(system_field.to_string(), "updated_at");
}

#[test]
fn test_system_field_iterator() {
    let system_fields = SystemField::iterator();

    assert_eq!(system_fields.len(), 3);

    let mut count = 0;
    for field in system_fields {
        count += 1;
        match count {
            1 => assert_eq!(field, &SystemField::Id),
            2 => assert_eq!(field, &SystemField::InsertedAt),
            3 => assert_eq!(field, &SystemField::UpdatedAt),
            _ => panic!("Unexpected field"),
        }
    }

    assert_eq!(count, 3);
}

#[test]
fn test_system_field_names() {
    let system_fields = SystemField::names();

    assert_eq!(system_fields.len(), 3);

    let mut count = 0;
    for field in system_fields {
        count += 1;
        match count {
            1 => assert_eq!(field, "id"),
            2 => assert_eq!(field, "inserted_at"),
            3 => assert_eq!(field, "updated_at"),
            _ => panic!("Unexpected field"),
        }
    }

    assert_eq!(count, 3);
}

#[test]
fn test_system_field_to_sql() {
    let system_fields = SystemField::iterator();

    assert_eq!(system_fields.len(), 3);

    let mut count = 0;
    for field in system_fields {
        count += 1;
        match count {
            1 => assert_eq!(
                field.to_sql(),
                "id UUID PRIMARY KEY DEFAULT gen_random_uuid()"
            ),
            2 => assert_eq!(
                field.to_sql(),
                "inserted_at TIMESTAMP without time zone NOT NULL"
            ),
            3 => assert_eq!(
                field.to_sql(),
                "updated_at TIMESTAMP without time zone NOT NULL"
            ),
            _ => panic!("Unexpected field"),
        }
    }

    assert_eq!(count, 3);
}

#[test]
fn test_field_options() {
    let options = FieldOptions::new(true, true, Some("default".to_string()));

    assert!(options.unique);
    assert!(options.not_null);
    assert_eq!(options.default, Some("default".to_string()));
}

#[test]
fn test_field_options_default() {
    let options = FieldOptions::default();

    assert!(!options.unique);
    assert!(!options.not_null);
    assert_eq!(options.default, None);
}

#[test]
fn test_field() {
    let field = Field::new("id", FieldType::Integer, Some(FieldOptions::default()));

    assert_eq!(field.name, "id");
    assert_eq!(field.type_, FieldType::Integer);
    assert_eq!(field.options.unwrap(), FieldOptions::default());

    let field = Field::new("id", FieldType::Integer, None);

    assert_eq!(field.name, "id");
    assert_eq!(field.type_, FieldType::Integer);
    assert_eq!(field.options.unwrap(), FieldOptions::default());
}
