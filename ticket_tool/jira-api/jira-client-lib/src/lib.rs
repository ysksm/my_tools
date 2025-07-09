pub mod api;
pub mod client;
pub mod error;
pub mod models;

pub use client::{Auth, JiraClient, JiraConfig};
pub use error::{Error, Result};
pub use models::{SearchRequest, SearchResults};

#[cfg(test)]
mod tests {
    use super::*;
    
    async fn create_test_client() -> (JiraClient, mockito::ServerGuard) {
        let server = mockito::Server::new_async().await;
        let config = JiraConfig::new(
            server.url(),
            Auth::Basic {
                username: "test@example.com".to_string(),
                api_token: "test-token".to_string(),
            },
        ).unwrap();
        
        (JiraClient::new(config).unwrap(), server)
    }
    
    #[tokio::test]
    async fn test_get_projects() {
        let (client, mut server) = create_test_client().await;
        
        let _m = server.mock("GET", "/rest/api/3/project")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[
                {
                    "id": "10000",
                    "key": "TEST",
                    "name": "Test Project",
                    "self": "https://example.atlassian.net/rest/api/3/project/10000"
                }
            ]"#)
            .create_async()
            .await;
        
        let projects = client.get_projects().await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].key, "TEST");
        assert_eq!(projects[0].name, "Test Project");
    }
    
    #[tokio::test]
    async fn test_search_issues_get() {
        let (client, mut server) = create_test_client().await;
        
        let _m = server.mock("GET", "/rest/api/3/search")
            .match_query(mockito::Matcher::UrlEncoded("jql".into(), "project=TEST".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "issues": [
                    {
                        "id": "10001",
                        "key": "TEST-1",
                        "self": "https://example.atlassian.net/rest/api/3/issue/10001",
                        "fields": {}
                    }
                ],
                "startAt": 0,
                "maxResults": 50,
                "total": 1
            }"#)
            .create_async()
            .await;
        
        let results = client.search_issues_get("project=TEST", None, None, None, None).await.unwrap();
        assert_eq!(results.total, 1);
        assert_eq!(results.issues.len(), 1);
        assert_eq!(results.issues[0].key, "TEST-1");
    }
    
    #[tokio::test]
    async fn test_authentication_header() {
        let (client, _server) = create_test_client().await;
        let headers = client.build_headers();
        
        assert!(headers.contains_key("authorization"));
        let auth_header = headers.get("authorization").unwrap().to_str().unwrap();
        assert!(auth_header.starts_with("Basic "));
    }
}