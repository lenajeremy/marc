@import "person.json" as person

# Hello World

{% if person.name == "Jeremiah" %}
### Hello Jeremiah
{% endif %}

{% if person.age > 15 * 30 * 5 %}
### Hello Jeremiah
{% endif %}

Total: {{ 5 + 5 * 2 }}
Grouped: {{ (5 + 5) * 2 }}
Prefix: {{ -5 }} and {{ !5 }} and {{ +5 }}
Chained: {{ 1 + 2 + 3 }}

Array access: {{ array[0] }}
Array index expression: {{ array[1 + 1] }}
Nested array access: {{ arr[arr[0]] }}

Object access: {{ user.address.street }}
Object access after array: {{ users[0].name }}
Object access after function: {{ getUser().name }}

Function call: {{ foo(bar, baz, 10) }}
Function call with expressions: {{ add(1 + 2, a * b) }}
Nested function call: {{ outer(inner()) }}
Chained function call: {{ obj.method(arg) }}

Mixed access: {{ getArray()[0].name }}
