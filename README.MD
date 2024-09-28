# About
Matrix-Lightning-Tip-Bot (MLTB) is a lightning network BTC tip bot for the matrix network. It is inspired by the LightningTipBot Telegram project at https://github.com/LightningTipBot/LightningTipBot.

# Warning
While we have tested this bot, it is in 'beta', don't put in more than you are willing to lose.

# How to use
MLTB can either be used directly or be self run. Running your own instance requires access to a LNbits (https://lnbits.com/) instance with the https://github.com/lnbits/usermanager plugin installed.

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
2. Run `./gradlew buildDocker`  to build the docker image. On Windows this might need the `--project-cache-dir=../cache` option due to caching issues.

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
--lnbits-x-api-key=<LNBITS-X-API-KEY>    # The super user x-api-key for your LNbits instance. See https://github.com/lnbits/lnbits/wiki/LNbits-Extensions on how to obtain it.  
--database-url=/db/db.db                 # The absolute path to your generated db.
```

### Running
Run `docker run --rm  -v <path-to-config-directory>:/config/  -v <path-to-database-directory>:/db  matrix-lightning-tip-bot  matrix-lightning-tip-bot @/config/config.conf` to start the MLTB container. 

It is also possible to use the docker-compose file. This will require .env file containing the entries 
```
CONFIG_DIR=<path-to-config-directory>
DATABASE_DIR=<path-to-database-directory>
CONFIG_FILE=@/config/config.conf
```
See https://docs.docker.com/compose/environment-variables/ for more information.

# Contact 
We have public matrix room at `#matrix-lightning-tip-bot-discussion:s-software-solutions.nl`

# BTC Donations
Non lightning BTC donations are welcome at `bc1q72dzh04fwxx780w05twtmn5fxzegpawdn5zg3g`

# Thanks
A big thanks the people over at LightningTipBot. This project is in no way ment to replace or critique the great work they have been doing, merely to give more people access to the lightning network.

Also, we would like to give a thank the people of LNbits and Umbrel (https://getumbrel.com/) for their amazing free products, this bot wouldn't have existed without their work.  

# Scheduled TODOs
- Inform users about donated funds.
- Graceliously handle multiple bots is a single
