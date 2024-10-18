 # About
Matrix-Lightning-Tip-Bot (MLTB) is a lightning network BTC tip bot for the matrix network. It is inspired by the LightningTipBot Telegram project at https://github.com/LightningTipBot/LightningTipBot.

@AE9999 has started this project so he is the main actor here and all appreciations belong to him. Just updating and adding new functions here.

# Warning
While I have tested this bot, it is in 'beta', don't put in more than you are willing to lose.

# How to use
MLTB can either be used directly or be self run. Running your own instance requires access to a LNbits (https://lnbits.com/) instance with the https://github.com/lnbits/usermanager plugin installed.

## Directly
I am are running a dedicated MLTB instance over at @lightning-wallet-bot:matrix.yourdevice.ch. Invite it to a room to get started immediately. After joining the bot should display the possible commands which are:

```
!tip     - Reply to a message to tip it: !tip <amount> [<memo>]
!balance - Check your balance: !balance
!send    - Send funds to a user: !send <amount> <@user> or <@user:domain.com> or <lightningadress@yourdomain.com> [<memo>]
!invoice - Receive over Lightning: !invoice <amount> [<memo>]
!pay     - Pay  over Lightning: !pay <invoice>
!help    - Read this help.\n\
!donate  - Donate to the matrix-lighting-tip-bot project: !donate <amount>
!party   - Start a Party: !party
!fiat-to-sats - Convert fiat to satoshis: !fiat-to-sats <amount> <currency (USD, EUR, CHF)>
!sats-to-fiat - Convert satoshis to fiat: !sats-to-fiat <amount> <currency (USD, EUR, CHF)>
!version - Print the version of this bot
```

My MLTB instance runs on this LightningNode https://amboss.space/node/034efe9ae4eecacc0a2d7cdff59852665398968cfea347185479cc049ccac5e8ec
## Running your own instance
We recommend running your own MLTB instance using Docker https://www.docker.com/.

Running your own instance is done by:
1. Install prequisites
2. Create a new user for your lnbits instance
3. Building the docker image.
4. Generating a local Database file
5. Create a new user on your matrix-server for the bot to use
6. Constructing a config file
7. Running a MLTB

### Install prequisites

Install java and libsqlite3 on your debian/ubuntu server with
`sudo apt install openjdk-17-jdk`
`sudo apt install libsqlite3-dev`

### Create a new user for your lnbits instance
Create a new user on your instance which manages the wallets. Install the user manager application plugin for that user. The user can be a normal user, doesnt need to be a admin or a superuser account

### Build Docker image
Building the Docker image is done with:
1. Install Docker (https://www.docker.com/)
2. Run `./gradlew buildDocker`  to build the docker image. On Windows this might need the `--project-cache-dir=../cache` option due to caching issues.

### Generate local sqlite Database file
Generating the local Database file is done with:
1. Install Rust (https://www.rust-lang.org/tools/install)
2. Install diesel (sqlite only) by running `cargo install diesel_cli --no-default-features --features sqlite`.
3. Run `diesel migration run --database-url=wallet-bot.db` to generate a local sqlite Database file.

### Create a new matrix user for the bot
Create a new user on your matrix server which will be used my MLTB.

### Construct a config file
Construct a file `config.conf` with the following entries:
```
--matrix-server=https://matrix.my-matrixserver.org     # Your Matrix Instance
--matrix-username=your_user_for_the_bot                # The username of the account your created
--matrix-password=the_passwort_for_that_user_account   # The password of your matrix bot
--lnbits-url=http://mylnbitsurl.com                    # The url of your LNbits instance
--lnbits-x-api-key=<LNBITS-X-API-KEY>                  # The user x-api-key for your LNbits instance. See https://github.com/lnbits/lnbits/wiki/LNbits-Extensions on how to obtain it.
--database-url=/db/db.db                               # The absolute path to your generated db.
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
The developer has public matrix room at `#matrix-lightning-tip-bot-discussion:s-software-solutions.nl`
# BTC Donations
Non lightning BTC donations are welcome at `bc1q72dzh04fwxx780w05twtmn5fxzegpawdn5zg3g`

Contributors lightning adress (me) = node-runner@btcpay.yourdevice.ch

# Thanks
A big thanks the people over at LightningTipBot and @AE9999. This fork-project is in no way ment to replace or critique the great work they have been doing, merely to give more people access to the lightning network.

# Scheduled TODOs
- make this use with a bot registered on your homeserver instead of a user-account
- limit usage to specific matrix instances
- Inform users about donated funds.
- Graceliously handle multiple bots is a single
- integrate a fiat/rate conversion tool (✅)
- send sats directly to a lightning adress (✅)
