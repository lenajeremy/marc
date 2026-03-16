Helloooo every{{ (7-6) }}

## what is 2 + 2?
2 + 2 is {{ 20 + 3 }}

## is 4 * 6 == 8 * 3?
4 * 6 == 8 * 3 is {{ 4 * 6 == 8 * 3 }}

{% block %}
numberOfPeople = 50
{% endblock %}

There are currently {{ numberOfPeople }} in the auditorium

30 people are about to leave.

{% block %}
numberOfPeople = numberOfPeople - 30
{% endblock %}

There are currently {{ numberOfPeople }} in the auditorium
