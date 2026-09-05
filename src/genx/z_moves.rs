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
        Items::LUNALIUMZ => PokemonType::GHOST,
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
        (PokemonName::LUNALA, Choices::MOONGEISTBEAM, Items::LUNALIUMZ) => {
            ("Menacing Moonraze Maelstrom", MoveCategory::Special, 200.0)
        }
        (PokemonName::NECROZMADUSKMANE, Choices::SUNSTEELSTRIKE, Items::SOLGANIUMZ) => {
            ("Searing Sunraze Smash", MoveCategory::Physical, 200.0)
        }
        (PokemonName::NECROZMADAWNWINGS, Choices::MOONGEISTBEAM, Items::LUNALIUMZ) => {
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
