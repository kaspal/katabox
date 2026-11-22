settings = {"theme": "dark", "size": 12}
backup = settings
current = dict(settings)
settings["size"] = 14
current["theme"] = "light"
print(backup["size"])
print(settings["theme"])
print(current["size"])
