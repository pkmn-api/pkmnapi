{% set api_domain = get_env(name="API_DOMAIN", default=config.extra.api_domain) %}
{% set api_host = api_domain | replace(from="http://", to="") | replace(from="https://", to="") %}
{% set lang = lang | default(value="http") %}

```{{ lang }}
{{ body | replace(from="{{API_DOMAIN}}", to=api_domain) | replace(from="{{API_HOST}}", to=api_host) }}
```
