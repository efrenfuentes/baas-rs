use std::fmt::Display;
use std::slice::Iter;

/// The type of a field. This is used to determine the type of the field when
/// creating a table.
///
/// # Example
///
/// ```
/// use collections::field::FieldType;
///
/// let field_type = FieldType::Integer;
///
/// assert_eq!(field_type.to_string(), "BIGINT");
/// ```
#[derive(Debug, PartialEq)]
pub enum FieldType {
    Integer,
    Double,
    Serial,
    Char,
    Text,
    Timestamp,
    Date,
    Time,
    Boolean,
    Json,
    UUID,
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Integer => write!(f, "BIGINT"),
            FieldType::Double => write!(f, "DOUBLE PRECISION"),
            FieldType::Serial => write!(f, "BIGSERIAL"),
            FieldType::Char => write!(f, "VARCHAR(255)"),
            FieldType::Text => write!(f, "TEXT"),
            FieldType::Timestamp => write!(f, "TIMESTAMP WITHOUT TIME ZONE"),
            FieldType::Date => write!(f, "DATE"),
            FieldType::Time => write!(f, "TIME"),
            FieldType::Boolean => write!(f, "BOOLEAN"),
            FieldType::Json => write!(f, "JSON"),
            FieldType::UUID => write!(f, "UUID"),
        }
    }
}

pub enum SystemField {
    Id,
    InsertedAt,
    UpdatedAt,
}

impl Display for SystemField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemField::Id => write!(f, "id"),
            SystemField::InsertedAt => write!(f, "inserted_at"),
            SystemField::UpdatedAt => write!(f, "updated_at"),
        }
    }
}

impl SystemField {
    pub fn iterator() -> Iter<'static, SystemField> {
        static FIELDS: [SystemField; 3] = [SystemField::Id, SystemField::InsertedAt, SystemField::UpdatedAt];
        FIELDS.iter()
    }

    pub fn names() -> Vec<String> {
        let mut names = Vec::new();

        for field in SystemField::iterator() {
            names.push(field.to_string());
        }

        names
    }

    fn to_sql_options(&self) -> &str {
        match self {
            SystemField::Id => "UUID PRIMARY KEY DEFAULT gen_random_uuid()",
            SystemField::InsertedAt => "TIMESTAMP without time zone NOT NULL",
            SystemField::UpdatedAt => "TIMESTAMP without time zone NOT NULL",
        }
    }

    pub fn to_sql(&self) -> String {
        format!("{} {}", self.to_string(), self.to_sql_options())
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldOptions {
    pub unique: bool,
    pub not_null: bool,
    pub default: Option<String>,
}

impl FieldOptions {
    /// Create a new FieldOptions struct with the given options. All options
    /// are optional.
    ///
    /// # Example
    ///
    /// ```
    /// use collections::field::FieldOptions;
    ///
    /// let options = FieldOptions::new(true, true, Some("default".to_string()));
    ///
    /// assert_eq!(options.unique, true);
    /// assert_eq!(options.not_null, true);
    /// assert_eq!(options.default.as_deref(), Some("default"));
    /// ```
    pub fn new(unique: bool, not_null: bool, default: Option<String>) -> Self {
        Self {
            unique: unique,
            not_null: not_null,
            default: default,
        }
    }
}

impl Default for FieldOptions {
    fn default() -> Self {
        Self::new(false, false, None)
    }
}

pub struct Field {
    pub name: String,
    pub type_: FieldType,
    pub options: Option<FieldOptions>,
}

pub type Fields = Vec<Field>;

impl Field {
    /// Create a new field with the given name and type. The name and type are
    /// required, if options are not provided, the default options will be used.
    ///
    /// # Example
    ///
    /// ```
    /// use collections::field::{Field, FieldType, FieldOptions};
    ///
    ///
    /// let field = Field::new("id", FieldType::Integer, Some(FieldOptions::default()));
    ///
    ///
    /// assert_eq!(field.name, "id");
    /// assert_eq!(field.type_, FieldType::Integer);
    /// assert_eq!(field.options.unwrap(), FieldOptions::default());
    /// ```
    ///
    /// ```
    /// use collections::field::{Field, FieldType, FieldOptions};
    ///
    ///
    /// let field = Field::new("id", FieldType::Integer, None);
    ///
    ///
    /// assert_eq!(field.name, "id");
    /// assert_eq!(field.type_, FieldType::Integer);
    /// assert_eq!(field.options.unwrap(), FieldOptions::default());
    ///
    /// ```
    pub fn new(name: &str, type_: FieldType, options: Option<FieldOptions>) -> Self {
        let options = match options {
            Some(options) => Some(options),
            None => Some(FieldOptions::default()),
        };

        Self {
            name: name.to_string(),
            type_: type_,
            options: options,
        }
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!("{} {}", self.name, self.type_);

        if let Some(options) = &self.options {
            if options.not_null {
                sql.push_str(" NOT NULL");
            }

            if let Some(default) = &options.default {
                if self.type_ == FieldType::Integer || self.type_ == FieldType::Serial || self.type_ == FieldType::Double || self.type_ == FieldType::Boolean {
                    sql.push_str(&format!(" DEFAULT {}", default));
                } else {
                    sql.push_str(&format!(" DEFAULT '{}'", default));
                }
            }
        }

        sql
    }
}
