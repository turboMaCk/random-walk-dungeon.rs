# Random Walk Dungeon Generator in Rust

This program generates random rouge-like dungeons. Based on [Random walker algorithm](https://en.wikipedia.org/wiki/Random_walker_algorithm).

**Parameters:**

1. Dimensions: the width and height of the map.
1. MaxTunnels: the greatest number of turns the algorithm can take while making the map.
1. MaxLength: the greatest length of each tunnel the algorithm will choose before making a horizontal or vertical turn.

## Example Usage

```
$ random-walk-dungeon --dimensions 20 --tunnels 50 --tunnel-length 8
#############      #
#       ##### #### #
#  #### ##### #### #
## #### #####    #
## #### #        ##
## #### # #########
## ####
## ##          ## #
##    ##       ## #
#########   ## ## #
########### ## ## #
########### #
###########       ##
###########   ##
###########       ##
############     ###
##########     #####
##########    ######
########## # #######
######       #######
```
