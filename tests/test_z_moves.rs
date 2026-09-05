#![cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]

use poke_engine::choices::{Choice, Choices, MoveCategory, StatBoosts, MOVES};
use poke_engine::engine::evaluate::evaluate;
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::items::Items;
use poke_engine::engine::state::{MoveChoice, Terrain};
use poke_engine::engine::z_moves::{
    apply_z_status_effect, get_z_move_base_power, get_z_move_for, get_z_status_effect,
    TerrainEffect, ZStatusEffect,
};
use poke_engine::instruction::Instruction;
use poke_engine::mcts::{perform_mcts, MctsSideResult};
use poke_engine::pokemon::PokemonName;
use poke_engine::state::{LastUsedMove, Pokemon, PokemonMoveIndex, SideReference, State};
use std::time::Duration;

fn z_state(item: Items, move_id: Choices) -> State {
    let mut state = State::default();
    state.side_one.allow_z_moves = true;
    state.side_one.get_active().item = item;
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, move_id);
    state
}

fn side_one_options(state: &State) -> Vec<MoveChoice> {
    state.get_all_options().0
}

fn has_z_move(options: &[MoveChoice]) -> bool {
    options
        .iter()
        .any(|choice| matches!(choice, MoveChoice::MoveZ(PokemonMoveIndex::M0)))
}

fn most_visited_move(options: &[MctsSideResult]) -> &MoveChoice {
    &options
        .iter()
        .max_by_key(|option| option.visits)
        .expect("MCTS must return at least one legal option")
        .move_choice
}

fn search(state: &mut State) -> (Vec<MctsSideResult>, Vec<MctsSideResult>) {
    let (side_one_options, side_two_options) = state.root_get_all_options();
    let result = perform_mcts(
        state,
        side_one_options,
        side_two_options,
        Duration::ZERO,
        10_000,
    );
    (result.s1, result.s2)
}

fn score_for(options: &[MctsSideResult], move_choice: MoveChoice) -> f32 {
    options
        .iter()
        .find(|option| option.move_choice == move_choice)
        .expect("MCTS must score every legal option")
        .average_score()
}

fn apply_turn(state: &State, side_one_move: MoveChoice, side_two_move: MoveChoice) -> State {
    let mut state_for_generation = state.clone();
    let instructions = generate_instructions_from_move_pair(
        &mut state_for_generation,
        &side_one_move,
        &side_two_move,
        false,
    );
    assert_eq!(instructions.len(), 1, "fixture turn must be deterministic");

    let mut next_state = state.clone();
    next_state.apply_instructions(&instructions[0].instruction_list);
    next_state
}

fn disable_extra_active_moves(state: &mut State) {
    for move_index in [
        PokemonMoveIndex::M1,
        PokemonMoveIndex::M2,
        PokemonMoveIndex::M3,
    ] {
        state.side_one.get_active().moves[&move_index].disabled = true;
        state.side_two.get_active().moves[&move_index].disabled = true;
    }
}

fn tailwhip_tie_state() -> State {
    let mut state = z_state(Items::NORMALIUMZ, Choices::TAILWHIP);
    state.side_one.force_trapped = true;
    state.side_two.force_trapped = true;
    disable_extra_active_moves(&mut state);
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().speed = 1;
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TAUNT);
    state
}

#[test]
fn evaluator_z_available_vs_spent() {
    let state_with_z = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    let mut state_with_spent_z = state_with_z.clone();
    state_with_spent_z.side_one.z_move_used = true;

    assert!(evaluate(&state_with_z) > evaluate(&state_with_spent_z));
}

#[test]
fn evaluator_z_side_symmetry() {
    let mut side_one_has_z = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    side_one_has_z.side_two.z_move_used = true;

    let mut side_two_has_z = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    side_two_has_z.side_one.z_move_used = true;
    side_two_has_z.side_two.z_move_used = false;

    assert!(evaluate(&side_one_has_z) > evaluate(&side_two_has_z));
}

#[test]
fn z_action_requires_a_compatible_crystal_and_move() {
    let compatible = z_state(Items::WATERIUMZ, Choices::SURF);
    let incompatible_crystal = z_state(Items::FIRIUMZ, Choices::SURF);
    let non_z_item = z_state(Items::LEFTOVERS, Choices::SURF);

    assert!(has_z_move(&side_one_options(&compatible)));
    assert!(!has_z_move(&side_one_options(&incompatible_crystal)));
    assert!(!has_z_move(&side_one_options(&non_z_item)));
}

