# Elden Ring Messages Bot

A bot that generates messages from Elden Ring and posts them to Mastodon. Expects three lists of strings:

- `erconjunctions.txt`: the strings that combine two messages into a two-line message
- `bmsg.txt`: a bunch of phrases with the placeholder `<?bmsg?>` in them
- `ermessages.txt`: a lot of strings that can be put where `<?bmsg?>` is in the above