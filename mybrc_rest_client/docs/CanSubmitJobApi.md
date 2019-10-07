# \CanSubmitJobApi

All URIs are relative to *https://scgup-dev.lbl.gov/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**can_submit_job_read**](CanSubmitJobApi.md#can_submit_job_read) | **Get** /can_submit_job/{job_cost}/{user_id}/{account_id}/ | 


# **can_submit_job_read**
> ::models::InlineResponse200 can_submit_job_read(ctx, job_cost, user_id, account_id, optional)


Returns whether or not a Job with the given cost can be submitted by the given user for the given account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_cost** | **String**| A string representation of a nonnegative decimal number with no greater than 11 total digits and no greater than 2 decimal places. | 
  **user_id** | **String**| A string representation of the user&#39;s cluster UID, a five digit number. | 
  **account_id** | **String**| The name of the account. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **job_cost** | **String**| A string representation of a nonnegative decimal number with no greater than 11 total digits and no greater than 2 decimal places. | 
 **user_id** | **String**| A string representation of the user&#39;s cluster UID, a five digit number. | 
 **account_id** | **String**| The name of the account. | 
 **authorization** | **String**| The authorization token for the requester. The token should be preceded by &#39;Token &#39; (no quotes). | 

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

