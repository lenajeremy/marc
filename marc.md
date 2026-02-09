@import "person.json" as person

# Hello World

{% if name == "Jeremiah" %}
### Hello Jeremiah
{% endif %}

{% if person.age > 15 * 30 * 5 %}
### Hello Jeremiah
{% endif %}
