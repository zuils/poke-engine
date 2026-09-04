use super::items::Items;
use crate::choices::{Boost, Choice, Choices, Heal, MoveCategory, MoveTarget, StatBoosts};
use crate::pokemon::PokemonName;
use crate::state::{Pokemon, PokemonType};

#[derive(Debug, Clone, PartialEq)]
pub struct ZMove {
    pub name: &'static str,
    pub move_type: PokemonType,
    pub category: MoveCategory,
    pub base_power: f32,
    pub status: bool,
    pub status_effect: Option<ZStatusEffect>,
    pub fixed_damage_fraction: Option<f32>,
    pub terrain_effect: Option<TerrainEffect>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerrainEffect {
    SetPsychic,
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ZStatusEffect {
    Boost(StatBoosts),
    Heal,
    ClearNegativeBoosts,
    CritRatio,
}

fn boosts(
    attack: i8,
    defense: i8,
    special_attack: i8,
    special_defense: i8,
    speed: i8,
    accuracy: i8,
) -> StatBoosts {
    StatBoosts {
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        accuracy,
    }
}

pub fn get_z_status_effect(move_id: Choices) -> Option<ZStatusEffect> {
    let effect = match move_id {
        Choices::ACIDARMOR
        | Choices::AGILITY
        | Choices::AMNESIA
        | Choices::ATTRACT
        | Choices::AUTOTOMIZE
        | Choices::BARRIER
        | Choices::BATONPASS
        | Choices::CALMMIND
        | Choices::COIL
        | Choices::COTTONGUARD
        | Choices::COTTONSPORE
        | Choices::DARKVOID
        | Choices::DISABLE
        | Choices::DOUBLETEAM
        | Choices::DRAGONDANCE
        | Choices::ENDURE
        | Choices::FOLLOWME
        | Choices::IRONDEFENSE
        | Choices::KINGSSHIELD
        | Choices::LEECHSEED
        | Choices::NASTYPLOT
        | Choices::PERISHSONG
        | Choices::PROTECT
        | Choices::QUIVERDANCE
        | Choices::RECOVER
        | Choices::ROOST
        | Choices::SHELLSMASH
        | Choices::SLACKOFF
        | Choices::SOFTBOILED
        | Choices::SPORE
        | Choices::SUBSTITUTE
        | Choices::SWORDSDANCE
        | Choices::TAILGLOW => ZStatusEffect::ClearNegativeBoosts,
        Choices::BELLYDRUM
        | Choices::HAPPYHOUR
        | Choices::AROMATHERAPY
        | Choices::CONVERSION2
        | Choices::HEALORDER
        | Choices::HEALPULSE
        | Choices::HAZE
        | Choices::HEALBELL
        | Choices::INGRAIN
        | Choices::MILKDRINK
        | Choices::MOONLIGHT
        | Choices::MORNINGSUN
        | Choices::PAINSPLIT
        | Choices::PSYCHUP
        | Choices::REFRESH
        | Choices::STOCKPILE
        | Choices::TELEPORT
        | Choices::TRANSFORM => ZStatusEffect::Heal,
        Choices::BESTOW | Choices::HONECLAWS | Choices::MEDITATE | Choices::SHARPEN => {
            ZStatusEffect::Boost(boosts(0, 0, 1, 1, 0, 0))
        }
        Choices::SPLASH => ZStatusEffect::Boost(boosts(3, 0, 0, 0, 0, 0)),
        Choices::AROMATICMIST
        | Choices::CAPTIVATE
        | Choices::IMPRISON
        | Choices::MAGICCOAT
        | Choices::POWDER
        | Choices::AURORAVEIL
        | Choices::MAGICROOM
        | Choices::MAGNETICFLUX
        | Choices::MAGNETRISE => ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)),
        Choices::HEALBLOCK | Choices::PSYCHOSHIFT => ZStatusEffect::Boost(boosts(0, 0, 2, 0, 0, 0)),
        Choices::CELEBRATE
        | Choices::CONVERSION
        | Choices::GEOMANCY
        | Choices::HOLDHANDS
        | Choices::PURIFY
        | Choices::SKETCH
        | Choices::TRICKORTREAT => ZStatusEffect::Boost(boosts(1, 1, 1, 1, 1, 0)),
        Choices::COSMICPOWER => ZStatusEffect::Boost(boosts(0, 1, 0, 1, 0, 0)),
        Choices::ELECTRICTERRAIN
        | Choices::FLOWERSHIELD
        | Choices::GEARUP
        | Choices::GROWTH
        | Choices::INSTRUCT
        | Choices::IONDELUGE
        | Choices::MIRACLEEYE
        | Choices::PLAYNICE
        | Choices::PSYCHICTERRAIN
        | Choices::SOAK
        | Choices::TELEKINESIS
        | Choices::CURSE => ZStatusEffect::Boost(boosts(0, 0, 1, 0, 0, 0)),
        Choices::AQUARING
        | Choices::BANEFULBUNKER
        | Choices::BLOCK
        | Choices::CHARM
        | Choices::DEFENDORDER
        | Choices::FEATHERDANCE
        | Choices::GRASSYTERRAIN
        | Choices::HARDEN
        | Choices::MATBLOCK
        | Choices::POISONGAS
        | Choices::POISONPOWDER
        | Choices::QUICKGUARD
        | Choices::REFLECT
        | Choices::SPIDERWEB
        | Choices::SPIKES
        | Choices::SPIKYSHIELD
        | Choices::STEALTHROCK
        | Choices::TOXIC
        | Choices::TOXICSPIKES
        | Choices::WIDEGUARD
        | Choices::WITHDRAW => ZStatusEffect::Boost(boosts(0, 1, 0, 0, 0, 0)),
        Choices::ACUPRESSURE | Choices::HEARTSWAP | Choices::SLEEPTALK | Choices::TAILWIND => {
            ZStatusEffect::CritRatio
        }
        Choices::FOCUSENERGY
        | Choices::COPYCAT
        | Choices::DEFENSECURL
        | Choices::SWEETSCENT
        | Choices::FORESIGHT => ZStatusEffect::Boost(boosts(0, 0, 0, 0, 0, 1)),
        Choices::ENCORE
        | Choices::GRASSWHISTLE
        | Choices::LOCKON
        | Choices::LOVELYKISS
        | Choices::RAINDANCE
        | Choices::RECYCLE
        | Choices::SAFEGUARD
        | Choices::SCARYFACE
        | Choices::SING
        | Choices::SUNNYDAY
        | Choices::SUPERSONIC
        | Choices::TOXICTHREAD
        | Choices::WORRYSEED
        | Choices::YAWN => ZStatusEffect::Boost(boosts(0, 0, 0, 0, 1, 0)),
        // Sp. Def +2
        Choices::MEANLOOK
        | Choices::MUDSPORT
        | Choices::NIGHTMARE
        | Choices::FORESTSCURSE
        | Choices::LUCKYCHANT
        | Choices::NATUREPOWER => ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)),
        // Def +2
        Choices::POWERTRICK | Choices::GUARDSPLIT | Choices::GUARDSWAP => {
            ZStatusEffect::Boost(boosts(0, 2, 0, 0, 0, 0))
        }
        // Sp. Atk +2
        Choices::POWERSPLIT | Choices::POWERSWAP => ZStatusEffect::Boost(boosts(0, 0, 2, 0, 0, 0)),
        // Atk +2
        Choices::MIMIC => ZStatusEffect::Boost(boosts(2, 0, 0, 0, 0, 0)),
        Choices::CHARGE
        | Choices::CONFIDE
        | Choices::CRAFTYSHIELD
        | Choices::EERIEIMPULSE
        | Choices::FLATTER
        | Choices::GLARE
        | Choices::LIGHTSCREEN
        | Choices::MISTYTERRAIN
        | Choices::SPOTLIGHT
        | Choices::THUNDERWAVE
        | Choices::WATERSPORT
        | Choices::WHIRLWIND => ZStatusEffect::Boost(boosts(0, 0, 0, 1, 0, 0)),
        // Accuracy +2
        Choices::MINDREADER | Choices::ODORSLEUTH => ZStatusEffect::Boost(boosts(0, 0, 0, 0, 0, 2)),
        // Atk +1 and Sp. Atk +1
        Choices::NOBLEROAR => ZStatusEffect::Boost(boosts(1, 0, 1, 0, 0, 0)),
        // Speed +2
        Choices::ENTRAINMENT
        | Choices::MEFIRST
        | Choices::MIRRORMOVE
        | Choices::RAPIDSPIN
        | Choices::ROCKPOLISH => ZStatusEffect::Boost(boosts(0, 0, 0, 0, 2, 0)),
        _ => return None,
    };
    Some(effect)
}

