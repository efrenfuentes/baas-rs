use collections::field::{FieldOptions, FieldType};
use collections::schema::{Schema, SchemaBuilder};

#[test]
fn test_schema_new() {
    let schema = Schema::new();

    assert_eq!(schema.table_name, "");
    assert_eq!(schema.fields.len(), 0);
}

#[test]
fn test_schema_default() {
    let schema = Schema::default();

    assert_eq!(schema.table_name, "");
    assert_eq!(schema.fields.len(), 0);
}

#[test]
fn test_add_field() {
    let mut schema = Schema::new();

    schema.add_field("name", FieldType::Char, None);
    schema.add_field("age", FieldType::Integer, None);

    assert_eq!(schema.fields.len(), 2);
    assert_eq!(schema.fields[0].name, "name");
    assert_eq!(schema.fields[0].type_, FieldType::Char);
    assert_eq!(schema.fields[1].name, "age");
    assert_eq!(schema.fields[1].type_, FieldType::Integer);
}

#[test]
fn test_add_field_ignore_system_fields() {
    let mut schema = Schema::new();

    schema.add_field("id", FieldType::UUID, None);
    schema.add_field("inserted_at", FieldType::Timestamp, None);
    schema.add_field("updated_at", FieldType::Timestamp, None);

    assert_eq!(schema.fields.len(), 0);
}

#[test]
fn test_to_sql() {
    let schema = SchemaBuilder::new()
        .with_table_name("users")
        .with_field("name", FieldType::Char, None)
        .with_field("age", FieldType::Integer, None)
        .build();

    assert_eq!(schema.to_sql(), "CREATE TABLE users (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), inserted_at TIMESTAMP without time zone NOT NULL, updated_at TIMESTAMP without time zone NOT NULL, name VARCHAR(255), age BIGINT);");
}

#[test]
fn test_to_sql_with_options() {
    let schema = SchemaBuilder::new()
        .with_table_name("users")
        .with_field(
            "name",
            FieldType::Char,
            Some(FieldOptions::new(true, true, None)),
        )
        .with_field(
            "age",
            FieldType::Integer,
            Some(FieldOptions::new(false, false, Some(5.to_string()))),
        )
        .with_field(
            "email",
            FieldType::Char,
            Some(FieldOptions::new(true, true, None)),
        )
        .with_field(
            "address",
            FieldType::Char,
            Some(FieldOptions::new(
                false,
                false,
                Some("123 Fake Street".to_string()),
            )),
        )
        .build();

    let sql_expected = "CREATE TABLE users (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), inserted_at TIMESTAMP without time zone NOT NULL, updated_at TIMESTAMP without time zone NOT NULL, name VARCHAR(255) NOT NULL, age BIGINT DEFAULT 5, email VARCHAR(255) NOT NULL, address VARCHAR(255) DEFAULT '123 Fake Street', CONSTRAINT users_name_key UNIQUE (name), CONSTRAINT users_email_key UNIQUE (email));";

    assert_eq!(schema.to_sql(), sql_expected);
}

#[test]
fn test_schema_builder_new() {
    let schema = SchemaBuilder::new().build();

    assert_eq!(schema.table_name, "");
    assert_eq!(schema.fields.len(), 0);
}

#[test]
fn test_schema_builder_default() {
    let schema = SchemaBuilder::default().build();

    assert_eq!(schema.table_name, "");
    assert_eq!(schema.fields.len(), 0);
}
