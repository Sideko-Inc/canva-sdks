// Generated by Sideko (sideko.dev)
package canva_connect_api

type BinaryResponse struct {
	Content []byte
	Header  map[string][]string
}

type TeamUser struct {
	DisplayName *string `json:"display_name,omitempty"`
	TeamId      *string `json:"team_id,omitempty"`
	UserId      *string `json:"user_id,omitempty"`
}

type Thumbnail struct {
	Height *int    `json:"height,omitempty"`
	Url    *string `json:"url,omitempty"`
	Width  *string `json:"width,omitempty"`
}

type Design struct {
	DoctypeName *string    `json:"doctype_name,omitempty"`
	Id          *string    `json:"id,omitempty"`
	Owner       *TeamUser  `json:"owner,omitempty"`
	Thumbnail   *Thumbnail `json:"thumbnail,omitempty"`
	Title       *string    `json:"title,omitempty"`
}

type GetDesignsResponse struct {
	Continuation *string   `json:"continuation,omitempty"`
	Items        *[]Design `json:"items,omitempty"`
}

type GetDesignsOwnershipEnum string

const (
	GetDesignsOwnershipEnumOwned  GetDesignsOwnershipEnum = "OWNED"
	GetDesignsOwnershipEnumShared GetDesignsOwnershipEnum = "SHARED"
	GetDesignsOwnershipEnumAny    GetDesignsOwnershipEnum = "ANY"
)

type GetDesignsSortByEnum string

const (
	GetDesignsSortByEnumRelevance          GetDesignsSortByEnum = "RELEVANCE"
	GetDesignsSortByEnumModifiedDescending GetDesignsSortByEnum = "MODIFIED_DESCENDING"
	GetDesignsSortByEnumModifiedAscending  GetDesignsSortByEnum = "MODIFIED_ASCENDING"
	GetDesignsSortByEnumTitleDescending    GetDesignsSortByEnum = "TITLE_DESCENDING"
	GetDesignsSortByEnumTitleAscending     GetDesignsSortByEnum = "TITLE_ASCENDING"
)
