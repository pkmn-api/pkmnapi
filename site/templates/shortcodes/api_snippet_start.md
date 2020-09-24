<div class="snippet">
<div class="snippet__langs">
{% for lang in langs %}
<div class="snippet__lang{% if loop.first %} selected{% endif %}" data-lang="{{ lang }}">
    {{ lang }}
</div>
{% endfor %}
</div>
