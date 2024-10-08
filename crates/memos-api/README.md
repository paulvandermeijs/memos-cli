# Rust API client for memos-api

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 0.1.0
- Generator version: 7.8.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `memos-api` and add the following to `Cargo.toml` under `[dependencies]`:

```
memos-api = { path = "./memos-api" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ActivityServiceApi* | [**activity_service_get_activity**](docs/ActivityServiceApi.md#activity_service_get_activity) | **GET** /api/v1/{name} | GetActivity returns the activity with the given id.
*AuthServiceApi* | [**auth_service_get_auth_status**](docs/AuthServiceApi.md#auth_service_get_auth_status) | **POST** /api/v1/auth/status | GetAuthStatus returns the current auth status of the user.
*AuthServiceApi* | [**auth_service_sign_in**](docs/AuthServiceApi.md#auth_service_sign_in) | **POST** /api/v1/auth/signin | SignIn signs in the user with the given username and password.
*AuthServiceApi* | [**auth_service_sign_in_with_sso**](docs/AuthServiceApi.md#auth_service_sign_in_with_sso) | **POST** /api/v1/auth/signin/sso | SignInWithSSO signs in the user with the given SSO code.
*AuthServiceApi* | [**auth_service_sign_out**](docs/AuthServiceApi.md#auth_service_sign_out) | **POST** /api/v1/auth/signout | SignOut signs out the user.
*AuthServiceApi* | [**auth_service_sign_up**](docs/AuthServiceApi.md#auth_service_sign_up) | **POST** /api/v1/auth/signup | SignUp signs up the user with the given username and password.
*IdentityProviderServiceApi* | [**identity_provider_service_create_identity_provider**](docs/IdentityProviderServiceApi.md#identity_provider_service_create_identity_provider) | **POST** /api/v1/identityProviders | CreateIdentityProvider creates an identity provider.
*IdentityProviderServiceApi* | [**identity_provider_service_delete_identity_provider**](docs/IdentityProviderServiceApi.md#identity_provider_service_delete_identity_provider) | **DELETE** /api/v1/{name_1} | DeleteIdentityProvider deletes an identity provider.
*IdentityProviderServiceApi* | [**identity_provider_service_get_identity_provider**](docs/IdentityProviderServiceApi.md#identity_provider_service_get_identity_provider) | **GET** /api/v1/{name_2} | GetIdentityProvider gets an identity provider.
*IdentityProviderServiceApi* | [**identity_provider_service_list_identity_providers**](docs/IdentityProviderServiceApi.md#identity_provider_service_list_identity_providers) | **GET** /api/v1/identityProviders | ListIdentityProviders lists identity providers.
*IdentityProviderServiceApi* | [**identity_provider_service_update_identity_provider**](docs/IdentityProviderServiceApi.md#identity_provider_service_update_identity_provider) | **PATCH** /api/v1/{identityProvider_name} | UpdateIdentityProvider updates an identity provider.
*InboxServiceApi* | [**inbox_service_delete_inbox**](docs/InboxServiceApi.md#inbox_service_delete_inbox) | **DELETE** /api/v1/{name_2} | DeleteInbox deletes an inbox.
*InboxServiceApi* | [**inbox_service_list_inboxes**](docs/InboxServiceApi.md#inbox_service_list_inboxes) | **GET** /api/v1/inboxes | ListInboxes lists inboxes for a user.
*InboxServiceApi* | [**inbox_service_update_inbox**](docs/InboxServiceApi.md#inbox_service_update_inbox) | **PATCH** /api/v1/{inbox_name} | UpdateInbox updates an inbox.
*MarkdownServiceApi* | [**markdown_service_get_link_metadata**](docs/MarkdownServiceApi.md#markdown_service_get_link_metadata) | **GET** /api/v1/markdown/link:metadata | GetLinkMetadata returns metadata for a given link.
*MarkdownServiceApi* | [**markdown_service_parse_markdown**](docs/MarkdownServiceApi.md#markdown_service_parse_markdown) | **POST** /api/v1/markdown:parse | ParseMarkdown parses the given markdown content and returns a list of nodes.
*MarkdownServiceApi* | [**markdown_service_restore_markdown_nodes**](docs/MarkdownServiceApi.md#markdown_service_restore_markdown_nodes) | **POST** /api/v1/markdown/node:restore | RestoreMarkdownNodes restores the given nodes to markdown content.
*MarkdownServiceApi* | [**markdown_service_stringify_markdown_nodes**](docs/MarkdownServiceApi.md#markdown_service_stringify_markdown_nodes) | **POST** /api/v1/markdown/node:stringify | StringifyMarkdownNodes stringify the given nodes to plain text content.
*MemoServiceApi* | [**memo_service_create_memo**](docs/MemoServiceApi.md#memo_service_create_memo) | **POST** /api/v1/memos | CreateMemo creates a memo.
*MemoServiceApi* | [**memo_service_create_memo_comment**](docs/MemoServiceApi.md#memo_service_create_memo_comment) | **POST** /api/v1/{name}/comments | CreateMemoComment creates a comment for a memo.
*MemoServiceApi* | [**memo_service_delete_memo**](docs/MemoServiceApi.md#memo_service_delete_memo) | **DELETE** /api/v1/{name_4} | DeleteMemo deletes a memo.
*MemoServiceApi* | [**memo_service_delete_memo_reaction**](docs/MemoServiceApi.md#memo_service_delete_memo_reaction) | **DELETE** /api/v1/reactions/{reactionId} | DeleteMemoReaction deletes a reaction for a memo.
*MemoServiceApi* | [**memo_service_delete_memo_tag**](docs/MemoServiceApi.md#memo_service_delete_memo_tag) | **DELETE** /api/v1/{parent}/tags/{tag} | DeleteMemoTag deletes a tag for a memo.
*MemoServiceApi* | [**memo_service_get_memo**](docs/MemoServiceApi.md#memo_service_get_memo) | **GET** /api/v1/{name_4} | GetMemo gets a memo.
*MemoServiceApi* | [**memo_service_get_memo_by_uid**](docs/MemoServiceApi.md#memo_service_get_memo_by_uid) | **GET** /api/v1/memos:by-uid/{uid} | GetMemoByUid gets a memo by uid
*MemoServiceApi* | [**memo_service_list_memo_comments**](docs/MemoServiceApi.md#memo_service_list_memo_comments) | **GET** /api/v1/{name}/comments | ListMemoComments lists comments for a memo.
*MemoServiceApi* | [**memo_service_list_memo_properties**](docs/MemoServiceApi.md#memo_service_list_memo_properties) | **GET** /api/v1/{name}/properties | ListMemoProperties lists memo properties.
*MemoServiceApi* | [**memo_service_list_memo_reactions**](docs/MemoServiceApi.md#memo_service_list_memo_reactions) | **GET** /api/v1/{name}/reactions | ListMemoReactions lists reactions for a memo.
*MemoServiceApi* | [**memo_service_list_memo_relations**](docs/MemoServiceApi.md#memo_service_list_memo_relations) | **GET** /api/v1/{name}/relations | ListMemoRelations lists relations for a memo.
*MemoServiceApi* | [**memo_service_list_memo_resources**](docs/MemoServiceApi.md#memo_service_list_memo_resources) | **GET** /api/v1/{name}/resources | ListMemoResources lists resources for a memo.
*MemoServiceApi* | [**memo_service_list_memo_tags**](docs/MemoServiceApi.md#memo_service_list_memo_tags) | **GET** /api/v1/{parent}/tags | ListMemoTags lists tags for a memo.
*MemoServiceApi* | [**memo_service_list_memos**](docs/MemoServiceApi.md#memo_service_list_memos) | **GET** /api/v1/memos | ListMemos lists memos with pagination and filter.
*MemoServiceApi* | [**memo_service_rebuild_memo_property**](docs/MemoServiceApi.md#memo_service_rebuild_memo_property) | **POST** /api/v1/{name}/properties:rebuild | RebuildMemoProperty rebuilds a memo property.
*MemoServiceApi* | [**memo_service_rename_memo_tag**](docs/MemoServiceApi.md#memo_service_rename_memo_tag) | **PATCH** /api/v1/{parent}/tags:rename | RenameMemoTag renames a tag for a memo.
*MemoServiceApi* | [**memo_service_set_memo_relations**](docs/MemoServiceApi.md#memo_service_set_memo_relations) | **PATCH** /api/v1/{name}/relations | SetMemoRelations sets relations for a memo.
*MemoServiceApi* | [**memo_service_set_memo_resources**](docs/MemoServiceApi.md#memo_service_set_memo_resources) | **PATCH** /api/v1/{name}/resources | SetMemoResources sets resources for a memo.
*MemoServiceApi* | [**memo_service_update_memo**](docs/MemoServiceApi.md#memo_service_update_memo) | **PATCH** /api/v1/{memo_name} | UpdateMemo updates a memo.
*MemoServiceApi* | [**memo_service_upsert_memo_reaction**](docs/MemoServiceApi.md#memo_service_upsert_memo_reaction) | **POST** /api/v1/{name}/reactions | UpsertMemoReaction upserts a reaction for a memo.
*ResourceServiceApi* | [**resource_service_create_resource**](docs/ResourceServiceApi.md#resource_service_create_resource) | **POST** /api/v1/resources | CreateResource creates a new resource.
*ResourceServiceApi* | [**resource_service_delete_resource**](docs/ResourceServiceApi.md#resource_service_delete_resource) | **DELETE** /api/v1/{name_3} | DeleteResource deletes a resource by name.
*ResourceServiceApi* | [**resource_service_get_resource**](docs/ResourceServiceApi.md#resource_service_get_resource) | **GET** /api/v1/{name_3} | GetResource returns a resource by name.
*ResourceServiceApi* | [**resource_service_get_resource_binary**](docs/ResourceServiceApi.md#resource_service_get_resource_binary) | **GET** /file/{name}/{filename} | GetResourceBinary returns a resource binary by name.
*ResourceServiceApi* | [**resource_service_get_resource_by_uid**](docs/ResourceServiceApi.md#resource_service_get_resource_by_uid) | **GET** /api/v1/resources:by-uid/{uid} | GetResourceByUid returns a resource by uid.
*ResourceServiceApi* | [**resource_service_list_resources**](docs/ResourceServiceApi.md#resource_service_list_resources) | **GET** /api/v1/resources | ListResources lists all resources.
*ResourceServiceApi* | [**resource_service_update_resource**](docs/ResourceServiceApi.md#resource_service_update_resource) | **PATCH** /api/v1/{resource_name} | UpdateResource updates a resource.
*UserServiceApi* | [**user_service_create_user**](docs/UserServiceApi.md#user_service_create_user) | **POST** /api/v1/users | CreateUser creates a new user.
*UserServiceApi* | [**user_service_create_user_access_token**](docs/UserServiceApi.md#user_service_create_user_access_token) | **POST** /api/v1/{name}/access_tokens | CreateUserAccessToken creates a new access token for a user.
*UserServiceApi* | [**user_service_delete_user**](docs/UserServiceApi.md#user_service_delete_user) | **DELETE** /api/v1/{name} | DeleteUser deletes a user.
*UserServiceApi* | [**user_service_delete_user_access_token**](docs/UserServiceApi.md#user_service_delete_user_access_token) | **DELETE** /api/v1/{name}/access_tokens/{accessToken} | DeleteUserAccessToken deletes an access token for a user.
*UserServiceApi* | [**user_service_get_user**](docs/UserServiceApi.md#user_service_get_user) | **GET** /api/v1/{name_1} | GetUser gets a user by name.
*UserServiceApi* | [**user_service_get_user_avatar_binary**](docs/UserServiceApi.md#user_service_get_user_avatar_binary) | **GET** /file/{name}/avatar | GetUserAvatarBinary gets the avatar of a user.
*UserServiceApi* | [**user_service_get_user_setting**](docs/UserServiceApi.md#user_service_get_user_setting) | **GET** /api/v1/{name}/setting | GetUserSetting gets the setting of a user.
*UserServiceApi* | [**user_service_list_user_access_tokens**](docs/UserServiceApi.md#user_service_list_user_access_tokens) | **GET** /api/v1/{name}/access_tokens | ListUserAccessTokens returns a list of access tokens for a user.
*UserServiceApi* | [**user_service_list_users**](docs/UserServiceApi.md#user_service_list_users) | **GET** /api/v1/users | ListUsers returns a list of users.
*UserServiceApi* | [**user_service_search_users**](docs/UserServiceApi.md#user_service_search_users) | **GET** /api/v1/users:search | SearchUsers searches users by filter.
*UserServiceApi* | [**user_service_update_user**](docs/UserServiceApi.md#user_service_update_user) | **PATCH** /api/v1/{user_name} | UpdateUser updates a user.
*UserServiceApi* | [**user_service_update_user_setting**](docs/UserServiceApi.md#user_service_update_user_setting) | **PATCH** /api/v1/{setting_name} | UpdateUserSetting updates the setting of a user.
*WebhookServiceApi* | [**webhook_service_create_webhook**](docs/WebhookServiceApi.md#webhook_service_create_webhook) | **POST** /api/v1/webhooks | CreateWebhook creates a new webhook.
*WebhookServiceApi* | [**webhook_service_delete_webhook**](docs/WebhookServiceApi.md#webhook_service_delete_webhook) | **DELETE** /api/v1/webhooks/{id} | DeleteWebhook deletes a webhook by id.
*WebhookServiceApi* | [**webhook_service_get_webhook**](docs/WebhookServiceApi.md#webhook_service_get_webhook) | **GET** /api/v1/webhooks/{id} | GetWebhook returns a webhook by id.
*WebhookServiceApi* | [**webhook_service_list_webhooks**](docs/WebhookServiceApi.md#webhook_service_list_webhooks) | **GET** /api/v1/webhooks | ListWebhooks returns a list of webhooks.
*WebhookServiceApi* | [**webhook_service_update_webhook**](docs/WebhookServiceApi.md#webhook_service_update_webhook) | **PATCH** /api/v1/webhooks/{webhook_id} | UpdateWebhook updates a webhook.
*WorkspaceServiceApi* | [**workspace_service_get_workspace_profile**](docs/WorkspaceServiceApi.md#workspace_service_get_workspace_profile) | **GET** /api/v1/workspace/profile | GetWorkspaceProfile returns the workspace profile.
*WorkspaceSettingServiceApi* | [**workspace_setting_service_get_workspace_setting**](docs/WorkspaceSettingServiceApi.md#workspace_setting_service_get_workspace_setting) | **GET** /api/v1/workspace/{name} | GetWorkspaceSetting returns the setting by name.
*WorkspaceSettingServiceApi* | [**workspace_setting_service_set_workspace_setting**](docs/WorkspaceSettingServiceApi.md#workspace_setting_service_set_workspace_setting) | **PATCH** /api/v1/workspace/{setting_name} | SetWorkspaceSetting updates the setting.


## Documentation For Models

 - [ApiHttpBody](docs/ApiHttpBody.md)
 - [Apiv1ActivityMemoCommentPayload](docs/Apiv1ActivityMemoCommentPayload.md)
 - [Apiv1ActivityPayload](docs/Apiv1ActivityPayload.md)
 - [Apiv1ActivityVersionUpdatePayload](docs/Apiv1ActivityVersionUpdatePayload.md)
 - [Apiv1FieldMapping](docs/Apiv1FieldMapping.md)
 - [Apiv1IdentityProvider](docs/Apiv1IdentityProvider.md)
 - [Apiv1IdentityProviderConfig](docs/Apiv1IdentityProviderConfig.md)
 - [Apiv1IdentityProviderType](docs/Apiv1IdentityProviderType.md)
 - [Apiv1OAuth2Config](docs/Apiv1OAuth2Config.md)
 - [Apiv1UserSetting](docs/Apiv1UserSetting.md)
 - [Apiv1WorkspaceCustomProfile](docs/Apiv1WorkspaceCustomProfile.md)
 - [Apiv1WorkspaceGeneralSetting](docs/Apiv1WorkspaceGeneralSetting.md)
 - [Apiv1WorkspaceMemoRelatedSetting](docs/Apiv1WorkspaceMemoRelatedSetting.md)
 - [Apiv1WorkspaceSetting](docs/Apiv1WorkspaceSetting.md)
 - [Apiv1WorkspaceStorageSetting](docs/Apiv1WorkspaceStorageSetting.md)
 - [Apiv1WorkspaceStorageSettingStorageType](docs/Apiv1WorkspaceStorageSettingStorageType.md)
 - [GooglerpcStatus](docs/GooglerpcStatus.md)
 - [InboxServiceUpdateInboxRequest](docs/InboxServiceUpdateInboxRequest.md)
 - [ListNodeKind](docs/ListNodeKind.md)
 - [MemoServiceRenameMemoTagBody](docs/MemoServiceRenameMemoTagBody.md)
 - [MemoServiceSetMemoRelationsBody](docs/MemoServiceSetMemoRelationsBody.md)
 - [MemoServiceSetMemoResourcesBody](docs/MemoServiceSetMemoResourcesBody.md)
 - [MemoServiceUpdateMemoRequest](docs/MemoServiceUpdateMemoRequest.md)
 - [MemoServiceUpsertMemoReactionBody](docs/MemoServiceUpsertMemoReactionBody.md)
 - [ProtobufAny](docs/ProtobufAny.md)
 - [ResourceServiceUpdateResourceRequest](docs/ResourceServiceUpdateResourceRequest.md)
 - [SettingIsTheSettingToUpdate](docs/SettingIsTheSettingToUpdate.md)
 - [TableNodeRow](docs/TableNodeRow.md)
 - [TheIdentityProviderToUpdate](docs/TheIdentityProviderToUpdate.md)
 - [UserRole](docs/UserRole.md)
 - [UserServiceCreateUserAccessTokenBody](docs/UserServiceCreateUserAccessTokenBody.md)
 - [UserServiceUpdateUserRequest](docs/UserServiceUpdateUserRequest.md)
 - [UserServiceUpdateUserSettingRequest](docs/UserServiceUpdateUserSettingRequest.md)
 - [V1Activity](docs/V1Activity.md)
 - [V1AutoLinkNode](docs/V1AutoLinkNode.md)
 - [V1BlockquoteNode](docs/V1BlockquoteNode.md)
 - [V1BoldItalicNode](docs/V1BoldItalicNode.md)
 - [V1BoldNode](docs/V1BoldNode.md)
 - [V1CodeBlockNode](docs/V1CodeBlockNode.md)
 - [V1CodeNode](docs/V1CodeNode.md)
 - [V1CreateMemoRequest](docs/V1CreateMemoRequest.md)
 - [V1CreateWebhookRequest](docs/V1CreateWebhookRequest.md)
 - [V1EmbeddedContentNode](docs/V1EmbeddedContentNode.md)
 - [V1EscapingCharacterNode](docs/V1EscapingCharacterNode.md)
 - [V1HeadingNode](docs/V1HeadingNode.md)
 - [V1HighlightNode](docs/V1HighlightNode.md)
 - [V1HorizontalRuleNode](docs/V1HorizontalRuleNode.md)
 - [V1HtmlElementNode](docs/V1HtmlElementNode.md)
 - [V1ImageNode](docs/V1ImageNode.md)
 - [V1Inbox](docs/V1Inbox.md)
 - [V1InboxStatus](docs/V1InboxStatus.md)
 - [V1InboxType](docs/V1InboxType.md)
 - [V1ItalicNode](docs/V1ItalicNode.md)
 - [V1LinkMetadata](docs/V1LinkMetadata.md)
 - [V1LinkNode](docs/V1LinkNode.md)
 - [V1ListIdentityProvidersResponse](docs/V1ListIdentityProvidersResponse.md)
 - [V1ListInboxesResponse](docs/V1ListInboxesResponse.md)
 - [V1ListMemoCommentsResponse](docs/V1ListMemoCommentsResponse.md)
 - [V1ListMemoPropertiesResponse](docs/V1ListMemoPropertiesResponse.md)
 - [V1ListMemoReactionsResponse](docs/V1ListMemoReactionsResponse.md)
 - [V1ListMemoRelationsResponse](docs/V1ListMemoRelationsResponse.md)
 - [V1ListMemoResourcesResponse](docs/V1ListMemoResourcesResponse.md)
 - [V1ListMemoTagsResponse](docs/V1ListMemoTagsResponse.md)
 - [V1ListMemosResponse](docs/V1ListMemosResponse.md)
 - [V1ListNode](docs/V1ListNode.md)
 - [V1ListResourcesResponse](docs/V1ListResourcesResponse.md)
 - [V1ListUserAccessTokensResponse](docs/V1ListUserAccessTokensResponse.md)
 - [V1ListUsersResponse](docs/V1ListUsersResponse.md)
 - [V1ListWebhooksResponse](docs/V1ListWebhooksResponse.md)
 - [V1MathBlockNode](docs/V1MathBlockNode.md)
 - [V1MathNode](docs/V1MathNode.md)
 - [V1Memo](docs/V1Memo.md)
 - [V1MemoProperty](docs/V1MemoProperty.md)
 - [V1MemoPropertyEntity](docs/V1MemoPropertyEntity.md)
 - [V1MemoRelation](docs/V1MemoRelation.md)
 - [V1MemoRelationType](docs/V1MemoRelationType.md)
 - [V1Node](docs/V1Node.md)
 - [V1NodeType](docs/V1NodeType.md)
 - [V1OrderedListItemNode](docs/V1OrderedListItemNode.md)
 - [V1ParagraphNode](docs/V1ParagraphNode.md)
 - [V1ParseMarkdownRequest](docs/V1ParseMarkdownRequest.md)
 - [V1ParseMarkdownResponse](docs/V1ParseMarkdownResponse.md)
 - [V1Reaction](docs/V1Reaction.md)
 - [V1ReactionType](docs/V1ReactionType.md)
 - [V1ReferencedContentNode](docs/V1ReferencedContentNode.md)
 - [V1Resource](docs/V1Resource.md)
 - [V1RestoreMarkdownNodesRequest](docs/V1RestoreMarkdownNodesRequest.md)
 - [V1RestoreMarkdownNodesResponse](docs/V1RestoreMarkdownNodesResponse.md)
 - [V1RowStatus](docs/V1RowStatus.md)
 - [V1SearchUsersResponse](docs/V1SearchUsersResponse.md)
 - [V1SpoilerNode](docs/V1SpoilerNode.md)
 - [V1StrikethroughNode](docs/V1StrikethroughNode.md)
 - [V1StringifyMarkdownNodesRequest](docs/V1StringifyMarkdownNodesRequest.md)
 - [V1StringifyMarkdownNodesResponse](docs/V1StringifyMarkdownNodesResponse.md)
 - [V1SubscriptNode](docs/V1SubscriptNode.md)
 - [V1SuperscriptNode](docs/V1SuperscriptNode.md)
 - [V1TableNode](docs/V1TableNode.md)
 - [V1TagNode](docs/V1TagNode.md)
 - [V1TaskListItemNode](docs/V1TaskListItemNode.md)
 - [V1TextNode](docs/V1TextNode.md)
 - [V1UnorderedListItemNode](docs/V1UnorderedListItemNode.md)
 - [V1User](docs/V1User.md)
 - [V1UserAccessToken](docs/V1UserAccessToken.md)
 - [V1Visibility](docs/V1Visibility.md)
 - [V1Webhook](docs/V1Webhook.md)
 - [V1WorkspaceProfile](docs/V1WorkspaceProfile.md)
 - [WebhookServiceUpdateWebhookRequest](docs/WebhookServiceUpdateWebhookRequest.md)
 - [WorkspaceStorageSettingS3Config](docs/WorkspaceStorageSettingS3Config.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



