{% set api_domain = get_env(name="API_DOMAIN", default=config.extra.api_domain) %}
{% set lang = lang | default(value="bash") %}

{% if body %}
<div class="snippet__code{% if hidden %} hidden{% endif %}" data-lang="{{ lang }}">

```{{ lang }}
{{ body | replace(from="{{API_DOMAIN}}", to=api_domain) }}
```

</div>
{% else %}
{% if raw %}
`<raw>`
{% else %}
None
{% endif %}
{% endif %}
