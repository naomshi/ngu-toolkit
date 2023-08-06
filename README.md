# ngu-toolkit

NGU Toolkit is an assistant process for the game NGU Idle. It uses cross-process memory inspection to provide some useful utilities for the game.

## Features
Some of the features included with NGU Toolkit are listed below.

### Questing
Will monitor your quest progress, checking if the amount of quest items in your inventory combined with your already handed in items will allow you to complete your current quest. If this is true, a desktop notification will be displayed.

### Adventure idle
Checks for your current adventure zone, sending a desktop notification if you enter the safe zone.

### Cooking
Calculates the best combination of Cooking ingredients and prints them to the terminal (may be inaccurate currently - try increasing or decreasing ingredients slightly if the total is a bit below 100%).

## Installation
I'll make a release build once a few more features have been added, currently you can try it out by cloning the repo and doing `cargo run`

## Usage
Currently works as a CLI application with a few commands:

| Command  | Description |
| ------------- | ------------- |
| quest  | Sends desktop notification upon detected quest completion  |
| adventure | Sends desktop notification when idling in adventure  |
| cooking | Shows optimal ingredient allocation for cooking  |

Run with `./ngu-toolkit.exe <COMMAND>`

## Known issues
- Currently the cooking feature might be slightly off. If it doesnt give you 100%, try cycling individual ingredients from 1-20 (This has worked for me every time so far)