#[test]
fn status_and_signature_z_actions_use_engine_legality() {
    let status = z_state(Items::NORMALIUMZ, Choices::SPLASH);
    assert!(has_z_move(&side_one_options(&status)));

    let mut signature = z_state(Items::PIKANIUMZ, Choices::VOLTTACKLE);
    signature.side_one.get_active().id = PokemonName::PIKACHU;
    assert!(has_z_move(&side_one_options(&signature)));
}

#[test]
fn z_transition_marks_resource_used_and_reverses() {
    let mut state = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    let instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::MoveZ(PokemonMoveIndex::M0),
        &MoveChoice::None,
        false,
    );

    assert!(instructions.iter().all(|branch| {
        branch
            .instruction_list
            .iter()
            .any(|instruction| matches!(instruction, Instruction::ToggleZMoveUsed(_)))
    }));

    let mut spent_branch = state.clone();
    spent_branch.apply_instructions(&instructions[0].instruction_list);
    assert!(spent_branch.side_one.z_move_used);
    spent_branch.reverse_instructions(&instructions[0].instruction_list);
    assert!(!spent_branch.side_one.z_move_used);
}

#[test]
fn z_branch_does_not_consume_sibling_resource() {
    let mut state = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    let z_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::MoveZ(PokemonMoveIndex::M0),
        &MoveChoice::None,
        false,
    );
    let normal_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::None,
        false,
    );

    let mut z_branch = state.clone();
    z_branch.apply_instructions(&z_instructions[0].instruction_list);
    let mut normal_branch = state.clone();
    normal_branch.apply_instructions(&normal_instructions[0].instruction_list);

    assert!(z_branch.side_one.z_move_used);
    assert!(!normal_branch.side_one.z_move_used);
    assert!(has_z_move(&side_one_options(&normal_branch)));
}

#[test]
fn mcts_spends_z_for_a_decisive_ko() {
    let mut state = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    state.side_one.get_active().hp = 20;
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().hp = 50;
    state.side_two.get_active().maxhp = 50;
    state.side_two.get_active().speed = 1;
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TACKLE);

    let (side_one, _) = search(&mut state);

    assert_eq!(
        most_visited_move(&side_one),
        &MoveChoice::MoveZ(PokemonMoveIndex::M0)
    );
}

#[test]
fn mcts_conserves_z_when_the_nonlethal_damage_gain_is_marginal() {
    let mut state = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    state.side_one.force_trapped = true;
    state.side_two.force_trapped = true;
    disable_extra_active_moves(&mut state);
    state.side_one.get_active().speed = 200;
    state.side_two.get_active().speed = 1;
    state.side_two.get_active().hp = 100;
    state.side_two.get_active().maxhp = 100;
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);

    let (side_one, _) = search(&mut state);

    assert_eq!(
        most_visited_move(&side_one),
        &MoveChoice::Move(PokemonMoveIndex::M0)
    );
}

#[test]
fn mcts_conserves_z_when_normal_and_z_actions_have_equal_immediate_state() {
    let mut state = tailwhip_tie_state();
    let normal_after = apply_turn(
        &state,
        MoveChoice::Move(PokemonMoveIndex::M0),
        MoveChoice::Move(PokemonMoveIndex::M0),
    );
    let z_after = apply_turn(
        &state,
        MoveChoice::MoveZ(PokemonMoveIndex::M0),
        MoveChoice::Move(PokemonMoveIndex::M0),
    );
    let mut normal_after_with_spent_z = normal_after.clone();
    normal_after_with_spent_z.side_one.z_move_used = true;

    assert_eq!(
        format!("{:?}", normal_after_with_spent_z),
        format!("{:?}", z_after)
    );
    assert!(evaluate(&normal_after) > evaluate(&z_after));

    let (side_one, _) = search(&mut state);

    assert_eq!(
        most_visited_move(&side_one),
        &MoveChoice::Move(PokemonMoveIndex::M0)
    );
    assert!(
        score_for(&side_one, MoveChoice::Move(PokemonMoveIndex::M0))
            > score_for(&side_one, MoveChoice::MoveZ(PokemonMoveIndex::M0))
    );
}

