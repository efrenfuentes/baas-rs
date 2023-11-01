use crate::field::{Field, FieldOptions, FieldType, Fields, SystemField};

pub struct Schema {
    pub table_name: String,
    pub fields: Fields,
}

impl Schema {
    /// Create a new Schema, with an empty list of fields.
    ///
    /// # Example
    ///
    /// ```
    /// use collections::schema::Schema;
    ///
    /// let schema = Schema::new();
    ///
    /// assert_eq!(schema.fields.len(), 0);
    /// ```
    pub fn new() -> Schema {
        Schema {
            table_name: "".to_string(),
            fields: vec![],
        }
    }

    /// Add a field to the schema. The name and type are required.
    /// Field name cannot be a system field. If the field name is a system field,
    /// the field will not be added to the schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::schema::Schema;
    /// use collections::field::FieldType;
    ///
    /// let mut schema = Schema::new();
    ///
    /// schema.add_field("name", FieldType::Char, None);
    /// schema.add_field("age", FieldType::Integer, None);
    ///
    /// assert_eq!(schema.fields.len(), 2);
    /// ```
    ///
    /// ```
    /// use collections::schema::Schema;
    /// use collections::field::FieldType;
    ///
    /// let mut schema = Schema::new();
    ///
    /// schema.add_field("id", FieldType::Char, None);
    ///
    /// assert_eq!(schema.fields.len(), 0);
    /// ```
    pub fn add_field(&mut self, name: &str, type_: FieldType, options: Option<FieldOptions>) {
        // check name is not a system field
        for field in SystemField::names() {
            if field.to_lowercase().trim() == name.to_lowercase().trim() {
                return;
            }
        }

        self.fields.push(Field::new(name, type_, options));
    }

    /// Generate the SQL for creating the table.
    /// The SQL will be in the form:
    ///
    /// CREATE TABLE <table_name> (<field1>, <field2>, ...);
    ///
    /// # Example
    ///
    /// ```
    /// use collections::schema::SchemaBuilder;
    /// use collections::field::FieldType;
    ///
    /// let schema = SchemaBuilder::new()
    ///    .with_table_name("users")
    ///    .with_field("name", FieldType::Char, None)
    ///    .with_field("age", FieldType::Integer, None)
    ///    .build();
    ///
    /// assert_eq!(schema.to_sql(), "CREATE TABLE users (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), inserted_at TIMESTAMP without time zone NOT NULL, updated_at TIMESTAMP without time zone NOT NULL, name VARCHAR(255), age BIGINT);");
    /// ```
    ///
    /// ```
    /// use collections::schema::SchemaBuilder;
    /// use collections::field::FieldType;
    /// use collections::field::FieldOptions;
    ///
    /// let schema = SchemaBuilder::new()
    ///     .with_table_name("users")
    ///     .with_field("name", FieldType::Char, Some(FieldOptions::new(true, true, None)))
    ///     .with_field("age", FieldType::Integer, Some(FieldOptions::new(false, false, Some(5.to_string()))))
    ///     .with_field("email", FieldType::Char, Some(FieldOptions::new(true, true, None)))
    ///     .with_field("address", FieldType::Char, Some(FieldOptions::new(false, false, Some("123 Fake Street".to_string()))))
    ///     .build();
    ///
    /// let sql_expected = "CREATE TABLE users (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), inserted_at TIMESTAMP without time zone NOT NULL, updated_at TIMESTAMP without time zone NOT NULL, name VARCHAR(255) NOT NULL, age BIGINT DEFAULT 5, email VARCHAR(255) NOT NULL, address VARCHAR(255) DEFAULT '123 Fake Street', CONSTRAINT users_name_key UNIQUE (name), CONSTRAINT users_email_key UNIQUE (email));";
    ///
    /// assert_eq!(schema.to_sql(), sql_expected);
    /// ```
    pub fn to_sql(&self) -> String {
        let mut sql = format!("CREATE TABLE {} (", self.table_name);
        let mut constraints: Vec<String> = vec![];

        sql.push_str(&Self::system_fields_sql());

        for (index, field) in self.fields.iter().enumerate() {
            sql.push_str(&field.to_sql());

            if let Some(constraints_sql) = self.unique_constraints_sql(field, field.options.clone())
            {
                constraints.push(constraints_sql);
            }

            if index < self.fields.len() - 1 {
                sql.push_str(", ");
            }
        }

        if !constraints.is_empty() {
            sql.push_str(", ");
            sql.push_str(&constraints.join(", "));
        }

        sql.push_str(");");

        sql
    }

