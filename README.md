tr — translate or delete characters

Process characters from stdin, translating, deleting, or squeezing characters
and writing the result to stdout.

This `tr` interprets Unicode graphemes as individual characters:

    $ echo As Qh | target/release/tr 'shdc' '♠♡♢♣'
    A♠ Q♡
    $ echo As Qh | tr 'shdc' '♠♡♢♣'
    A Q
