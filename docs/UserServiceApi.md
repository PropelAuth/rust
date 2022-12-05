# \UserServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_magic_link**](UserServiceApi.md#create_magic_link) | **POST** /api/backend/v1/magic_link | 
[**create_user**](UserServiceApi.md#create_user) | **POST** /api/backend/v1/user/ | 
[**delete_user**](UserServiceApi.md#delete_user) | **DELETE** /api/backend/v1/user/{user_id} | 
[**disable_user**](UserServiceApi.md#disable_user) | **POST** /api/backend/v1/user/{user_id}/disable | 
[**disable_user2fa**](UserServiceApi.md#disable_user2fa) | **POST** /api/backend/v1/user/{user_id}/disable_2fa | 
[**enable_user**](UserServiceApi.md#enable_user) | **POST** /api/backend/v1/user/{user_id}/enable | 
[**fetch_user_by_email**](UserServiceApi.md#fetch_user_by_email) | **GET** /api/backend/v1/user/email | 
[**fetch_user_by_id**](UserServiceApi.md#fetch_user_by_id) | **GET** /api/backend/v1/user/{user_id} | 
[**fetch_user_by_username**](UserServiceApi.md#fetch_user_by_username) | **GET** /api/backend/v1/user/username | 
[**fetch_users_by_emails**](UserServiceApi.md#fetch_users_by_emails) | **POST** /api/backend/v1/user/emails | 
[**fetch_users_by_ids**](UserServiceApi.md#fetch_users_by_ids) | **POST** /api/backend/v1/user/user_ids | 
[**fetch_users_by_query**](UserServiceApi.md#fetch_users_by_query) | **GET** /api/backend/v1/user/query | 
[**fetch_users_by_usernames**](UserServiceApi.md#fetch_users_by_usernames) | **POST** /api/backend/v1/user/usernames | 
[**migrate_user**](UserServiceApi.md#migrate_user) | **POST** /api/backend/v1/migrate_user/ | 
[**update_user_email**](UserServiceApi.md#update_user_email) | **PUT** /api/backend/v1/user/{user_id}/email | 
[**update_user_metadata**](UserServiceApi.md#update_user_metadata) | **PUT** /api/backend/v1/user/{user_id} | 
[**update_user_password**](UserServiceApi.md#update_user_password) | **PUT** /api/backend/v1/user/{user_id}/password | 



## create_magic_link

> crate::models::MagicLink create_magic_link(create_magic_link_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_magic_link_request** | [**CreateMagicLinkRequest**](CreateMagicLinkRequest.md) |  | [required] |

### Return type

[**crate::models::MagicLink**](MagicLink.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::CreatedUserResponse create_user(create_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::CreatedUserResponse**](CreatedUserResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::SuccessfulResponse delete_user(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_user

> crate::models::SuccessfulResponse disable_user(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_user2fa

> crate::models::SuccessfulResponse disable_user2fa(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_user

> crate::models::SuccessfulResponse enable_user(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_by_email

> crate::models::UserMetadata fetch_user_by_email(email, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**include_orgs** | Option<**bool**> | Defaults to false |  |

### Return type

[**crate::models::UserMetadata**](UserMetadata.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_by_id

> crate::models::UserMetadata fetch_user_by_id(user_id, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**include_orgs** | Option<**bool**> | Defaults to false |  |

### Return type

[**crate::models::UserMetadata**](UserMetadata.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_by_username

> crate::models::UserMetadata fetch_user_by_username(username, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**include_orgs** | Option<**bool**> | Defaults to false |  |

### Return type

[**crate::models::UserMetadata**](UserMetadata.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_by_emails

> Vec<crate::models::UserMetadata> fetch_users_by_emails(emails_query, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emails_query** | [**EmailsQuery**](EmailsQuery.md) |  | [required] |
**include_orgs** | Option<**bool**> | Defaults to false |  |

### Return type

[**Vec<crate::models::UserMetadata>**](UserMetadata.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_by_ids

> Vec<crate::models::UserMetadata> fetch_users_by_ids(user_ids_query, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ids_query** | [**UserIdsQuery**](UserIdsQuery.md) |  | [required] |
**include_orgs** | Option<**bool**> | Defaults to false |  |

### Return type

[**Vec<crate::models::UserMetadata>**](UserMetadata.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_by_query

> crate::models::UserPagedResponse fetch_users_by_query(page_size, page_number, order_by, email_or_username, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i64**> |  |  |
**page_number** | Option<**i64**> |  |  |
**order_by** | Option<[**FetchUsersOrderBy**](.md)> |  |  |
**email_or_username** | Option<**String**> |  |  |
**include_orgs** | Option<**bool**> |  |  |

### Return type

[**crate::models::UserPagedResponse**](UserPagedResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_by_usernames

> Vec<crate::models::UserMetadata> fetch_users_by_usernames(usernames_query, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usernames_query** | [**UsernamesQuery**](UsernamesQuery.md) |  | [required] |
**include_orgs** | Option<**bool**> | Defaults to false |  |

### Return type

[**Vec<crate::models::UserMetadata>**](UserMetadata.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_user

> crate::models::CreatedUserResponse migrate_user(migrate_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migrate_user_request** | [**MigrateUserRequest**](MigrateUserRequest.md) |  | [required] |

### Return type

[**crate::models::CreatedUserResponse**](CreatedUserResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_email

> crate::models::SuccessfulResponse update_user_email(user_id, update_email_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**update_email_request** | [**UpdateEmailRequest**](UpdateEmailRequest.md) |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_metadata

> crate::models::SuccessfulResponse update_user_metadata(user_id, update_metadata_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**update_metadata_request** | [**UpdateMetadataRequest**](UpdateMetadataRequest.md) |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_password

> crate::models::SuccessfulResponse update_user_password(user_id, update_password_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**update_password_request** | [**UpdatePasswordRequest**](UpdatePasswordRequest.md) |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