    fn system_fields_sql() -> String {
        let mut sql = String::new();

        for field in SystemField::iterator() {
            sql.push_str(&field.to_sql());
            sql.push_str(", ");
        }

        sql
    }

    fn unique_constraints_sql(
        &self,
        field: &Field,
        options: Option<FieldOptions>,
    ) -> Option<String> {
        if let Some(ref options) = options {
            if options.unique {
                return Some(format!(
                    "CONSTRAINT {}_{}_key UNIQUE ({})",
                    self.table_name, field.name, field.name
                ));
            }
        }

        None
    }
}

impl Default for Schema {
    /// Create a new Schema, with the default fields.
    /// The default fields are:
    /// * id: Integer
    /// * inserted_at: Timestamp
    /// * updated_at: Timestamp
    ///
    /// # Example
    ///
    /// ```
    /// use collections::schema::Schema;
    /// use collections::field::FieldType;
    ///
    /// let schema = Schema::default();
    ///
    /// assert_eq!(schema.fields.len(), 0);
    /// ```
    fn default() -> Self {
        SchemaBuilder::default().build()
    }
}

pub struct SchemaBuilder {
    pub schema: Schema,
}

impl SchemaBuilder {
    /// Create a new Schema, with an empty list of fields.
    ///
    /// # Example
    ///
    /// ```
    /// use collections::schema::SchemaBuilder;
    ///
    /// let schema = SchemaBuilder::new().build();
    ///
    /// assert_eq!(schema.fields.len(), 0);
    /// ```
    pub fn new() -> SchemaBuilder {
        SchemaBuilder {
            schema: Schema::new(),
        }
    }

    /// Set the table name for the schema.
    /// The default table name is the empty string.
    ///
    /// # Example
    ///
    /// ```
    /// use collections::schema::SchemaBuilder;
    ///
    /// let schema = SchemaBuilder::new()
    ///    .with_table_name("users")
    ///   .build();
    ///
    /// assert_eq!(schema.table_name, "users");
    /// ```
    pub fn with_table_name(mut self, table_name: &str) -> Self {
        self.schema.table_name = table_name.to_string();
        self
    }

    /// Add a field to the schema builder. The name and type are required.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::schema::SchemaBuilder;
    /// use collections::field::FieldType;
    ///
    /// let schema = SchemaBuilder::new()
    ///     .with_field("name", FieldType::Char, None)
    ///     .with_field("age", FieldType::Integer, None)
    ///     .build();
    ///
    /// assert_eq!(schema.fields.len(), 2);
    /// ```
    pub fn with_field(
        mut self,
        name: &str,
        type_: FieldType,
        options: Option<FieldOptions>,
    ) -> Self {
        self.schema.add_field(name, type_, options);
        self
    }

    /// Build the schema.
    pub fn build(self) -> Schema {
        self.schema
    }
}

impl Default for SchemaBuilder {
    /// Create a new SchemaBuilder.
    ///
    /// # Example
    ///
    /// ```
    /// use collections::schema::SchemaBuilder;
    /// use collections::field::FieldType;
    ///
    /// let schema = SchemaBuilder::default().build();
    ///
    /// assert_eq!(schema.table_name, "");
    /// assert_eq!(schema.fields.len(), 0);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
