table! {
    games (gid) {
        gid -> Text,
        name -> Text,
        species_id -> BigInt,
        background_id -> BigInt,
        xl -> BigInt,
        dam -> BigInt,
        sdam -> BigInt,
        tdam -> BigInt,
        tmsg -> Text,
        turn -> BigInt,
        dur -> BigInt,
        runes -> BigInt,
        score -> BigInt,
        start -> Text,
        end -> Text,
        potions_used -> BigInt,
        scrolls_used -> BigInt,
    }
}
