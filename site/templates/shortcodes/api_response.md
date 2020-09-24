{% set api_domain = get_env(name="API_DOMAIN", default=config.extra.api_domain) %}
{% set lang = type | default(value="json") %}

{% if body %}
```{{ lang }}
{{ body | replace(from="{{API_DOMAIN}}", to=api_domain) }}
```
{% else %}
None
{% endif %}
