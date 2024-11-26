# lichess-id-divider
### A fast Rust script for moving Lichess Game IDs from the Lichess Database to individual, easily queryable binary files

The Rust project is located in [`lichess-id-divider`](https://github.com/RandomGamingDev/lichess-id-divider/tree/main/lichess-id-divider) and reads `lichess_db_standard_rated_2024-10.pgn` (although you can change it in the script),
spitting it out in the [`out-lichess_db_standard_rated_2024-10`](https://github.com/RandomGamingDev/lichess-id-divider/tree/main/out-lichess_db_standard_rated_2024-10) directory in 6kb files each containing 1000 game IDs in binary (converted from base64 to binary and can be decoded with all major programming langauges).
This is done so that it's easier for programs including web applications to query individual files like the elo guessing game [`ChessELOle`](https://github.com/RandomGamingDev/ChessELOle), since no chunking to avoid flooding memory is required (the original `lichess_db_standard_rated_2024-10.pgn` file is 219.5 GB)
and so that it can be stored on Github (Github only accepts individual files of a maximum size of 2GB).

If you want to use the script yourself, or are interested in getting chess games from [Lichess](https://lichess.org/) for your own project, be it hobby, commercial, or research, you can find the archives here: https://database.lichess.org/ (BEWARE: The files are MASSIVE and even more so when unzipped)