#[test]
fn mcts_preserves_z_for_a_stronger_turn_two_blood_moon() {
    let mut state = tailwhip_tie_state();
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::BLOODMOON);
    state.side_one.get_active().moves[&PokemonMoveIndex::M1].disabled = false;
    state.use_last_used_move = true;
    state.side_one.last_used_move = LastUsedMove::Move(PokemonMoveIndex::M1);
    state.side_one.get_active().hp = 1;
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M1, Choices::GIGATONHAMMER);
    state.side_two.get_active().moves[&PokemonMoveIndex::M1].disabled = false;
    state.side_two.last_used_move = LastUsedMove::Move(PokemonMoveIndex::M1);
    state.side_two.get_active().hp = 230;
    state.side_two.get_active().maxhp = 230;

    let normal_turn_one = apply_turn(
        &state,
        MoveChoice::Move(PokemonMoveIndex::M0),
        MoveChoice::Move(PokemonMoveIndex::M0),
    );
    let z_turn_one = apply_turn(
        &state,
        MoveChoice::MoveZ(PokemonMoveIndex::M0),
        MoveChoice::Move(PokemonMoveIndex::M0),
    );
    let mut normalized_normal_turn_one = normal_turn_one.clone();
    normalized_normal_turn_one.side_one.z_move_used = true;
    assert_eq!(
        format!("{:?}", normalized_normal_turn_one),
        format!("{:?}", z_turn_one)
    );

    let mut normal_turn_two = apply_turn(
        &normal_turn_one,
        MoveChoice::Move(PokemonMoveIndex::M1),
        MoveChoice::Move(PokemonMoveIndex::M1),
    );
    let mut z_turn_two = apply_turn(
        &normal_turn_one,
        MoveChoice::MoveZ(PokemonMoveIndex::M1),
        MoveChoice::Move(PokemonMoveIndex::M1),
    );

    assert_eq!(normal_turn_two.side_one.get_active().hp, 0);
    assert!(normal_turn_two.side_two.get_active().hp > 0);
    assert_eq!(z_turn_two.side_two.get_active().hp, 0);
    let mut normalized_normal_turn_two = normal_turn_two.clone();
    normalized_normal_turn_two.side_one.z_move_used = true;
    assert!(evaluate(&z_turn_two) > evaluate(&normalized_normal_turn_two));

    assert!(
        side_one_options(&normal_turn_one)
            .iter()
            .any(|choice| *choice == MoveChoice::Move(PokemonMoveIndex::M1)),
        "Blood Moon must become legal after Tail Whip: last_used={:?}, volatiles={:?}, force_switch={}",
        normal_turn_one.side_one.last_used_move,
        normal_turn_one.side_one.volatile_statuses,
        normal_turn_one.side_one.force_switch,
    );

    let (side_one, _) = search(&mut state);
    let (unspent_turn_two, _) = search(&mut normal_turn_one.clone());
    let (spent_turn_two, _) = search(&mut z_turn_one.clone());

    assert_eq!(
        most_visited_move(&side_one),
        &MoveChoice::Move(PokemonMoveIndex::M0)
    );
    assert_eq!(
        most_visited_move(&unspent_turn_two),
        &MoveChoice::MoveZ(PokemonMoveIndex::M1)
    );
    assert!(
        score_for(&unspent_turn_two, MoveChoice::MoveZ(PokemonMoveIndex::M1))
            > score_for(&unspent_turn_two, MoveChoice::Move(PokemonMoveIndex::M1))
    );
    assert_eq!(
        most_visited_move(&spent_turn_two),
        &MoveChoice::Move(PokemonMoveIndex::M1)
    );
}

#[test]
fn mcts_opponent_z_actions_are_world_specific_and_adversarial() {
    let mut z_world = State::default();
    z_world.side_two.allow_z_moves = true;
    z_world.side_two.get_active().item = Items::WATERIUMZ;
    z_world
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SURF);
    let (_, z_options) = search(&mut z_world);

    let mut non_z_world = z_world.clone();
    non_z_world.side_two.get_active().item = Items::LEFTOVERS;
    let (_, non_z_options) = search(&mut non_z_world);

    assert!(z_options
        .iter()
        .any(|option| option.move_choice == MoveChoice::MoveZ(PokemonMoveIndex::M0)));
    assert!(!non_z_options
        .iter()
        .any(|option| option.move_choice == MoveChoice::MoveZ(PokemonMoveIndex::M0)));
}

