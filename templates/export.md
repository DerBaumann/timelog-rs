# Arbeitsjournal
{% for (date, entries) in entries %}
## {{ date.format("%d.%m.%y") }}

| Projekt | Beschreibung | Zeit |
| ------- | ------------ | ---- |
{% for entry in entries %}| {{ entry.project }} | {{ entry.description }} | {{ entry.start_time.format("%H:%M") }} - {{ entry.end_time.format("%H:%M") }} |
{% endfor %}{% endfor %}
