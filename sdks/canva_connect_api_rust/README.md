# canva_connect_api rust 

 The Canva Connect API facilitates third-party app developers to extend key Canva capabilities off-platform. It allows Canva to share data with third parties, even when a user is not actively designing in Canva.
You can use the Canva Connect API to create integrations with third-party apps, keep Canva in sync with cloud storage and content management apps, collaborate and communicate with different platforms, or embed Canva into project management workflows.
For instance, you could create an integration (we call this a client) between Canva and your digital asset management system using the Connect API, to create Canva folders and upload your assets directly to these folders.
This is in contrast to the Canva App SDKs which bring third-party capabilities to Canva (on-platform), to enhance user experience designing in Canva.
 

 # Authentication 
  
 ```rust
use canva_connect_api::Client;

let client = Client::default().with_bearer_auth(&std::env::var("API_TOKEN").expect("API token not defined"));
```

# list_all_designs

```rust
use canva_connect_api::Client;
use canva_connect_api::request_types::*;
use canva_connect_api::schemas::*;
let client = Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").expect("API token not defined"));
let response = client
    .list_all_designs(ListAllDesignsRequest {
        continuation: Some("597ad709-4e0d-4714-8e4f-19f34bffe473".to_string()),
        ownership: Some(GetDesignsOwnershipEnum::Owned),
        query: Some("logos".to_string()),
        sort_by: Some(GetDesignsSortByEnum::Relevance),
    });
```