#[test]
fn mcts_opponent_conserves_z_when_normal_and_z_actions_have_equal_immediate_state() {
    let mut state = State::default();
    state.side_one.force_trapped = true;
    state.side_two.force_trapped = true;
    disable_extra_active_moves(&mut state);
    state
        .side_one
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::SPLASH);
    state.side_two.allow_z_moves = true;
    state.side_two.get_active().item = Items::NORMALIUMZ;
    state
        .side_two
        .get_active()
        .replace_move(PokemonMoveIndex::M0, Choices::TAILWHIP);

    let (_, side_two) = search(&mut state);

    assert_eq!(
        most_visited_move(&side_two),
        &MoveChoice::Move(PokemonMoveIndex::M0)
    );
}

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
    for (item, move_id, expected_power) in [
        (Items::FIRIUMZ, Choices::VCREATE, 220.0),
        (Items::NORMALIUMZ, Choices::WRINGOUT, 190.0),
        (Items::GROUNDIUMZ, Choices::LANDSWRATH, 185.0),
        (Items::NORMALIUMZ, Choices::TACKLE, 100.0),
    ] {
        let mut pokemon = Pokemon::default();
        pokemon.item = item;
        let z_move = get_z_move_for(&pokemon, MOVES.get(&move_id).unwrap()).unwrap();
        assert_eq!(z_move.base_power, expected_power, "failed for {move_id:?}");
    }
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
    let mut choice = Choice::default();
    apply_z_status_effect(&mut choice, &ZStatusEffect::CritRatio);
    assert_eq!(choice.z_crit_ratio, 1);
}

#[test]
fn maps_additional_status_z_power_effects() {
    let special_defense = StatBoosts {
        special_defense: 2,
        ..Default::default()
    };
    for move_id in [
        Choices::MEANLOOK,
        Choices::MUDSPORT,
        Choices::NIGHTMARE,
        Choices::FORESTSCURSE,
        Choices::LUCKYCHANT,
        Choices::NATUREPOWER,
    ] {
        assert_eq!(
            get_z_status_effect(move_id),
            Some(ZStatusEffect::Boost(special_defense.clone()))
        );
    }

    for (move_id, expected) in [
        (
            Choices::POWERTRICK,
            StatBoosts {
                defense: 2,
                ..Default::default()
            },
        ),
        (
            Choices::GUARDSPLIT,
            StatBoosts {
                defense: 2,
                ..Default::default()
            },
        ),
        (
            Choices::GUARDSWAP,
            StatBoosts {
                defense: 2,
                ..Default::default()
            },
        ),
        (
            Choices::POWERSPLIT,
            StatBoosts {
                special_attack: 2,
                ..Default::default()
            },
        ),
        (
            Choices::POWERSWAP,
            StatBoosts {
                special_attack: 2,
                ..Default::default()
            },
        ),
        (
            Choices::MIMIC,
            StatBoosts {
                attack: 2,
                ..Default::default()
            },
        ),
        (
            Choices::MINDREADER,
            StatBoosts {
                accuracy: 2,
                ..Default::default()
            },
        ),
        (
            Choices::ODORSLEUTH,
            StatBoosts {
                accuracy: 2,
                ..Default::default()
            },
        ),
        (
            Choices::NOBLEROAR,
            StatBoosts {
                attack: 1,
                special_attack: 1,
                ..Default::default()
            },
        ),
        (
            Choices::ENTRAINMENT,
            StatBoosts {
                speed: 2,
                ..Default::default()
            },
        ),
        (
            Choices::MEFIRST,
            StatBoosts {
                speed: 2,
                ..Default::default()
            },
        ),
        (
            Choices::MIRRORMOVE,
            StatBoosts {
                speed: 2,
                ..Default::default()
            },
        ),
        (
            Choices::RAPIDSPIN,
            StatBoosts {
                speed: 2,
                ..Default::default()
            },
        ),
        (
            Choices::ROCKPOLISH,
            StatBoosts {
                speed: 2,
                ..Default::default()
            },
        ),
    ] {
        assert_eq!(
            get_z_status_effect(move_id),
            Some(ZStatusEffect::Boost(expected))
        );
    }
    assert_eq!(
        get_z_status_effect(Choices::HAPPYHOUR),
        Some(ZStatusEffect::Heal)
    );
}

#[test]
fn generic_status_moves_require_matching_crystal_type() {
    let mut pokemon = Pokemon::default();
    pokemon.item = Items::NORMALIUMZ;
    let swords_dance = MOVES.get(&Choices::SWORDSDANCE).unwrap();
    assert!(get_z_move_for(&pokemon, swords_dance).is_some());

    pokemon.item = Items::FIRIUMZ;
    assert!(get_z_move_for(&pokemon, swords_dance).is_none());
}

#[test]
fn mcts_root_contains_normal_and_z_actions() {
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
        .any(|node| node.move_choice == MoveChoice::Move(PokemonMoveIndex::M0)));
    assert!(result
        .s1
        .iter()
        .any(|node| node.move_choice == MoveChoice::MoveZ(PokemonMoveIndex::M0)));
    assert!(!state.side_one.z_move_used);
}