pub fn apply_z_status_effect(choice: &mut Choice, effect: &ZStatusEffect) {
    match effect {
        ZStatusEffect::Boost(z_boosts) => {
            let boost = choice.boost.get_or_insert_with(|| Boost {
                target: MoveTarget::User,
                boosts: StatBoosts::default(),
            });
            boost.boosts.attack += z_boosts.attack;
            boost.boosts.defense += z_boosts.defense;
            boost.boosts.special_attack += z_boosts.special_attack;
            boost.boosts.special_defense += z_boosts.special_defense;
            boost.boosts.speed += z_boosts.speed;
            boost.boosts.accuracy += z_boosts.accuracy;
        }
        ZStatusEffect::Heal => {
            choice.heal = Some(Heal {
                target: MoveTarget::User,
                amount: 1.0,
            });
        }
        ZStatusEffect::CritRatio => choice.z_crit_ratio += 1,
        ZStatusEffect::ClearNegativeBoosts => {}
    }
}

pub fn get_z_move_base_power(base_power: f32) -> f32 {
    match base_power as u16 {
        0..=55 => 100.0,
        56..=65 => 120.0,
        66..=75 => 140.0,
        76..=85 => 160.0,
        86..=95 => 175.0,
        96..=105 => 180.0,
        106..=115 => 185.0,
        116..=125 => 190.0,
        126..=135 => 195.0,
        _ => 200.0,
    }
}

