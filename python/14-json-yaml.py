import json
import io

my_json = """
{
    "name": "Jason",
    "age": 44,
    "cats": ["Ruby", "Dora", "Mitzi", "Tiny"]
}
"""

val = json.load(io.StringIO(my_json))

with open("tmp.json", "w") as f:
    json.dump(val, f, indent=2)

# skipping yaml and the anyconfig stuff
# will pick it up if/when i need it...