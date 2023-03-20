# Rust API client for bitwarden-api-identity

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project. By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `bitwarden-api-identity` and add the following to `Cargo.toml` under `[dependencies]`:

```
bitwarden-api-identity = { path = "./bitwarden-api-identity" }
```

## Documentation for API Endpoints

All URIs are relative to _http://localhost_

| Class         | Method                                                                              | HTTP request                       | Description |
| ------------- | ----------------------------------------------------------------------------------- | ---------------------------------- | ----------- |
| _AccountsApi_ | [**accounts_prelogin_post**](docs/AccountsApi.md#accounts_prelogin_post)            | **POST** /accounts/prelogin        |
| _AccountsApi_ | [**accounts_register_post**](docs/AccountsApi.md#accounts_register_post)            | **POST** /accounts/register        |
| _InfoApi_     | [**alive_get**](docs/InfoApi.md#alive_get)                                          | **GET** /alive                     |
| _InfoApi_     | [**now_get**](docs/InfoApi.md#now_get)                                              | **GET** /now                       |
| _InfoApi_     | [**version_get**](docs/InfoApi.md#version_get)                                      | **GET** /version                   |
| _SsoApi_      | [**account_external_callback_get**](docs/SsoApi.md#account_external_callback_get)   | **GET** /account/ExternalCallback  |
| _SsoApi_      | [**account_external_challenge_get**](docs/SsoApi.md#account_external_challenge_get) | **GET** /account/ExternalChallenge |
| _SsoApi_      | [**account_login_get**](docs/SsoApi.md#account_login_get)                           | **GET** /account/Login             |
| _SsoApi_      | [**account_pre_validate_get**](docs/SsoApi.md#account_pre_validate_get)             | **GET** /account/PreValidate       |
| _SsoApi_      | [**sso_external_callback_get**](docs/SsoApi.md#sso_external_callback_get)           | **GET** /sso/ExternalCallback      |
| _SsoApi_      | [**sso_external_challenge_get**](docs/SsoApi.md#sso_external_challenge_get)         | **GET** /sso/ExternalChallenge     |
| _SsoApi_      | [**sso_login_get**](docs/SsoApi.md#sso_login_get)                                   | **GET** /sso/Login                 |
| _SsoApi_      | [**sso_pre_validate_get**](docs/SsoApi.md#sso_pre_validate_get)                     | **GET** /sso/PreValidate           |

## Documentation For Models

- [KdfType](docs/KdfType.md)
- [KeysRequestModel](docs/KeysRequestModel.md)
- [PreloginRequestModel](docs/PreloginRequestModel.md)
- [PreloginResponseModel](docs/PreloginResponseModel.md)
- [RegisterRequestModel](docs/RegisterRequestModel.md)
- [RegisterResponseModel](docs/RegisterResponseModel.md)

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author