fn override_z_move_base_power(move_id: Choices, base_power: f32) -> f32 {
    match move_id {
        Choices::VCREATE => 220.0,
        Choices::WRINGOUT | Choices::CRUSHGRIP => 190.0,
        Choices::LANDSWRATH => 185.0,
        Choices::THOUSANDARROWS
        | Choices::SHEERCOLD
        | Choices::HORNDRILL
        | Choices::GUILLOTINE
        | Choices::GEARGRIND
        | Choices::FISSURE
        | Choices::FINALGAMBIT => 180.0,
        Choices::FLYINGPRESS => 170.0,
        Choices::WEATHERBALL
        | Choices::TRUMPCARD
        | Choices::STOREDPOWER
        | Choices::REVERSAL
        | Choices::RETURN
        | Choices::PUNISHMENT
        | Choices::POWERTRIP
        | Choices::NATURALGIFT
        | Choices::LOWKICK
        | Choices::HEX
        | Choices::HEAVYSLAM
        | Choices::HEATCRASH
        | Choices::GYROBALL
        | Choices::GRASSKNOT
        | Choices::FRUSTRATION
        | Choices::FLAIL
        | Choices::ENDEAVOR
        | Choices::ELECTROBALL => 160.0,
        Choices::TAILSLAP
        | Choices::ROCKBLAST
        | Choices::PINMISSILE
        | Choices::MISTBALL
        | Choices::MAGNITUDE
        | Choices::LUSTERPURGE
        | Choices::ICICLESPEAR
        | Choices::DOUBLEHIT
        | Choices::COREENFORCER
        | Choices::BULLETSEED
        | Choices::BONERUSH => 140.0,
        Choices::TRIPLEKICK | Choices::MEGADRAIN => 120.0,
        _ => base_power,
    }
}

fn crystal_type(item: Items) -> Option<PokemonType> {
    Some(match item {
        Items::NORMALIUMZ => PokemonType::NORMAL,
        Items::FIGHTINIUMZ => PokemonType::FIGHTING,
        Items::FLYINIUMZ => PokemonType::FLYING,
        Items::POISONIUMZ => PokemonType::POISON,
        Items::GROUNDIUMZ => PokemonType::GROUND,
        Items::ROCKIUMZ => PokemonType::ROCK,
        Items::BUGINIUMZ => PokemonType::BUG,
        Items::GHOSTIUMZ => PokemonType::GHOST,
        Items::STEELIUMZ => PokemonType::STEEL,
        Items::FIRIUMZ => PokemonType::FIRE,
        Items::WATERIUMZ => PokemonType::WATER,
        Items::GRASSIUMZ => PokemonType::GRASS,
        Items::ELECTRIUMZ => PokemonType::ELECTRIC,
        Items::PSYCHIUMZ => PokemonType::PSYCHIC,
        Items::ICIUMZ => PokemonType::ICE,
        Items::DRAGONIUMZ => PokemonType::DRAGON,
        Items::DARKINIUMZ => PokemonType::DARK,
        Items::FAIRIUMZ => PokemonType::FAIRY,
        Items::PIKANIUMZ | Items::PIKASHUNIUMZ | Items::ALORAICHIUMZ => PokemonType::ELECTRIC,
        Items::DECIDIUMZ => PokemonType::GHOST,
        Items::INCINIUMZ => PokemonType::DARK,
        Items::MARSHADIUMZ => PokemonType::GHOST,
        Items::PRIMARIUMZ => PokemonType::WATER,
        Items::TAPUNIUMZ => PokemonType::FAIRY,
        Items::SNORLIUMZ | Items::EEVIUMZ => PokemonType::NORMAL,
        Items::MEWNIUMZ => PokemonType::PSYCHIC,
        Items::ULTRANECROZIUMZ => PokemonType::PSYCHIC,
        Items::SOLGANIUMZ => PokemonType::STEEL,
        Items::LUNAIUMZ => PokemonType::GHOST,
        Items::MIMIKIUMZ => PokemonType::FAIRY,
        Items::LYCANIUMZ => PokemonType::ROCK,
        Items::KOMMONIUMZ => PokemonType::DRAGON,
        _ => return None,
    })
}

