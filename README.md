# Random Walk Dungeon Generator in Rust

This program generates random rouge-like dungeons. Based on [Random walker algorithm](https://en.wikipedia.org/wiki/Random_walker_algorithm).

**Parameters:**

1. Dimensions: the width and height of the map.
1. MaxTunnels: the greatest number of turns the algorithm can take while making the map.
1. MaxLength: the greatest length of each tunnel the algorithm will choose before making a horizontal or vertical turn.
1. Seed: 32 bytes hex string used to seed random generator

## Example Usage

```
$ random-walk-dungeon --dimensions 80 --tunnels 1000 --tunnel-length 50 --seed DD85025F53B1A9CE10496324476981BD8DAA37AAF0B74433D314C69F82581D95
seed: DD85025F53B1A9CE10496324476981BD8DAA37AAF0B74433D314C69F82581D95
########################        ##############    #####                ######
######################## # #### ############## #        ####  ##       ####
##################          ### ##############      ### ###   ##   #   ##
##################       #  ### ###     ########  # ### ###   ##       ## #
################## ## ##    ### ### ### ########        ###               # #
################## ## ## #      ###        #####
################## ## ## ############## ## #######   ## ###
################## ## ## ############## ## ###                #           #
###########  #####    ## ##############        ###    ### ##
######        ####       #################            ##
###### ####   ######        ################           #  #  #
######        ######## #### ################  # #   ## #  #            #
############ #######        ##########            # ## #  #            #
########                  ############ ##        #   # #                      #
######## ##########    ## ############ ## #   #                            #  #
######## ##########    ## ############ ## #   ## # # # ####  #   #
######## ##########   ### ############ ##     ##       ####               #
########  #########     #       ######             #   ####  #              ###
######### ########## ## ####### #########          # ######               #####
######### ########## ## ####### #########          #   #        ##   ###  #####
######### ########      ####### #########   # ##   ### # ##               #####
######### #        # ########## ####        # ######## # #####        ###
######### # ##       ########## #### #### # # ######## # #####      # ##### ###
######### # ## ################ #### ####     ######## # ####      ##     # ###
#########   ## ############          ######   ########   #### # #  ## ### # ###
############## ############ #### ######       ########             ## ### #
############## ############ #### ###### ####      ####        #       ### #####
##############  ########### #### ###### ####   ## ##       ##### ######## #####
############### #########   #### ###### ####   ## ## # ### ##### ########
############### #########   #### ###### ####    # ## # ###       ##############
###############       ###        ###    ###              #       ##############
##################### ### #       ## ######  ##  ##        ##### #########
##################### ### ####### ## ######                   ## ######### #####
##################### ### ####### ## ###### # ##  #           ## ######### #
##################### ###    #### ## ###### # ##     #  #  ## ## ######### #
################   ##        #### ## ###### # ### #  #                     ## #
############     # ############## ## ###      ###         # #  ## #### ######
############ #####  ##########       ### # ## ###           #  ## #### ######
#        ### ###### ##########                ### ##### #####     ###
#        ### ###### ########## ##       ##    ### ##### ###### ## ###
##### ## ### ###### ###    ###          ### # ###       ###### ##      ##
#####    ### ###### ### ## ######  # ## ###             ######       ####
#######  ### ###### ### ## #       # ## ### # # ##### ########### ## ###
#######  ###               # ####       ###   # ##### ########       ###
#######   ####### #  ## #  #            ##                ##   ## ######
####### # ####### #        #  ##  ####      ###  #  # #   ## # ## #
####### # #######    #     #  ##            ###  #  # #        ## # ####
####### # ########               #####  ##  ###                ## # ####
#######     ######  ## #       #                   #### #
#########     ###   ##       # #                   #### #         #    #
######### # # ###         ##   ###### #           ##### #      ####       #
#######            ########      ###               #### #  ##  ####    #
#######       # #  ########      #                 ##   #  ##             #    #
########      # #  ########                  ##    ## ###  ##        #       # #
########  #            #####  #                    ## #######              # # #
########     ## #      #####  ###  #             #### ########                 #
#####        ## ##  ########  #      # ##  # ######## ########             #####
##### ## ###### ##  #######        ### ##  #     #### ######## ###  # #    #####
##### ## ######      ######     #  ### ##  ##### #### ######## ###  # #    #####
####     ###### #  # #####             ##  #####      ######## ###         #####
####  ######### #  # #####  #   ###### ## #################### ### ## #    #####
####       #### #    #####  ### ###    ## #################### ### ## #    #####
#####    # #### # ## #####  ###   # #     ####################     ##     ######
######## # #### # ## #####        # # ##########################################
######## # ###  # ## #####  ###     # ##########################################
######## # ###    ##                # ##########################################
########   ###    ##      ## ### #  # ##########################################
########## ###    ####### ##        # ##########################################
##########        ####       ##          #######################################
########## ##        # ## ## ## ##   ### #######################################
########## ## ##  #       ## ##      ### #######################################
########## ## ##  #    ##### ##  ##  ### #######################################
########## ##     #   ###### ##  ##  ### #######################################
##########            ###### ##  ##  ### #######################################
############## ####   ###     #  ##  ### #######################################
############## ####   ###  ## #          #######################################
############## ####   ###  ## ## ###############################################
############## ####           ## ###############################################
############## #####             ###############################################
##############        ######  ##################################################
```
