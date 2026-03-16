{% fn add(a, b) %}
sum = 50 + a * b
print(sum)
return sum
{% endfn %}

{% fn describePerson(person) %}
{% if person.age >= 18 %}
label = "adult"
{% else %}
label = "minor"
{% endif %}

print(label)
return label
{% endfn %}