fn signature(pokemon: PokemonName, move_id: Choices, item: Items) -> Option<ZMove> {
    let (name, category, power) = match (pokemon, move_id, item) {
        (PokemonName::PIKACHU, Choices::VOLTTACKLE, Items::PIKANIUMZ) => {
            ("Catastropika", MoveCategory::Physical, 210.0)
        }
        (
            PokemonName::PIKACHUORIGINAL
            | PokemonName::PIKACHUHOENN
            | PokemonName::PIKACHUSINNOH
            | PokemonName::PIKACHUUNOVA
            | PokemonName::PIKACHUKALOS
            | PokemonName::PIKACHUALOLA
            | PokemonName::PIKACHUPARTNER,
            Choices::THUNDERBOLT,
            Items::PIKASHUNIUMZ,
        ) => ("10,000,000 Volt Thunderbolt", MoveCategory::Special, 195.0),
        (PokemonName::DECIDUEYE, Choices::SPIRITSHACKLE, Items::DECIDIUMZ) => {
            ("Sinister Arrow Raid", MoveCategory::Physical, 180.0)
        }
        (PokemonName::INCINEROAR, Choices::DARKESTLARIAT, Items::INCINIUMZ) => {
            ("Malicious Moonsault", MoveCategory::Physical, 180.0)
        }
        (PokemonName::PRIMARINA, Choices::SPARKLINGARIA, Items::PRIMARIUMZ) => {
            ("Oceanic Operetta", MoveCategory::Special, 195.0)
        }
        (PokemonName::MARSHADOW, Choices::SPECTRALTHIEF, Items::MARSHADIUMZ) => {
            ("Soul-Stealing 7-Star Strike", MoveCategory::Physical, 195.0)
        }
        (PokemonName::RAICHUALOLA, Choices::THUNDERBOLT, Items::ALORAICHIUMZ) => {
            ("Stoked Sparksurfer", MoveCategory::Special, 175.0)
        }
        (PokemonName::SNORLAX, Choices::GIGAIMPACT, Items::SNORLIUMZ) => {
            ("Pulverizing Pancake", MoveCategory::Physical, 210.0)
        }
        (PokemonName::MEW, Choices::PSYCHIC, Items::MEWNIUMZ) => {
            ("Genesis Supernova", MoveCategory::Special, 185.0)
        }
        (PokemonName::EEVEE, Choices::LASTRESORT, Items::EEVIUMZ) => {
            ("Extreme Evoboost", MoveCategory::Status, 0.0)
        }
        (
            PokemonName::TAPUKOKO
            | PokemonName::TAPULELE
            | PokemonName::TAPUBULU
            | PokemonName::TAPUFINI,
            Choices::NATURESMADNESS,
            Items::TAPUNIUMZ,
        ) => ("Guardian of Alola", MoveCategory::Special, 0.0),
        (PokemonName::NECROZMAULTRA, Choices::PHOTONGEYSER, Items::ULTRANECROZIUMZ) => {
            ("Light That Burns the Sky", MoveCategory::Special, 200.0)
        }
        (PokemonName::SOLGALEO, Choices::SUNSTEELSTRIKE, Items::SOLGANIUMZ) => {
            ("Searing Sunraze Smash", MoveCategory::Physical, 200.0)
        }
        (PokemonName::LUNALA, Choices::MOONGEISTBEAM, Items::LUNAIUMZ) => {
            ("Menacing Moonraze Maelstrom", MoveCategory::Special, 200.0)
        }
        (PokemonName::NECROZMADUSKMANE, Choices::SUNSTEELSTRIKE, Items::SOLGANIUMZ) => {
            ("Searing Sunraze Smash", MoveCategory::Physical, 200.0)
        }
        (PokemonName::NECROZMADAWNWINGS, Choices::MOONGEISTBEAM, Items::LUNAIUMZ) => {
            ("Menacing Moonraze Maelstrom", MoveCategory::Special, 200.0)
        }
        (
            PokemonName::MIMIKYU
            | PokemonName::MIMIKYUBUSTED
            | PokemonName::MIMIKYUTOTEM
            | PokemonName::MIMIKYUBUSTEDTOTEM,
            Choices::PLAYROUGH,
            Items::MIMIKIUMZ,
        ) => ("Let's Snuggle Forever", MoveCategory::Physical, 190.0),
        (
            PokemonName::LYCANROC | PokemonName::LYCANROCMIDNIGHT | PokemonName::LYCANROCDUSK,
            Choices::STONEEDGE,
            Items::LYCANIUMZ,
        ) => ("Splintered Stormshards", MoveCategory::Physical, 190.0),
        (
            PokemonName::KOMMOO | PokemonName::KOMMOOTOTEM,
            Choices::CLANGINGSCALES,
            Items::KOMMONIUMZ,
        ) => ("Clangorous Soulblaze", MoveCategory::Special, 185.0),
        _ => return None,
    };
    Some(ZMove {
        name,
        move_type: crystal_type(item)?,
        category,
        base_power: power,
        status: category == MoveCategory::Status,
        status_effect: if item == Items::EEVIUMZ && base_move_id_is_last_resort(move_id) {
            Some(ZStatusEffect::Boost(boosts(2, 2, 2, 2, 2, 0)))
        } else {
            None
        },
        fixed_damage_fraction: if name == "Guardian of Alola" {
            Some(0.75)
        } else {
            None
        },
        terrain_effect: match name {
            "Genesis Supernova" => Some(TerrainEffect::SetPsychic),
            "Splintered Stormshards" => Some(TerrainEffect::Clear),
            _ => None,
        },
    })
}

fn base_move_id_is_last_resort(move_id: Choices) -> bool {
    move_id == Choices::LASTRESORT
}

pub fn get_z_move_for(pokemon: &Pokemon, base_move: &Choice) -> Option<ZMove> {
    if let Some(special) = signature(pokemon.id, base_move.move_id, pokemon.item) {
        return Some(special);
    }
    let move_type = crystal_type(pokemon.item)?;
    if base_move.category == MoveCategory::Status {
        if base_move.move_type != move_type {
            return None;
        }
        return Some(ZMove {
            name: "Z-Power",
            move_type,
            category: MoveCategory::Status,
            base_power: 0.0,
            status: true,
            status_effect: get_z_status_effect(base_move.move_id),
            fixed_damage_fraction: None,
            terrain_effect: None,
        });
    }
    if base_move.move_type != move_type {
        return None;
    }
    let base_power = override_z_move_base_power(
        base_move.move_id,
        get_z_move_base_power(base_move.base_power),
    );
    Some(ZMove {
        name: "Generic Z-Move",
        move_type,
        category: base_move.category,
        base_power,
        status: false,
        status_effect: None,
        fixed_damage_fraction: None,
        terrain_effect: None,
    })
}

#[cfg(test)]
mod tests {
    use super::super::items::Items;
    use super::{
        apply_z_status_effect, boosts, get_z_move_base_power, get_z_move_for, get_z_status_effect,
        override_z_move_base_power, Choices, MoveCategory, TerrainEffect, ZStatusEffect,
    };
    use crate::pokemon::PokemonName;
    use crate::state::Pokemon;

