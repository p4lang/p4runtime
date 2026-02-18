---
layout: default
title: P4Runtime - Directory Listing
---

## Directory Listing

<ul>
  {% for file in site.static_files | sort: 'path' %}
    {% assign ext = file.extname | downcase %}
    {% if ext == '.pdf' or ext == '.html' %}
    <li>
      <a href="{{ site.baseurl }}{{ file.path }}">
        {{ file.path | replace_first: '/', '' }}
      </a>
    </li>
    {% endif %}
  {% endfor %}
</ul>
