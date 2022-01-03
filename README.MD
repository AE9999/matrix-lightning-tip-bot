# About
Matrix-Lightning-Tip-Bot (MLTB) is a lightning network BTC tip bot for the matrix network. It is inspired by the LightningTipBot Telegram project at https://github.com/LightningTipBot/LightningTipBot.

# How to use 

MLTB can either be used directly or be self run. Running your own instance requires access to a LNbits (https://lnbits.com/) instance.

## Directly

We are running a dedicated MLTB instance over at @matrix-lightning-tip-bot:matrix.org. Invite it to a room to get started immediately. After joining the bot should display the possible commands which are:

```
!tip      - Reply to a message to tip it: !tip <amount> [<memo>]
!balance - Check your balance: !balance
!send    - Send funds to a user: !send <amount> <@user> or <@user:domain.com> [<memo>]
!invoice - Receive over Lightning: !invoice <amount> [<memo>]
!pay     - Pay  over Lightning: !pay <invoice>
!help    - Read this help.\n\
!donate  - Donate to the matrix-lighting-tip-bot project: !donate <amount>
!party   - Start a Party: !party
!version - Print the version of this bot
```

Our MLTB instance runs on this LightningNode https://amboss.space/node/031017e6563c2ba50cf43300b8ac67e6fe577646df76e1216368dce75ac8c3457d

## Running your own instance

We recommend running your own MLTB instance using Docker https://www.docker.com/. 

Running your own instance is done by:
1. Building the docker image.
2. Generating a local Database file
3. Constructing a config file
4. Running a MLTB

### Build Docker image
Building the Docker image is done with:
1. Install Docker (https://www.docker.com/)
2. Run `./gradlew build`  to build the docker image. On Windows this might need the `--project-cache-dir=../cache` option due to caching issues.
   

### Generate local sqlite Database file
Generating the local Database file is done with:
1. Install Rust (https://www.rust-lang.org/tools/install)
2. Install diesel (sqlite only) by running `cargo install diesel_cli --no-default-features --features sqlite`. (On ubuntu required installing the 'libsqlite3-dev' package) 
3. Run `diesel migration run --database-url=db.db` to generate a local sqlite Database file.

### Construct a config file
Construct a file `config.conf` with the following entries:
```
--matrix-server=@example_name:matrix.org # The registred username of your bot
--matrix-password=SomeExamplePassowrd    # The password of your matrxi
--lnbits-url=http://mylnbitsurl.com      # The url of your LNbits instance  
--lnbits-x-api-key=<LNBITS-X-API-KEY>    # The x-api-key for your LNbits instance. See i.e. https://golangrepo.com/repo/LightningTipBot-LightningTipBot for details on how to do that.  
--database-url=/db/db.db                 # The absolute path to your generated db.
```

### Running
Run `docker run --rm  -v <path-to-config-directory>:/config/  -v <path-to-database-directory>:/db  matrix-lightning-tip-bot  matrix-lightning-tip-bot @/config/config.conf` to start the MLTB container

# Contact 
We have public matrix room at `#matrix-lightning-tip-bot-discussion:s-software-solutions.nl`  

# Warning
This software is in 'beta', don't put in more than you are willing to lose. For now this bot assumes it's the only one of its kind in chat. Multiple versions shouldn't matter as it won't spend money you don't have, but it can be annoying.

# BTC Donations
Non lightning BTC donations are welcome at `bc1q72dzh04fwxx780w05twtmn5fxzegpawdn5zg3g`

# Thanks
A big thanks the people over at LightningTipBot. This project is in no way ment to replace or critique the great work they have been doing, merely to give more people access to the lightning network.

Also, we would like to give a thank the people of LNbits and Umbrel (https://getumbrel.com/) for their amazing free products, this bot wouldn't have existed without their work.  