    #[test]
    fn uses_gen7_base_power_breakpoints() {
        assert_eq!(get_z_move_base_power(55.0), 100.0);
        assert_eq!(get_z_move_base_power(60.0), 120.0);
        assert_eq!(get_z_move_base_power(70.0), 140.0);
        assert_eq!(get_z_move_base_power(80.0), 160.0);
        assert_eq!(get_z_move_base_power(90.0), 175.0);
        assert_eq!(get_z_move_base_power(100.0), 180.0);
        assert_eq!(get_z_move_base_power(110.0), 185.0);
        assert_eq!(get_z_move_base_power(120.0), 190.0);
        assert_eq!(get_z_move_base_power(130.0), 195.0);
        assert_eq!(get_z_move_base_power(140.0), 200.0);
    }

    #[test]
    fn applies_move_specific_base_power_overrides() {
        assert_eq!(override_z_move_base_power(Choices::VCREATE, 200.0), 220.0);
        assert_eq!(override_z_move_base_power(Choices::WRINGOUT, 100.0), 190.0);
        assert_eq!(
            override_z_move_base_power(Choices::LANDSWRATH, 175.0),
            185.0
        );
        assert_eq!(override_z_move_base_power(Choices::TACKLE, 100.0), 100.0);
    }

    #[test]
    fn maps_status_z_power_effects() {
        assert!(matches!(
            get_z_status_effect(Choices::SWORDSDANCE),
            Some(ZStatusEffect::ClearNegativeBoosts)
        ));
        assert!(matches!(
            get_z_status_effect(Choices::RECOVER),
            Some(ZStatusEffect::ClearNegativeBoosts)
        ));
        assert!(matches!(
            get_z_status_effect(Choices::HAZE),
            Some(ZStatusEffect::Heal)
        ));
        let mut choice = crate::choices::Choice::default();
        apply_z_status_effect(&mut choice, &ZStatusEffect::CritRatio);
        assert_eq!(choice.z_crit_ratio, 1);
    }

