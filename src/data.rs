use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
#[repr(i64)]
pub enum Species {
    Barachi = 0,
    Centaur,
    DeepDwarf,
    DeepElf,
    Demigod,
    Demonspawn,
    Draconian,
    RedDraconian,
    WhiteDraconian,
    GreenDraconian,
    YellowDraconian,
    GreyDraconian,
    BlackDraconian,
    PurpleDraconian,
    MottledDraconian,
    PaleDraconian,
    Felid,
    Formicid,
    Gargoyle,
    Ghoul,
    Gnoll,
    Halfling,
    HighElf,
    HillOrc,
    Human,
    Kobold,
    Merfolk,
    Minotaur,
    Mummy,
    Naga,
    Ocotopode,
    Ogre,
    Spriggan,
    Tengu,
    Troll,
    Vampire,
    VineStalker,
}

impl FromStr for Species {
    type Err = ();
    fn from_str(string: &str) -> Result<Species, ()> {
        // TODO: we pay the price of to_lowercase() every call in parsing even though we could match on Title Case
        // the lowercase is only really needed on the serve (web page) for parsing URL values
        // we could create two specialized versions of from_str for each, but that would mean we can't use parse which is nice
        // tricky. perhaps we shouldn't be sharing the enums/impls after all? but they shouldn't go out of sync. ugh.
        let s = string.to_lowercase();
        if s.ends_with("draconian") {
            return Ok(Species::Draconian);
        }
        match s.as_str() {
            "barachi" => Ok(Species::Barachi),
            "centaur" => Ok(Species::Centaur),
            "deep dwarf" => Ok(Species::DeepDwarf),
            "deep elf" => Ok(Species::DeepElf),
            "demigod" => Ok(Species::Demigod),
            "demonspawn" => Ok(Species::Demonspawn),
            "draconian" => Ok(Species::Draconian),
            // These are impossible to hit for now,
            // eventually sub races will be a possibility? TOOD
            "red draconian" => Ok(Species::RedDraconian),
            "white draconian" => Ok(Species::WhiteDraconian),
            "green draconian" => Ok(Species::GreenDraconian),
            "yellow draconian" => Ok(Species::YellowDraconian),
            "grey draconian" => Ok(Species::GreyDraconian),
            "black draconian" => Ok(Species::BlackDraconian),
            "purple draconian" => Ok(Species::PurpleDraconian),
            "mottled draconian" => Ok(Species::MottledDraconian),
            "pale draconian" => Ok(Species::PaleDraconian),
            // -----------
            "felid" => Ok(Species::Felid),
            "formicid" => Ok(Species::Formicid),
            "gargoyle" => Ok(Species::Gargoyle),
            "ghoul" => Ok(Species::Ghoul),
            "gnoll" => Ok(Species::Gnoll),
            "halfling" => Ok(Species::Halfling),
            "high elf" => Ok(Species::HighElf),
            "hill orc" => Ok(Species::HillOrc),
            "human" => Ok(Species::Human),
            "kobold" => Ok(Species::Kobold),
            "merfolk" => Ok(Species::Merfolk),
            "minotaur" => Ok(Species::Minotaur),
            "mummy" => Ok(Species::Mummy),
            "naga" => Ok(Species::Naga),
            "octopode" => Ok(Species::Ocotopode),
            "ogre" => Ok(Species::Ogre),
            "spriggan" => Ok(Species::Spriggan),
            "tengu" => Ok(Species::Tengu),
            "troll" => Ok(Species::Troll),
            "vampire" => Ok(Species::Vampire),
            "vine Stalker" => Ok(Species::VineStalker),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i64)]
pub enum Background {
    Fighter = 0,
    Gladiator,
    Monk,
    Hunter,
    Assassin,
    Berserker,
    AbyssalKnight,
    ChaosKnight,
    Skald,
    Enchanter,
    Transmuter,
    ArcaneMarksman,
    Warper,
    Wizard,
    Conjurer,
    Summoner,
    Necromancer,
    FireElementalist,
    IceElementalist,
    AirElementalist,
    EarthElementalist,
    VenomMage,
    Artificer,
    Wanderer,
}

impl FromStr for Background {
    type Err = ();
    fn from_str(s: &str) -> Result<Background, ()> {
        // TODO: we pay the price of to_lowercase() every call in parsing even though we could match on Title Case
        // the lowercase is only really needed on the serve (web page) for parsing URL values
        // we could create two specialized versions of from_str for each, but that would mean we can't use parse which is nice
        // tricky. perhaps we shouldn't be sharing the enums/impls after all? but they shouldn't go out of sync. ugh.
        match s.to_lowercase().as_str() {
            "fighter" => Ok(Background::Fighter),
            "gladiator" => Ok(Background::Gladiator),
            "monk" => Ok(Background::Monk),
            "hunter" => Ok(Background::Hunter),
            "assassin" => Ok(Background::Assassin),
            "berserker" => Ok(Background::Berserker),
            "abyssal knight" => Ok(Background::AbyssalKnight),
            "chaos knight" => Ok(Background::ChaosKnight),
            "skald" => Ok(Background::Skald),
            "enchanter" => Ok(Background::Enchanter),
            "transmuter" => Ok(Background::Transmuter),
            "arcane marksman" => Ok(Background::ArcaneMarksman),
            "warper" => Ok(Background::Warper),
            "wizard" => Ok(Background::Wizard),
            "conjurer" => Ok(Background::Conjurer),
            "summoner" => Ok(Background::Summoner),
            "necromancer" => Ok(Background::Necromancer),
            "fire elementalist" => Ok(Background::FireElementalist),
            "ice elementalist" => Ok(Background::IceElementalist),
            "air elementalist" => Ok(Background::AirElementalist),
            "earth elementalist" => Ok(Background::EarthElementalist),
            "venom mage" => Ok(Background::VenomMage),
            "artificer" => Ok(Background::Artificer),
            "wanderer" => Ok(Background::Wanderer),
            _ => Err(()),
        }
    }
}
