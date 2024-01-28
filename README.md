# Bittorrent
Building a bittorrent client to learn rust

## Bencoding
- String: length prefixed base 10 : string
    - 4:test -> test
    - 7:testing -> testing
    - 3:hello -> hel
- Integers: i base10 number e
    - i3e -> 3
    - i-3e -> -3
    - i0e -> 0
    - i-0e -> invalid
- List: l bencoded elements e
    - l4:test2:hie -> ["test","hi"]
- Dictionaries: d alternating key/value e
    - d3:cow3:moo4:spam4:eggse -> {'cow': 'moo', 'spam': 'eggs'}
    - d4:spaml1:a1:bee -> {'spam': ['a','b']}
### TO DO
[Spec](https://www.bittorrent.org/beps/bep_0003.html)
- [ ] parse bencode
- [ ] parse torrent file
- [ ] request files from others
- [ ] download pieces
- [ ] put together pieces
- [ ] seeding