#[test]
fn used_z_resource_removes_z_action() {
    let mut state = z_state(Items::NORMALIUMZ, Choices::TACKLE);
    state.side_one.z_move_used = true;

    assert!(!side_one_options(&state)
        .iter()
        .any(|choice| matches!(choice, MoveChoice::MoveZ(_))));
}

#[test]
fn signature_status_and_ultra_burst_metadata_are_specialized() {
    let mut eevee = Pokemon::default();
    eevee.id = PokemonName::EEVEE;
    eevee.item = Items::EEVIUMZ;
    let extreme_evoboost =
        get_z_move_for(&eevee, MOVES.get(&Choices::LASTRESORT).unwrap()).unwrap();
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
    use poke_engine::engine::damage_calc::{calculate_damage, DamageRolls};

    let mut state = State::default();
    state.side_one.get_active().id = PokemonName::TAPUKOKO;
    state.side_one.get_active().item = Items::TAPUNIUMZ;
    state.side_two.get_active().hp = 200;
    let z_move = get_z_move_for(
        state.side_one.get_active(),
        MOVES.get(&Choices::NATURESMADNESS).unwrap(),
    )
    .unwrap();
    let mut choice = MOVES.get(&Choices::NATURESMADNESS).unwrap().clone();
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
    use poke_engine::engine::damage_calc::{calculate_damage, DamageRolls};

    for hp in [1, 2, 99, 100, 101, 999] {
        let mut state = State::default();
        state.side_one.get_active().id = PokemonName::TAPUKOKO;
        state.side_one.get_active().item = Items::TAPUNIUMZ;
        state.side_two.get_active().hp = hp;
        let z_move = get_z_move_for(
            state.side_one.get_active(),
            MOVES.get(&Choices::NATURESMADNESS).unwrap(),
        )
        .unwrap();
        let mut choice = MOVES.get(&Choices::NATURESMADNESS).unwrap().clone();
        choice.move_type = z_move.move_type;
        choice.category = z_move.category;
        choice.base_power = z_move.base_power;
        choice.z_fixed_damage_fraction = z_move.fixed_damage_fraction;
        let damage = calculate_damage(&state, &SideReference::SideOne, &choice, DamageRolls::Max)
            .unwrap()
            .0;
        assert_eq!(damage, (hp as f32 * 0.75) as i16);
    }
}

#[test]
fn executing_z_action_consumes_and_reverses_resource() {
    let mut state = z_state(Items::NORMALIUMZ, Choices::TACKLE);
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
    for (pokemon, item, move_id, expected_terrain) in [
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
    ] {
        let mut state = z_state(item, move_id);
        state.side_one.get_active().id = pokemon;
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
            Some(ZStatusEffect::Boost(StatBoosts {
                attack: 2,
                defense: 2,
                special_attack: 2,
                special_defense: 2,
                speed: 2,
                accuracy: 0,
            })),
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
            Items::LUNALIUMZ,
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
            Items::LUNALIUMZ,
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
        pokemon_id,
        move_id,
        item,
        name,
        category,
        base_power,
        status,
        status_effect,
        fixed_damage_fraction,
        terrain_effect,
    ) in test_cases
    {
        let mut pokemon = Pokemon::default();
        pokemon.id = pokemon_id;
        pokemon.item = item;
        let z_move = get_z_move_for(&pokemon, MOVES.get(&move_id).unwrap()).unwrap();
        assert_eq!(
            z_move.name, name,
            "failed for {pokemon_id:?} {move_id} {item:?}"
        );
        assert_eq!(
            z_move.category, category,
            "failed for {pokemon_id:?} {move_id} {item:?}"
        );
        assert_eq!(
            z_move.base_power, base_power,
            "failed for {pokemon_id:?} {move_id} {item:?}"
        );
        assert_eq!(
            z_move.status, status,
            "failed for {pokemon_id:?} {move_id} {item:?}"
        );
        assert_eq!(
            z_move.status_effect, status_effect,
            "failed for {pokemon_id:?} {move_id} {item:?}"
        );
        assert_eq!(
            z_move.fixed_damage_fraction, fixed_damage_fraction,
            "failed for {pokemon_id:?} {move_id} {item:?}"
        );
        assert_eq!(
            z_move.terrain_effect, terrain_effect,
            "failed for {pokemon_id:?} {move_id} {item:?}"
        );
    }
}
