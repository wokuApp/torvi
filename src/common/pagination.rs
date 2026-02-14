use mongodb::bson::oid::ObjectId;
use rocket::form::FromForm;
use serde::Serialize;

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 100;

#[derive(Debug, FromForm)]
pub struct PaginationParams {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

impl PaginationParams {
    pub fn cursor_oid(&self) -> Result<Option<ObjectId>, String> {
        match &self.cursor {
            Some(c) => ObjectId::parse_str(c)
                .map(Some)
                .map_err(|_| "Invalid cursor".to_string()),
            None => Ok(None),
        }
    }

    pub fn effective_limit(&self) -> i64 {
        self.limit
            .map(|l| l.clamp(1, MAX_LIMIT))
            .unwrap_or(DEFAULT_LIMIT)
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, limit: i64) -> Self {
        let has_more = data.len() as i64 > limit;
        let mut data = data;
        if has_more {
            data.pop();
        }
        let next_cursor = if has_more {
            None // Will be set by caller based on last item's ID
        } else {
            None
        };
        PaginatedResponse {
            data,
            next_cursor,
            has_more,
        }
    }

    pub fn with_cursor(data: Vec<T>, limit: i64, cursor_fn: impl Fn(&T) -> String) -> Self {
        let has_more = data.len() as i64 > limit;
        let mut data = data;
        if has_more {
            data.pop();
        }
        let next_cursor = if has_more {
            data.last().map(|item| cursor_fn(item))
        } else {
            None
        };
        PaginatedResponse {
            data,
            next_cursor,
            has_more,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params_default_limit() {
        let params = PaginationParams {
            cursor: None,
            limit: None,
        };
        assert_eq!(params.effective_limit(), DEFAULT_LIMIT);
    }

    #[test]
    fn test_pagination_params_custom_limit() {
        let params = PaginationParams {
            cursor: None,
            limit: Some(50),
        };
        assert_eq!(params.effective_limit(), 50);
    }

    #[test]
    fn test_pagination_params_clamp_max() {
        let params = PaginationParams {
            cursor: None,
            limit: Some(500),
        };
        assert_eq!(params.effective_limit(), MAX_LIMIT);
    }

    #[test]
    fn test_pagination_params_clamp_min() {
        let params = PaginationParams {
            cursor: None,
            limit: Some(0),
        };
        assert_eq!(params.effective_limit(), 1);
    }

    #[test]
    fn test_pagination_params_valid_cursor() {
        let oid = ObjectId::new();
        let params = PaginationParams {
            cursor: Some(oid.to_string()),
            limit: None,
        };
        assert_eq!(params.cursor_oid().unwrap(), Some(oid));
    }

    #[test]
    fn test_pagination_params_invalid_cursor() {
        let params = PaginationParams {
            cursor: Some("not-an-oid".to_string()),
            limit: None,
        };
        assert!(params.cursor_oid().is_err());
    }

    #[test]
    fn test_pagination_params_no_cursor() {
        let params = PaginationParams {
            cursor: None,
            limit: None,
        };
        assert_eq!(params.cursor_oid().unwrap(), None);
    }

    #[test]
    fn test_paginated_response_no_more() {
        let data = vec![1, 2, 3];
        let response = PaginatedResponse::new(data, 5);
        assert_eq!(response.data.len(), 3);
        assert!(!response.has_more);
        assert!(response.next_cursor.is_none());
    }

    #[test]
    fn test_paginated_response_has_more() {
        let data = vec![1, 2, 3, 4, 5, 6]; // 6 items, limit 5 -> has_more
        let response = PaginatedResponse::with_cursor(data, 5, |item| item.to_string());
        assert_eq!(response.data.len(), 5);
        assert!(response.has_more);
        assert_eq!(response.next_cursor, Some("5".to_string()));
    }

    #[test]
    fn test_paginated_response_exact_limit() {
        let data = vec![1, 2, 3, 4, 5];
        let response = PaginatedResponse::new(data, 5);
        assert_eq!(response.data.len(), 5);
        assert!(!response.has_more);
    }

    #[test]
    fn test_paginated_response_serialization() {
        let data = vec!["a".to_string(), "b".to_string()];
        let response = PaginatedResponse::new(data, 10);
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"data\""));
        assert!(json.contains("\"has_more\""));
        assert!(json.contains("\"next_cursor\""));
    }
}
