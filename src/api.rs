// API client - STUB IMPLEMENTATION
// 
// This file originally used internal API clients that have been removed.
// You'll need to implement your own API integration here.
//
// Recommended approach:
// 1. Define your data types (User, Workflow, Profile, etc.)
// 2. Use gloo-net or reqwest-wasm for HTTP calls
// 3. Implement authentication with your backend

use gloo_console::log;
use serde::{Deserialize, Serialize};

// ===== TYPE DEFINITIONS =====
// Replace these with your actual data types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserType {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub id: ProfileId,
    pub name: String,
    pub codename: String,
    pub description: Option<String>,
    pub publish_state: PublishState,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProfileId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileSourceData {
    pub source: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub publish_state: Option<PublishState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PublishState {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: WorkflowId,
    pub name: String,
    pub description: Option<String>,
    pub content: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    pub id: WorkflowId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ===== STUB API FUNCTIONS =====

/// Login - STUB IMPLEMENTATION
/// Replace with actual authentication logic
pub async fn login(_base_url: &str, email: &str, password: &str) -> Result<UserType, String> {
    log!("STUB: Login attempt for:", email);
    
    // Demo: Accept any non-empty credentials
    if !email.is_empty() && !password.is_empty() {
        Ok(UserType {
            id: "demo-user-1".to_string(),
            email: email.to_string(),
            name: Some("Demo User".to_string()),
            role: "admin".to_string(),
        })
    } else {
        Err("Email and password required".to_string())
    }
}

/// Get profiles - STUB IMPLEMENTATION
pub async fn get_profiles(_base_url: &str) -> Result<Vec<Profile>, String> {
    log!("STUB: Fetching profiles");
    
    Ok(vec![
        Profile {
            id: ProfileId("demo-profile-1".to_string()),
            name: "Demo Profile 1".to_string(),
            codename: "demo1".to_string(),
            description: Some("A demo profile".to_string()),
            publish_state: PublishState::Published,
            created_at: "2026-01-01T00:00:00Z".to_string(),
        },
        Profile {
            id: ProfileId("demo-profile-2".to_string()),
            name: "Demo Profile 2".to_string(),
            codename: "demo2".to_string(),
            description: Some("Another demo profile".to_string()),
            publish_state: PublishState::Draft,
            created_at: "2026-01-15T00:00:00Z".to_string(),
        },
    ])
}

/// Get profile by ID - STUB IMPLEMENTATION
pub async fn get_profile(_base_url: &str, id: &ProfileId) -> Result<Profile, String> {
    log!("STUB: Fetching profile:", &id.0);
    
    Ok(Profile {
        id: id.clone(),
        name: format!("Profile {}", id.0),
        codename: format!("code-{}", id.0),
        description: Some("Demo profile description".to_string()),
        publish_state: PublishState::Published,
        created_at: "2026-01-01T00:00:00Z".to_string(),
    })
}

/// Get profile source - STUB IMPLEMENTATION
pub async fn get_profile_source(_base_url: &str, id: &ProfileId) -> Result<Option<ProfileSourceData>, String> {
    log!("STUB: Fetching profile source for:", &id.0);
    
    Ok(Some(ProfileSourceData {
        source: "Demo source content".to_string(),
        version: "1.0.0".to_string(),
    }))
}

/// Delete profile - STUB IMPLEMENTATION
pub async fn delete_profile(_base_url: &str, id: &ProfileId) -> Result<(), String> {
    log!("STUB: Deleting profile:", &id.0);
    Ok(())
}

/// Update profile - STUB IMPLEMENTATION
pub async fn update_profile(_base_url: &str, id: &ProfileId, _update_data: ProfileUpdate) -> Result<(), String> {
    log!("STUB: Updating profile:", &id.0);
    Ok(())
}

/// Get workflows - STUB IMPLEMENTATION
pub async fn get_workflows(_base_url: &str) -> Result<Vec<WorkflowMetadata>, String> {
    log!("STUB: Fetching workflows");
    
    Ok(vec![
        WorkflowMetadata {
            id: WorkflowId("demo-workflow-1".to_string()),
            name: "Demo Workflow 1".to_string(),
            description: Some("A demo workflow".to_string()),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-20T00:00:00Z".to_string(),
        },
        WorkflowMetadata {
            id: WorkflowId("demo-workflow-2".to_string()),
            name: "Demo Workflow 2".to_string(),
            description: Some("Another demo workflow".to_string()),
            created_at: "2026-01-10T00:00:00Z".to_string(),
            updated_at: "2026-01-25T00:00:00Z".to_string(),
        },
    ])
}

/// Get workflow by ID - STUB IMPLEMENTATION
pub async fn get_workflow(_base_url: &str, workflow_id: WorkflowId) -> Result<Workflow, String> {
    log!("STUB: Fetching workflow:", &workflow_id.0);
    
    Ok(Workflow {
        id: workflow_id.clone(),
        name: format!("Workflow {}", workflow_id.0),
        description: Some("Demo workflow description".to_string()),
        content: serde_json::json!({
            "nodes": [],
            "edges": [],
            "version": "1.0"
        }),
    })
}
