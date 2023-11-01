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

#[derive(Debug, PartialEq)]
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
    /// Get an iterator over the system fields.
    /// The system fields are:
    /// * id
    /// * inserted_at
    /// * updated_at
    ///
    /// # Example
    ///
    /// ```
    /// use collections::field::SystemField;
    ///
    /// let fields = SystemField::iterator().collect::<Vec<&SystemField>>();
    ///
    /// assert_eq!(fields.len(), 3);
    /// assert_eq!(fields[0], &SystemField::Id);
    /// assert_eq!(fields[1], &SystemField::InsertedAt);
    /// assert_eq!(fields[2], &SystemField::UpdatedAt);
    /// ```
    pub fn iterator() -> Iter<'static, SystemField> {
        static FIELDS: [SystemField; 3] = [
            SystemField::Id,
            SystemField::InsertedAt,
            SystemField::UpdatedAt,
        ];
        FIELDS.iter()
    }

    /// Get the names of the system fields.
    /// The system fields are:
    /// * id
    /// * inserted_at
    /// * updated_at
    ///
    /// # Example
    ///
    /// ```
    /// use collections::field::SystemField;
    ///
    /// let names = SystemField::names();
    ///
    /// assert_eq!(names.len(), 3);
    /// assert_eq!(names[0], "id");
    /// assert_eq!(names[1], "inserted_at");
    /// assert_eq!(names[2], "updated_at");
    /// ```
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

    /// Get the SQL for the system field.
    /// The system fields are:
    ///
    /// * id
    /// * inserted_at
    /// * updated_at
    ///
    /// # Example
    ///
    /// ```
    /// use collections::field::SystemField;
    ///
    /// let sql = SystemField::Id.to_sql();
    /// assert_eq!(sql, "id UUID PRIMARY KEY DEFAULT gen_random_uuid()");
    ///
    /// let sql = SystemField::InsertedAt.to_sql();
    /// assert_eq!(sql, "inserted_at TIMESTAMP without time zone NOT NULL");
    ///
    /// let sql = SystemField::UpdatedAt.to_sql();
    /// assert_eq!(sql, "updated_at TIMESTAMP without time zone NOT NULL");
    /// ```
    pub fn to_sql(&self) -> String {
        format!("{} {}", self, self.to_sql_options())
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    /// assert!(options.unique);
    /// assert!(options.not_null);
    /// assert_eq!(options.default, Some("default".to_string()));
    /// ```
    pub fn new(unique: bool, not_null: bool, default: Option<String>) -> Self {
        Self {
            unique,
            not_null,
            default,
        }
    }
}

impl Default for FieldOptions {
    /// Create a new FieldOptions struct with the default options. The default
    /// options are:
    /// * unique: false
    /// * not_null: false
    /// * default: None
    ///
    /// # Example
    ///
    /// ```
    /// use collections::field::FieldOptions;
    ///
    /// let options = FieldOptions::default();
    ///
    /// assert!(!options.unique);
    /// assert!(!options.not_null);
    /// assert_eq!(options.default, None);
    /// ```
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
        let options = Self::field_options_or_default(options);

        Self {
            name: name.to_string(),
            type_,
            options,
        }
    }

    fn field_options_or_default(options: Option<FieldOptions>) -> Option<FieldOptions> {
        match options {
            Some(options) => Some(options),
            None => Some(FieldOptions::default()),
        }
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!("{} {}", self.name, self.type_);

        if self.has_options() {
            sql.push_str(self.not_null_sql());
            sql.push_str(&self.default_sql());
        }

        sql
    }

    fn is_numeric_field(&self) -> bool {
        self.type_ == FieldType::Integer
            || self.type_ == FieldType::Serial
            || self.type_ == FieldType::Double
    }

    fn is_boolean_field(&self) -> bool {
        self.type_ == FieldType::Boolean
    }

    fn has_options(&self) -> bool {
        self.options.is_some()
    }

    fn not_null_option(&self) -> bool {
        if let Some(options) = &self.options {
            return options.not_null;
        }

        false
    }

    fn has_default(&self) -> bool {
        if let Some(options) = &self.options {
            return options.default.is_some();
        }

        false
    }

    fn default_value(&self) -> String {
        if self.has_default() {
            if let Some(options) = &self.options {
                if let Some(default) = &options.default {
                    return default.to_string();
                }
            }
        }

        String::new()
    }

    fn not_null_sql(&self) -> &str {
        if self.not_null_option() {
            return " NOT NULL";
        }

        ""
    }

    fn default_sql(&self) -> String {
        if self.has_default() {
            if self.is_numeric_field() || self.is_boolean_field() {
                return format!(" DEFAULT {}", self.default_value());
            } else {
                return format!(" DEFAULT '{}'", self.default_value());
            }
        }

        String::new()
    }
}
