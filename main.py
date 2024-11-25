import base64

import chess.pgn

pgns = open("lichess_db_standard_rated_2024-10.pgn")

c = True
i = 0
while c:
	with open(f"out/lichess-{i}.ids", "wb") as f:
		for _ in range(10 ** 3):
			pgn = str(chess.pgn.read_game(pgns))
			if pgn == None:
				c = False
				break

			id_head = "[Site \"https://lichess.org/"
			id_start = pgn.find(id_head) + len(id_head)
			id_end = pgn.find('"', id_start)
			id = pgn[id_start:id_end] # url-safe (no '/' or '+') base64 id
			f.write(bytearray(base64.b64decode(id))) 
	i += 1