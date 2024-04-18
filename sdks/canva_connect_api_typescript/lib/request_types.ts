// Generated by Sideko (sideko.dev)
import {
	TeamUser,
	Thumbnail,
	Design,
	GetDesignsResponse,
	GetDesignsOwnershipEnum,
	GetDesignsSortByEnum,
} from "./schemas";

export type ListAllDesignsRequest = {
	continuation?: string;
	ownership?: GetDesignsOwnershipEnum;
	query?: string;
	sortBy?: GetDesignsSortByEnum;
};