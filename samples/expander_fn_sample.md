@import "person.json" as person

{% fn add(a, b) %}
sum = 50 + a * b
print(sum)
return sum
{% endfn %}

{% fn fullName(first, last) %}
return first + " " + last
{% endfn %}

{% if person.age >= 18 %}
Hello {{ fullName(person.first, person.last) }}
{% endif %}
