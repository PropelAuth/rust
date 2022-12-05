# \OrgServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_to_org**](OrgServiceApi.md#add_user_to_org) | **POST** /api/backend/v1/org/add_user | 
[**allow_org_to_enable_saml**](OrgServiceApi.md#allow_org_to_enable_saml) | **POST** /api/backend/v1/org/{org_id}/allow_saml | 
[**change_user_role_in_org**](OrgServiceApi.md#change_user_role_in_org) | **POST** /api/backend/v1/org/change_role | 
[**create_org**](OrgServiceApi.md#create_org) | **POST** /api/backend/v1/org/ | 
[**disallow_saml**](OrgServiceApi.md#disallow_saml) | **POST** /api/backend/v1/org/{org_id}/disallow_saml | 
[**fetch_org**](OrgServiceApi.md#fetch_org) | **GET** /api/backend/v1/org/{org_id} | 
[**fetch_orgs_by_query**](OrgServiceApi.md#fetch_orgs_by_query) | **GET** /api/backend/v1/org/query | 
[**fetch_users_in_org**](OrgServiceApi.md#fetch_users_in_org) | **GET** /api/backend/v1/user/org/{org_id} | 
[**remove_user_from_org**](OrgServiceApi.md#remove_user_from_org) | **POST** /api/backend/v1/org/remove_user | 
[**update_org**](OrgServiceApi.md#update_org) | **PUT** /api/backend/v1/org/{org_id} | 



## add_user_to_org

> crate::models::SuccessfulResponse add_user_to_org(add_user_to_org_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_user_to_org_request** | [**AddUserToOrgRequest**](AddUserToOrgRequest.md) |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allow_org_to_enable_saml

> crate::models::SuccessfulResponse allow_org_to_enable_saml(org_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_user_role_in_org

> crate::models::SuccessfulResponse change_user_role_in_org(change_user_role_in_org_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_user_role_in_org_request** | [**ChangeUserRoleInOrgRequest**](ChangeUserRoleInOrgRequest.md) |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_org

> crate::models::CreateOrgResponse create_org(create_org_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_org_request** | [**CreateOrgRequest**](CreateOrgRequest.md) |  | [required] |

### Return type

[**crate::models::CreateOrgResponse**](CreateOrgResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disallow_saml

> crate::models::SuccessfulResponse disallow_saml(org_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_org

> crate::models::FetchOrgResponse fetch_org(org_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**crate::models::FetchOrgResponse**](FetchOrgResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_orgs_by_query

> crate::models::FetchOrgsResponse fetch_orgs_by_query(page_size, page_number, order_by)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i64**> |  |  |
**page_number** | Option<**i64**> |  |  |
**order_by** | Option<[**FetchOrgOrderBy**](.md)> |  |  |

### Return type

[**crate::models::FetchOrgsResponse**](FetchOrgsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_users_in_org

> crate::models::UserPagedResponse fetch_users_in_org(org_id, page_size, page_number, include_orgs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**page_size** | Option<**i64**> |  |  |
**page_number** | Option<**i64**> |  |  |
**include_orgs** | Option<**bool**> | Defaults to false |  |

### Return type

[**crate::models::UserPagedResponse**](UserPagedResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_org

> crate::models::SuccessfulResponse remove_user_from_org(remove_user_from_org_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_user_from_org_request** | [**RemoveUserFromOrgRequest**](RemoveUserFromOrgRequest.md) |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org

> crate::models::SuccessfulResponse update_org(org_id, update_org_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**update_org_request** | [**UpdateOrgRequest**](UpdateOrgRequest.md) |  | [required] |

### Return type

[**crate::models::SuccessfulResponse**](SuccessfulResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

