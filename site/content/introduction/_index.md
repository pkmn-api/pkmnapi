+++
title = "Introduction"
weight = 1
+++

`pkmnapi` is a Pokémon REST API that uses a Gen 1 Game Boy ROM as its data source.

Using `pkmnapi`, it is possible to query and update a Pokémon Game Boy ROM using REST API conventions.

## Getting Started

Check out the section on [Getting Started](@/getting-started/_index.md).

## Base URL

The base URL for `pkmnapi` is: <a href="{{ api_domain() }}" target="_blank">{{ api_domain() }}</a>

## Rate-Limiting

Requests are capped at 120 per minute (AKA 2 per second). Limits may be raised in the future.

Additional requests over the rate-limit will return a `429 Too Many Requests` error.

## Authentication

`pkmnapi` uses access tokens and Bearer authentication for all requests (unless otherwise indicated).

Access tokens are emailed to the user, so no login or username is required.

For more information, see the section on [Using an Access Token](@/getting-started/using-an-access-token.md).

## Content-Type

All requests and responses will have a `Content-Type` of `application/json` (unless otherwise indicated).

## Updating Content

`pkmnapi` is not in the business of statically re-compiling Game Boy ROMs. As such, pointers will not be shuffled around to accomodate updated content.

All content must be (depending on the resource) the same exact size as the data it replaces or smaller.
