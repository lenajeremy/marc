@import "person.json" as person

{% fn fullName(first, last) %}
{% return first + " " + last %}
{% endfn %}

{% if person.age >= 18 %}
Hello {{ fullName(person.first, person.last) }}
{% endif %}
