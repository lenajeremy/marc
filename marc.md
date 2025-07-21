@import "person.json" as person

# Hello World

{% if name == "jeremiah" %}
### Hello Jeremiah
{% endif %}

{% if person.age > 15 %}
### Hello Jeremiah
{% endif %}
