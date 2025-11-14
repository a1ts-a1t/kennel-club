## why should you join

it's fun! and ostensibly you would have more people linking to your site. and also you get to have a little creature running around (see "it's fun")!

## how to join

If you wanna add your website to this webring...

1. clone this repo with

```sh
git clone https://github.com/a1ts-a1t/kennel-club.git
```

2. add a new folder with some id for your website in the `data` folder
3. in that folder, add the sprites for your creature
    * you can add as many or as few as you would like. there's nothing stopping you from just using the same sprite for everything
4. in `data/metadata.json`, add a new entry into the json. the entry should follow [this schema](./metadata_schema.json)
    * `id`: the name of the folder you just created
    * `display_name`: the display name of your creature
    * `url`: the link to your website
    * `step_size`: how far your creature walks every frame. between 0 (not at all) and 1 (the length of the entire kennel)
    * `radius`: how large your creature is. must be between 0 and 0.5. the larger it is, the less likely there will be room for any other creature though
    * `sprites`: the file names of the sprite you want to use for each state your creature is in. if you add more sprites per state, then they will play one after another if the creature stays in that state. each state must have at least one sprite. you can reuse sprites see [this metadata entry](https://github.com/a1ts-a1t/kennel-club/blob/19a4751/data/metadata.json#L9) for example. the states you must account for are
        * `idle`
        * `sleep`
        * `east`
        * `northeast`
        * `north`
        * `northwest`
        * `west`
        * `southwest`
        * `south`
        * `southeast`
5. create [a pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests) with your changes. if you're unfamiliar, run the commands below and follow the instructions in the link created in the last command

```sh
git add .
git checkout -b <your creature id>
git commit -m "adding <your creature id>"
git push -u origin <your creature id>
```

6. let the automated tests run on the pull requests. if they fail, you may have to go back and if something. i'll leave a comment in the PR
7. when the tests pass, i'll merge it in!

## adding the webring to your website

in the spirit of the webring, you have to link to somebody's page from the webring on your website. i'm leaving it up to you and your website for how you wanna do it, but here are some options

* link to somebody's webpage (or a bunch of webpages) from your website. the list of url's is in [the metadata file](./data/metadata.json)
* link to [`https://alts-alt.online/api/kennel-club/random/site`](https://alts-alt.online/api/kennel-club/random/site). this link takes you to a random site of someone in the webring
* link to [`https://alts-alt.online/projects/kennel-club`](https://alts-alt.online/projects/kennel-club). this is where a main display of all the creatures running around is
* embed the image `https://alts-alt.online/api/kennel-club/img`. this is a static image of the webring's current state
* embed the image `https://alts-alt.online/api/kennel-club/<id>/img`. this is a static image of your creature's current state
* call the api at `https://alts-alt.online/api/kennel-club`. this is a json representation of the current webring state
* subscribe to the current webring state at `wss://alts-alt.online/ws/kennel-club` and do whatever you want with it

