# Hello World

@import "data.json" as data;
@import "products.json" as products;

> this should be the only blockquote i get and this should work as expected 💕

Something interesting should happen

> this is a blockquote with **Bold** text as well as a [link something happens](https://google.com)

{% for product in products %}
Name: {{ product.name }}
Price: {{ product.price }}

{% if product.count == 0 %}
Out of stock
{% endif %}
{% endfor %}

{% include 'footer.md' %}
