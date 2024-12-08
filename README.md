# Getting started with spaces game

Check it out at https://randallard.github.io/spaces-game/

## Development

You'll need to install Tailwind CSS locally for the project. 

1. First, initialize npm for the project:
```powershell
npm init -y
```

2. Then install Tailwind CSS as a dev dependency:
```powershell
npm install -D tailwindcss
```

3. Initialize Tailwind CSS:
```powershell
npx tailwindcss init
```

After these steps, try `trunk serve` again. The build should work now since Tailwind CSS will be available locally in the project.

Let me know what output you get and we can continue troubleshooting if needed!

## Description

We'll be using the newest version of leptos with a postgres database, but initially we'll have sqlite database.


Users first visit will allow them to create an account or connect with their github account.


After creating an account or logging in, they'll see a link they can send to friends in order to connect and play.  Also on the home screen the player should see links to create boards or start an AI game (not really AI, just a system player that plays random boards).


Friends they've played with will appear in a column on the left, along with system generated players that just play randomly.


Before selecting a player you have the opportunity to create a strategy - not sure that's the best word for it, do suggest a better one - what I'm thinking is that there's a grid that's 2 X 2 - we'll add bigger boards as well - but we'll start off with 2 X 2 - in this phase the player will choose a start square at the bottom of the board, their "piece" starts there and they can either place a trap in any adjascent square - so on a two by two either the square in front (above) or beside their piece - or they can move their piece to any adjascent square, either in front or beside.  The goal is to create a sequence of turns and get their piece off the other side of the board, and stop their opponent with a trap.  In play, their opponents board will be set up in the opposite direction and scored.


During board creation a graphic of the board will appear and a key will be along side.  A circle signifies the player piece and an x is a trap.  For the first turn, the player will see buttons to choose circle in any of the bottom, squares of the grid - after choosing one of those, any adjascent square that doesn't already have a trap or circle in it will have buttons to choose either circle or x.  When there is a circle in the last (top) row, there will be one button labeled "Finish" Signifying the last move reaching the goal.


Points are added up for each sequential turn - players get points for a move towards their goal that doesn't land on a trap or the other player.  No more turns are played after a player lands in a square containing either a trap or the other player.  


The player must create at least one board before connecting for a game - during a game you cannot create boards, only choose from boards you have already made.  So starting a game with no boards sends you to the board creation phase before connecting with the chosen player.


When both players are connected, they are presented with a list of board thumbnails, they have three seconds to choose a board or the opponent receives all points for that round.  After both players have selected a board, the boards are displayed, opponent's board overlaid but only so far as the player hasn't hit a trap or the other player, only valid turns are displayed for each player.  Player totals for that round are displayed for 2 seconds, then added to the totals displayed off to the side, then players are prompted to choose a board with a timer counting down 3 seconds.  After three seconds, results are displayed again.


1 game consists of 8 rounds.  After which the total points are displayed along with the option to play again or "Naur".  Play again when clicked by one player says "waiting for other player" and displays "[username] wants to play again!" on the other player's screen.  New game starts when both players click play again.  If a player click Naur the other player receives a message that says the other player declined to play again with a button to go back to the main screen.  