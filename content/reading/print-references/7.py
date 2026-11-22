team = {"name": "A", "members": ["ana", "luis"]}
clone = dict(team)
clone["name"] = "B"
clone["members"].append("eva")
print(team["name"])
print(len(team["members"]))
