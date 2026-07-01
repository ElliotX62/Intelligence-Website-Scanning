// database/query_builder.rs
// IWS v1.0 - Query Builder
// Membangun SQL queries secara dinamis dengan parameterized queries

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueryBuilder {
    table: String,
    columns: Vec<String>,
    conditions: Vec<String>,
    params: Vec<String>,
    param_values: Vec<String>,
    order_by: Option<String>,
    order_dir: Option<String>,
    limit_val: Option<u32>,
    offset_val: Option<u32>,
    joins: Vec<String>,
    group_by: Option<String>,
    param_counter: usize,
}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        QueryBuilder {
            table: table.to_string(),
            columns: vec!["*".to_string()],
            conditions: Vec::new(),
            params: Vec::new(),
            param_values: Vec::new(),
            order_by: None,
            order_dir: None,
            limit_val: None,
            offset_val: None,
            joins: Vec::new(),
            group_by: None,
            param_counter: 0,
        }
    }

    pub fn select(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|c| c.to_string()).collect();
        self
    }

    pub fn where_eq(mut self, field: &str, value: &str) -> Self {
        self.param_counter += 1;
        let param = format!("${}", self.param_counter);
        self.conditions.push(format!("{} = {}", field, param));
        self.param_values.push(value.to_string());
        self
    }

    pub fn where_like(mut self, field: &str, pattern: &str) -> Self {
        self.param_counter += 1;
        let param = format!("${}", self.param_counter);
        self.conditions.push(format!("{} LIKE {}", field, param));
        self.param_values.push(pattern.to_string());
        self
    }

    pub fn where_in(mut self, field: &str, values: &[String]) -> Self {
        if values.is_empty() { return self; }
        let placeholders: Vec<String> = values.iter().map(|v| {
            self.param_counter += 1;
            self.param_values.push(v.clone());
            format!("${}", self.param_counter)
        }).collect();
        self.conditions.push(format!("{} IN ({})", field, placeholders.join(", ")));
        self
    }

    pub fn where_null(mut self, field: &str) -> Self {
        self.conditions.push(format!("{} IS NULL", field));
        self
    }

    pub fn where_not_null(mut self, field: &str) -> Self {
        self.conditions.push(format!("{} IS NOT NULL", field));
        self
    }

    pub fn where_raw(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn order(mut self, field: &str, ascending: bool) -> Self {
        self.order_by = Some(field.to_string());
        self.order_dir = Some(if ascending { "ASC".to_string() } else { "DESC".to_string() });
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit_val = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset_val = Some(offset);
        self
    }

    pub fn join(mut self, join_type: &str, table: &str, on: &str) -> Self {
        self.joins.push(format!("{} JOIN {} ON {}", join_type, table, on));
        self
    }

    pub fn group(mut self, field: &str) -> Self {
        self.group_by = Some(field.to_string());
        self
    }

    pub fn build_select(&self) -> (String, Vec<String>) {
        let mut sql = format!("SELECT {} FROM {}", self.columns.join(", "), self.table);
        for join in &self.joins { sql.push_str(&format!(" {}", join)); }
        if !self.conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        if let Some(ref group) = self.group_by { sql.push_str(&format!(" GROUP BY {}", group)); }
        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {} {}", order, self.order_dir.as_deref().unwrap_or("ASC")));
        }
        if let Some(limit) = self.limit_val { sql.push_str(&format!(" LIMIT {}", limit)); }
        if let Some(offset) = self.offset_val { sql.push_str(&format!(" OFFSET {}", offset)); }
        (sql, self.param_values.clone())
    }

    pub fn build_count(&self) -> (String, Vec<String>) {
        let mut sql = format!("SELECT COUNT(*) FROM {}", self.table);
        for join in &self.joins { sql.push_str(&format!(" {}", join)); }
        if !self.conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        (sql, self.param_values.clone())
    }

    pub fn build_insert(&self, data: &HashMap<String, String>) -> (String, Vec<String>) {
        let mut columns = Vec::new();
        let mut placeholders = Vec::new();
        let mut values = Vec::new();
        let mut counter = self.param_counter;

        for (col, val) in data {
            counter += 1;
            columns.push(col.clone());
            placeholders.push(format!("${}", counter));
            values.push(val.clone());
        }

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table, columns.join(", "), placeholders.join(", ")
        );
        (sql, values)
    }

    pub fn build_update(&self, data: &HashMap<String, String>) -> (String, Vec<String>) {
        let mut sets = Vec::new();
        let mut values = self.param_values.clone();
        let mut counter = self.param_counter;

        for (col, val) in data {
            counter += 1;
            sets.push(format!("{} = ${}", col, counter));
            values.push(val.clone());
        }

        let mut sql = format!("UPDATE {} SET {}", self.table, sets.join(", "));
        if !self.conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        (sql, values)
    }

    pub fn build_delete(&self) -> (String, Vec<String>) {
        let mut sql = format!("DELETE FROM {}", self.table);
        if !self.conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        (sql, self.param_values.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_all() {
        let (sql, _) = QueryBuilder::new("users").build_select();
        assert_eq!(sql, "SELECT * FROM users");
    }

    #[test]
    fn test_select_columns() {
        let (sql, _) = QueryBuilder::new("users").select(&["id", "username"]).build_select();
        assert!(sql.contains("id, username"));
    }

    #[test]
    fn test_where_eq() {
        let (sql, params) = QueryBuilder::new("users").where_eq("username", "admin").build_select();
        assert!(sql.contains("WHERE username = $1"));
        assert_eq!(params[0], "admin");
    }

    #[test]
    fn test_multiple_conditions() {
        let (sql, params) = QueryBuilder::new("scans")
            .where_eq("status", "active")
            .where_eq("user_id", "user-123")
            .build_select();
        assert!(sql.contains("AND"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_order_limit() {
        let (sql, _) = QueryBuilder::new("scans")
            .order("created_at", false)
            .limit(10)
            .offset(20)
            .build_select();
        assert!(sql.contains("ORDER BY created_at DESC"));
        assert!(sql.contains("LIMIT 10"));
        assert!(sql.contains("OFFSET 20"));
    }

    #[test]
    fn test_build_count() {
        let (sql, _) = QueryBuilder::new("users").where_eq("role", "admin").build_count();
        assert!(sql.contains("COUNT(*)"));
    }

    #[test]
    fn test_build_insert() {
        let mut data = HashMap::new();
        data.insert("username".to_string(), "testuser".to_string());
        data.insert("email".to_string(), "test@test.com".to_string());
        let (sql, params) = QueryBuilder::new("users").build_insert(&data);
        assert!(sql.contains("INSERT INTO users"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_build_delete() {
        let (sql, _) = QueryBuilder::new("scans").where_eq("id", "scan-1").build_delete();
        assert!(sql.contains("DELETE FROM scans"));
    }
}
