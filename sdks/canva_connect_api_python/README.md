# canva_connect_api python 

 The Canva Connect API facilitates third-party app developers to extend key Canva capabilities off-platform. It allows Canva to share data with third parties, even when a user is not actively designing in Canva.
You can use the Canva Connect API to create integrations with third-party apps, keep Canva in sync with cloud storage and content management apps, collaborate and communicate with different platforms, or embed Canva into project management workflows.
For instance, you could create an integration (we call this a client) between Canva and your digital asset management system using the Connect API, to create Canva folders and upload your assets directly to these folders.
This is in contrast to the Canva App SDKs which bring third-party capabilities to Canva (on-platform), to enhance user experience designing in Canva.
 

 # Authentication 
  
 ```python
from os import getenv
from canva_connect_api import AsyncClient, Client

# Synchronous client initialization
client = Client(bearer_auth=getenv("API_TOKEN"))

# Asynchronous client initialization
async_client = AsyncClient(bearer_auth=getenv("API_TOKEN"))
```

# list_all_designs

```python
from os import getenv
from canva_connect_api import Client, AsyncClient
from canva_connect_api.schemas import *

# Synchronous client initialization
client = Client(bearer_auth=getenv("API_TOKEN"))

# Asynchronous client initialization
async_client = AsyncClient(bearer_auth=getenv("API_TOKEN"))

# Call synchronous method
response = client.list_all_designs(
    continuation="597ad709-4e0d-4714-8e4f-19f34bffe473",
    ownership=GetDesignsOwnershipEnum.OWNED,
    query="logos",
    sort_by=GetDesignsSortByEnum.RELEVANCE,
)

# Call asynchronous method
async_response = await async_client.list_all_designs(
    continuation="597ad709-4e0d-4714-8e4f-19f34bffe473",
    ownership=GetDesignsOwnershipEnum.OWNED,
    query="logos",
    sort_by=GetDesignsSortByEnum.RELEVANCE,
)

```