    #[test]
    fn maps_additional_status_z_power_effects() {
        // Sp. Def +2
        assert_eq!(
            get_z_status_effect(Choices::MEANLOOK),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::MUDSPORT),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::NIGHTMARE),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::FORESTSCURSE),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::LUCKYCHANT),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::NATUREPOWER),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 2, 0, 0)))
        );

        // Def +2
        assert_eq!(
            get_z_status_effect(Choices::POWERTRICK),
            Some(ZStatusEffect::Boost(boosts(0, 2, 0, 0, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::GUARDSPLIT),
            Some(ZStatusEffect::Boost(boosts(0, 2, 0, 0, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::GUARDSWAP),
            Some(ZStatusEffect::Boost(boosts(0, 2, 0, 0, 0, 0)))
        );

        // Sp. Atk +2
        assert_eq!(
            get_z_status_effect(Choices::POWERSPLIT),
            Some(ZStatusEffect::Boost(boosts(0, 0, 2, 0, 0, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::POWERSWAP),
            Some(ZStatusEffect::Boost(boosts(0, 0, 2, 0, 0, 0)))
        );

        // Atk +2
        assert_eq!(
            get_z_status_effect(Choices::MIMIC),
            Some(ZStatusEffect::Boost(boosts(2, 0, 0, 0, 0, 0)))
        );

        // Accuracy +2
        assert_eq!(
            get_z_status_effect(Choices::MINDREADER),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 0, 0, 2)))
        );
        assert_eq!(
            get_z_status_effect(Choices::ODORSLEUTH),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 0, 0, 2)))
        );

        // Atk +1 and Sp. Atk +1
        assert_eq!(
            get_z_status_effect(Choices::NOBLEROAR),
            Some(ZStatusEffect::Boost(boosts(1, 0, 1, 0, 0, 0)))
        );

        // Heal
        assert_eq!(
            get_z_status_effect(Choices::HAPPYHOUR),
            Some(ZStatusEffect::Heal)
        );

        // Speed +2
        assert_eq!(
            get_z_status_effect(Choices::ENTRAINMENT),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 0, 2, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::MEFIRST),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 0, 2, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::MIRRORMOVE),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 0, 2, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::RAPIDSPIN),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 0, 2, 0)))
        );
        assert_eq!(
            get_z_status_effect(Choices::ROCKPOLISH),
            Some(ZStatusEffect::Boost(boosts(0, 0, 0, 0, 2, 0)))
        );
    }
    #[test]
    fn generic_status_moves_require_matching_crystal_type() {
        use crate::choices::MOVES;
        use crate::state::Pokemon;

        let mut pokemon = Pokemon::default();
        pokemon.item = Items::NORMALIUMZ;
        let swords_dance = MOVES.get(&Choices::SWORDSDANCE).unwrap();
        assert!(get_z_move_for(&pokemon, swords_dance).is_some());

        pokemon.item = Items::FIRIUMZ;
        assert!(get_z_move_for(&pokemon, swords_dance).is_none());
    }

    #[test]
    fn mcts_root_contains_normal_and_z_actions() {
        use crate::engine::state::MoveChoice;
        use crate::mcts::perform_mcts;
        use crate::state::{PokemonMoveIndex, State};
        use std::time::Duration;

        let mut state = State::default();
        state
            .side_one
            .get_active()
            .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
        state.side_one.get_active().item = Items::NORMALIUMZ;
        state.side_one.allow_z_moves = true;
        state
            .side_two
            .get_active()
            .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

        let (side_one_options, side_two_options) = state.get_all_options();
        let result = perform_mcts(
            &mut state,
            side_one_options,
            side_two_options,
            Duration::ZERO,
            1,
        );

        assert!(result
            .s1
            .iter()
            .any(|node| { node.move_choice == MoveChoice::Move(PokemonMoveIndex::M0) }));
        assert!(result
            .s1
            .iter()
            .any(|node| { node.move_choice == MoveChoice::MoveZ(PokemonMoveIndex::M0) }));
        assert!(!state.side_one.z_move_used);
    }

    #[test]
    fn used_z_resource_removes_z_action() {
        use crate::engine::state::MoveChoice;
        use crate::state::{PokemonMoveIndex, State};

        let mut state = State::default();
        state
            .side_one
            .get_active()
            .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
        state.side_one.get_active().item = Items::NORMALIUMZ;
        state.side_one.allow_z_moves = true;
        state.side_one.z_move_used = true;

        let (side_one_options, _) = state.get_all_options();
        assert!(!side_one_options
            .iter()
            .any(|choice| matches!(choice, MoveChoice::MoveZ(_))));
    }

    #[test]
    fn signature_status_and_ultra_burst_metadata_are_specialized() {
        use crate::state::Pokemon;

        let mut eevee = Pokemon::default();
        eevee.id = PokemonName::EEVEE;
        eevee.item = Items::EEVIUMZ;
        let last_resort = crate::choices::MOVES.get(&Choices::LASTRESORT).unwrap();
        let extreme_evoboost = get_z_move_for(&eevee, last_resort).unwrap();
        assert!(extreme_evoboost.status);
        assert!(matches!(
            extreme_evoboost.status_effect,
            Some(ZStatusEffect::Boost(_))
        ));

        for form in [
            PokemonName::NECROZMA,
            PokemonName::NECROZMADUSKMANE,
            PokemonName::NECROZMADAWNWINGS,
        ] {
            assert_eq!(
                form.ultra_burst_target(Items::ULTRANECROZIUMZ).unwrap().id,
                PokemonName::NECROZMAULTRA
            );
        }
    }

    #[test]
    fn guardian_of_alola_deals_three_quarters_of_current_hp() {
        use crate::engine::damage_calc::{calculate_damage, DamageRolls};
        use crate::state::{SideReference, State};

        let mut state = State::default();
        state.side_one.get_active().id = PokemonName::TAPUKOKO;
        state.side_one.get_active().item = Items::TAPUNIUMZ;
        state.side_two.get_active().hp = 200;
        let base_move = crate::choices::MOVES.get(&Choices::NATURESMADNESS).unwrap();
        let z_move = get_z_move_for(state.side_one.get_active(), base_move).unwrap();
        let mut choice = base_move.clone();
        choice.move_type = z_move.move_type;
        choice.category = z_move.category;
        choice.base_power = z_move.base_power;
        choice.z_fixed_damage_fraction = z_move.fixed_damage_fraction;
        assert_eq!(
            calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Max),
            Some((150, 150))
        );
    }

    #[test]
    fn guardian_of_alola_uses_exact_current_hp_rounding() {
        use crate::engine::damage_calc::{calculate_damage, DamageRolls};
        use crate::state::{SideReference, State};

        for hp in [1, 2, 99, 100, 101, 999] {
            let mut state = State::default();
            state.side_one.get_active().id = PokemonName::TAPUKOKO;
            state.side_one.get_active().item = Items::TAPUNIUMZ;
            state.side_two.get_active().hp = hp;
            let base_move = crate::choices::MOVES.get(&Choices::NATURESMADNESS).unwrap();
            let z_move = get_z_move_for(state.side_one.get_active(), base_move).unwrap();
            let mut choice = base_move.clone();
            choice.move_type = z_move.move_type;
            choice.category = z_move.category;
            choice.base_power = z_move.base_power;
            choice.z_fixed_damage_fraction = z_move.fixed_damage_fraction;
            let damage =
                calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Max)
                    .unwrap()
                    .0;
            assert_eq!(damage, (hp as f32 * 0.75) as i16);
        }
    }

    #[test]
    fn executing_z_action_consumes_and_reverses_resource() {
        use crate::engine::generate_instructions::generate_instructions_from_move_pair;
        use crate::engine::state::MoveChoice;
        use crate::state::{PokemonMoveIndex, State};

        let mut state = State::default();
        state
            .side_one
            .get_active()
            .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
        state.side_one.get_active().item = Items::NORMALIUMZ;
        state.side_one.allow_z_moves = true;
        state
            .side_two
            .get_active()
            .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
        let branches = generate_instructions_from_move_pair(
            &mut state,
            &MoveChoice::MoveZ(PokemonMoveIndex::M0),
            &MoveChoice::Move(PokemonMoveIndex::M0),
            false,
        );
        assert!(!branches.is_empty());
        for branch in branches {
            state.apply_instructions(&branch.instruction_list);
            assert!(state.side_one.z_move_used);
            state.reverse_instructions(&branch.instruction_list);
            assert!(!state.side_one.z_move_used);
        }
    }

    #[test]
    fn signature_z_moves_apply_reversible_terrain_effects() {
        use super::super::state::Terrain;
        use crate::engine::generate_instructions::generate_instructions_from_move_pair;
        use crate::engine::state::MoveChoice;
        use crate::state::{PokemonMoveIndex, State};

        let cases = [
            (
                PokemonName::MEW,
                Items::MEWNIUMZ,
                Choices::PSYCHIC,
                Terrain::PSYCHICTERRAIN,
            ),
            (
                PokemonName::LYCANROC,
                Items::LYCANIUMZ,
                Choices::STONEEDGE,
                Terrain::NONE,
            ),
        ];
        for (pokemon, item, move_id, expected_terrain) in cases {
            let mut state = State::default();
            state.side_one.get_active().id = pokemon;
            state.side_one.get_active().item = item;
            state
                .side_one
                .get_active()
                .replace_move(PokemonMoveIndex::M0, move_id);
            state.side_one.allow_z_moves = true;
            state
                .side_two
                .get_active()
                .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);
            state.terrain.terrain_type = Terrain::ELECTRICTERRAIN;
            state.terrain.turns_remaining = 3;
            let branches = generate_instructions_from_move_pair(
                &mut state,
                &MoveChoice::MoveZ(PokemonMoveIndex::M0),
                &MoveChoice::Move(PokemonMoveIndex::M0),
                false,
            );
            assert!(!branches.is_empty());
            state.apply_instructions(&branches[0].instruction_list);
            assert_eq!(state.terrain.terrain_type, expected_terrain);
            state.reverse_instructions(&branches[0].instruction_list);
            assert_eq!(state.terrain.terrain_type, Terrain::ELECTRICTERRAIN);
            assert_eq!(state.terrain.turns_remaining, 3);
        }
    }

    #[test]
    fn test_all_signature_z_moves() {
        let test_cases = [
            // (pokemon, move, item, expected_name, expected_category, expected_power, expected_status, expected_status_effect, expected_fixed_damage_fraction, expected_terrain_effect)
            (
                PokemonName::PIKACHU,
                Choices::VOLTTACKLE,
                Items::PIKANIUMZ,
                "Catastropika",
                MoveCategory::Physical,
                210.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PIKACHUORIGINAL,
                Choices::THUNDERBOLT,
                Items::PIKASHUNIUMZ,
                "10,000,000 Volt Thunderbolt",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PIKACHUHOENN,
                Choices::THUNDERBOLT,
                Items::PIKASHUNIUMZ,
                "10,000,000 Volt Thunderbolt",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PIKACHUSINNOH,
                Choices::THUNDERBOLT,
                Items::PIKASHUNIUMZ,
                "10,000,000 Volt Thunderbolt",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PIKACHUUNOVA,
                Choices::THUNDERBOLT,
                Items::PIKASHUNIUMZ,
                "10,000,000 Volt Thunderbolt",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PIKACHUKALOS,
                Choices::THUNDERBOLT,
                Items::PIKASHUNIUMZ,
                "10,000,000 Volt Thunderbolt",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PIKACHUALOLA,
                Choices::THUNDERBOLT,
                Items::PIKASHUNIUMZ,
                "10,000,000 Volt Thunderbolt",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PIKACHUPARTNER,
                Choices::THUNDERBOLT,
                Items::PIKASHUNIUMZ,
                "10,000,000 Volt Thunderbolt",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::DECIDUEYE,
                Choices::SPIRITSHACKLE,
                Items::DECIDIUMZ,
                "Sinister Arrow Raid",
                MoveCategory::Physical,
                180.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::INCINEROAR,
                Choices::DARKESTLARIAT,
                Items::INCINIUMZ,
                "Malicious Moonsault",
                MoveCategory::Physical,
                180.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::PRIMARINA,
                Choices::SPARKLINGARIA,
                Items::PRIMARIUMZ,
                "Oceanic Operetta",
                MoveCategory::Special,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::MARSHADOW,
                Choices::SPECTRALTHIEF,
                Items::MARSHADIUMZ,
                "Soul-Stealing 7-Star Strike",
                MoveCategory::Physical,
                195.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::RAICHUALOLA,
                Choices::THUNDERBOLT,
                Items::ALORAICHIUMZ,
                "Stoked Sparksurfer",
                MoveCategory::Special,
                175.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::SNORLAX,
                Choices::GIGAIMPACT,
                Items::SNORLIUMZ,
                "Pulverizing Pancake",
                MoveCategory::Physical,
                210.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::MEW,
                Choices::PSYCHIC,
                Items::MEWNIUMZ,
                "Genesis Supernova",
                MoveCategory::Special,
                185.0,
                false,
                None,
                None,
                Some(TerrainEffect::SetPsychic),
            ),
            (
                PokemonName::EEVEE,
                Choices::LASTRESORT,
                Items::EEVIUMZ,
                "Extreme Evoboost",
                MoveCategory::Status,
                0.0,
                true,
                Some(ZStatusEffect::Boost(boosts(2, 2, 2, 2, 2, 0))),
                None,
                None,
            ),
            (
                PokemonName::TAPUKOKO,
                Choices::NATURESMADNESS,
                Items::TAPUNIUMZ,
                "Guardian of Alola",
                MoveCategory::Special,
                0.0,
                false,
                None,
                Some(0.75),
                None,
            ),
            (
                PokemonName::TAPULELE,
                Choices::NATURESMADNESS,
                Items::TAPUNIUMZ,
                "Guardian of Alola",
                MoveCategory::Special,
                0.0,
                false,
                None,
                Some(0.75),
                None,
            ),
            (
                PokemonName::TAPUBULU,
                Choices::NATURESMADNESS,
                Items::TAPUNIUMZ,
                "Guardian of Alola",
                MoveCategory::Special,
                0.0,
                false,
                None,
                Some(0.75),
                None,
            ),
            (
                PokemonName::TAPUFINI,
                Choices::NATURESMADNESS,
                Items::TAPUNIUMZ,
                "Guardian of Alola",
                MoveCategory::Special,
                0.0,
                false,
                None,
                Some(0.75),
                None,
            ),
            (
                PokemonName::NECROZMAULTRA,
                Choices::PHOTONGEYSER,
                Items::ULTRANECROZIUMZ,
                "Light That Burns the Sky",
                MoveCategory::Special,
                200.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::SOLGALEO,
                Choices::SUNSTEELSTRIKE,
                Items::SOLGANIUMZ,
                "Searing Sunraze Smash",
                MoveCategory::Physical,
                200.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::LUNALA,
                Choices::MOONGEISTBEAM,
                Items::LUNAIUMZ,
                "Menacing Moonraze Maelstrom",
                MoveCategory::Special,
                200.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::NECROZMADUSKMANE,
                Choices::SUNSTEELSTRIKE,
                Items::SOLGANIUMZ,
                "Searing Sunraze Smash",
                MoveCategory::Physical,
                200.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::NECROZMADAWNWINGS,
                Choices::MOONGEISTBEAM,
                Items::LUNAIUMZ,
                "Menacing Moonraze Maelstrom",
                MoveCategory::Special,
                200.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::MIMIKYU,
                Choices::PLAYROUGH,
                Items::MIMIKIUMZ,
                "Let's Snuggle Forever",
                MoveCategory::Physical,
                190.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::MIMIKYUBUSTED,
                Choices::PLAYROUGH,
                Items::MIMIKIUMZ,
                "Let's Snuggle Forever",
                MoveCategory::Physical,
                190.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::MIMIKYUTOTEM,
                Choices::PLAYROUGH,
                Items::MIMIKIUMZ,
                "Let's Snuggle Forever",
                MoveCategory::Physical,
                190.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::MIMIKYUBUSTEDTOTEM,
                Choices::PLAYROUGH,
                Items::MIMIKIUMZ,
                "Let's Snuggle Forever",
                MoveCategory::Physical,
                190.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::LYCANROC,
                Choices::STONEEDGE,
                Items::LYCANIUMZ,
                "Splintered Stormshards",
                MoveCategory::Physical,
                190.0,
                false,
                None,
                None,
                Some(TerrainEffect::Clear),
            ),
            (
                PokemonName::LYCANROCMIDNIGHT,
                Choices::STONEEDGE,
                Items::LYCANIUMZ,
                "Splintered Stormshards",
                MoveCategory::Physical,
                190.0,
                false,
                None,
                None,
                Some(TerrainEffect::Clear),
            ),
            (
                PokemonName::LYCANROCDUSK,
                Choices::STONEEDGE,
                Items::LYCANIUMZ,
                "Splintered Stormshards",
                MoveCategory::Physical,
                190.0,
                false,
                None,
                None,
                Some(TerrainEffect::Clear),
            ),
            (
                PokemonName::KOMMOO,
                Choices::CLANGINGSCALES,
                Items::KOMMONIUMZ,
                "Clangorous Soulblaze",
                MoveCategory::Special,
                185.0,
                false,
                None,
                None,
                None,
            ),
            (
                PokemonName::KOMMOOTOTEM,
                Choices::CLANGINGSCALES,
                Items::KOMMONIUMZ,
                "Clangorous Soulblaze",
                MoveCategory::Special,
                185.0,
                false,
                None,
                None,
                None,
            ),
        ];

        for (
            test_pokemon,
            move_id,
            item,
            expected_name,
            expected_category,
            expected_power,
            expected_status,
            expected_status_effect,
            expected_fixed_damage_fraction,
            expected_terrain_effect,
        ) in test_cases.iter()
        {
            let mut pokemon = Pokemon::default();
            pokemon.id = *test_pokemon;
            pokemon.item = *item;
            let base_move = crate::choices::MOVES.get(move_id).expect("Move not found");
            let z_move = get_z_move_for(&pokemon, base_move)
                .expect("Failed to get Z-Move for valid combination");

            assert_eq!(
                z_move.name, *expected_name,
                "Failed for {:?} {} {:?}",
                test_pokemon, move_id, item
            );
            assert_eq!(
                z_move.category, *expected_category,
                "Failed for {:?} {} {:?}",
                test_pokemon, move_id, item
            );
            assert_eq!(
                z_move.base_power, *expected_power,
                "Failed for {:?} {} {:?}",
                test_pokemon, move_id, item
            );
            assert_eq!(
                z_move.status, *expected_status,
                "Failed for {:?} {} {:?}",
                test_pokemon, move_id, item
            );
            assert_eq!(
                z_move.status_effect.as_ref(),
                expected_status_effect.as_ref(),
                "Failed for {:?} {} {:?}",
                test_pokemon,
                move_id,
                item
            );
            assert_eq!(
                z_move.fixed_damage_fraction.as_ref(),
                expected_fixed_damage_fraction.as_ref(),
                "Failed for {:?} {} {:?}",
                test_pokemon,
                move_id,
                item
            );
            assert_eq!(
                z_move.terrain_effect.as_ref(),
                expected_terrain_effect.as_ref(),
                "Failed for {:?} {} {:?}",
                test_pokemon,
                move_id,
                item
            );
        }
    }
}
