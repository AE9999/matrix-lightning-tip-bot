# About
This is a lightning tip bot for the matrix network, inspired by the LightningTipBot Telegram project at https://github.com/LightningTipBot/LightningTipBot.

# How to Use
TODO

# Who are we
https://amboss.space/node/031017e6563c2ba50cf43300b8ac67e6fe577646df76e1216368dce75ac8c3457d

# Warning
This software is definitely in 'beta', don't put in more than you are willing to lose! Also for now this bot assumes it's the only one of it's kind in chat. multiple versions shouldn't matter as it won't spend money you don't have, but it can be annoying.

# Running your own instance

## construct a config

## build project
`docker build -t lnbits-matrix-bot .`

## Generate local db
run `diesel migration run` to generate a local db.

## Running
`docker run
        --rm
        -v <path-to-config-directory>:/config/
        -v <path-to-database-directory>:/db
        lnbits-matrix-bot
        lnbits-matrix-bot @/config/config.conf`

# BTC Donations

Non lightning BTC donations are welcome at `bc1q72dzh04fwxx780w05twtmn5fxzegpawdn5zg3g`

# Thanks
Many thanks to the people over at LightningTipBot and
