Draw the players as points and the rivalries as lines. Once you put one player in team A, the team of every player connected to them is forced.
---
Give each player a colour (team 0 or team 1) by walking the graph: a rival of a player gets the other colour.
---
For each player not yet coloured, start a BFS with colour 0. When you see a rival who already has the **same** colour as the current player, no split exists.
