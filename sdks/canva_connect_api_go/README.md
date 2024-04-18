# canva_connect_api go 

 The Canva Connect API facilitates third-party app developers to extend key Canva capabilities off-platform. It allows Canva to share data with third parties, even when a user is not actively designing in Canva.
You can use the Canva Connect API to create integrations with third-party apps, keep Canva in sync with cloud storage and content management apps, collaborate and communicate with different platforms, or embed Canva into project management workflows.
For instance, you could create an integration (we call this a client) between Canva and your digital asset management system using the Connect API, to create Canva folders and upload your assets directly to these folders.
This is in contrast to the Canva App SDKs which bring third-party capabilities to Canva (on-platform), to enhance user experience designing in Canva.
 

 # Authentication 
  
 ```go
import "os"
import "github.com/Sideko-Inc/canva_connect_api"

client := canva_connect_api.NewClient(canva_connect_api.WithBearerAuth(os.Getenv("API_TOKEN")))
```

# list_all_designs

```go
import "os"
import "github.com/Sideko-Inc/canva_connect_api"

client := canva_connect_api.NewClient(canva_connect_api.WithBearerAuth(os.Getenv("API_TOKEN")))

response, err := client.ListAllDesigns(canva_connect_api.ListAllDesignsRequest { Continuation: "597ad709-4e0d-4714-8e4f-19f34bffe473", Ownership: canva_connect_api.GetDesignsOwnershipEnumOwned, Query: "logos", SortBy: canva_connect_api.GetDesignsSortByEnumRelevance })
```
