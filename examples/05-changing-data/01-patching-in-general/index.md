## Patching In General

First, a little bit about patching in general.

Every change to the ROM generates a patch that contains information for how the source ROM should be updated to reflect the change.

Multiple patches may update the same area of the ROM with different data. As such, every change to the data can be identified by adding a description to the request by setting the `X-Patch-Description` header:

```bash
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H 'X-Patch-Description: Brief description of data updated' \
    -H 'Content-Type: application/json' \
    ...
```

[[Up]](../index.md) - [[Next - Updating BROCK's Name]](../02-updating-brocks-name/index.md)
