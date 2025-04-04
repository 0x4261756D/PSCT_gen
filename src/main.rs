use rand::prelude::SliceRandom;
use rand::Rng;

const PERCENTAGE_GENERATE_SENTENCE: f32 = 0.9;
const PERCENTAGE_GENERATE_CONDITION: f32 = 0.5;
const CONDITION_CASES: u8 = 7;
const PERCENTAGE_GENERATE_SUMMONING_CONDITION_LOCATION: f32 = 0.5;
const PERCENTAGE_GENERATE_SUMMONING_CONDITION_EXCEPTION: f32 = 0.5;
const PERCENTAGE_GENERATE_SUMMONING_CONDITION_EXCEPTION_FIRST: f32 = 0.5;
const MAT_LIM_CASES: u8 = 3;
const MAT_LIM_OTHERS_CASES: u8 = 3;
const PERCENTAGE_ANOTHER_SUMMONING_TYPE: f32 = 0.2;
const PERCENTAGE_GENERATE_ATTRIBUTE_MONSTER_LEVEL: f32 = 0.5;
const PERCENTAGE_GENERATE_ATTRIBUTE_ATTRIBUTE: f32 = 0.5;
const PERCENTAGE_GENERATE_ATTRIBUTE_TYPE: f32 = 0.5;
const PERCENTAGE_GENERATE_ATTRIBUTE_ARCHETYPE: f32 = 0.5;
const PERCENTAGE_ANOTHER_SUMMONING_MATERIAL_TYPE: f32 = 0.5;
const MAX_MAT_SUBSTITUTIONS: u8 = 5;
const MAX_SINGLE_FUSION_MAT: u8 = 5;
const PERCENTAGE_ANOTHER_FUSION_MATERIAL: f32 = 0.5;
const PERCENTAGE_GENERATE_ACTIVATION_CONDITION: f32 = 0.5;
const PERCENTAGE_GENERATE_COST_OR_TARGET: f32 = 0.5;
const PERCENTAGE_GENERATE_COST: f32 = 0.5;
const MAX_ATK: u16 = 80;
const PERCENTAGE_ALLOW_ADDITIONAL_FUSION_MAT: f32 = 0.3;
const PERCENTAGE_FUSION_MAT_NAME: f32 = 0.5;
const PERCENTAGE_SYNCHRO_MAT_NAME: f32 = 0.5;
const MAX_SINGLE_SYNCHRO_MAT: u8 = 5;
const PERCENTAGE_ALLOW_ADDITIONAL_SYNCHRO_MAT: f32 = 0.3;
const MAX_XYZ_MAT: u8 = 5;
const PERCENTAGE_ALLOW_ADDITIONAL_XYZ_MAT: f32 = 0.3;
const PERCENTAGE_ALLOW_ADDITIONAL_LINK_MAT: f32 = 0.3;
const PERCENTAGE_MISS_TIMING: f32 = 0.1;
const PERCENTAGE_SOFT_OPT: f32 = 0.4;
const PERCENTAGE_FURTHER_ACTIVATION_CONDITION: f32 = 0.7;
const PERCENTAGE_IS_QUICK: f32 = 0.2;
const PERCENTAGE_HARD_OPT: f32 = 0.2;
const PERCENTAGE_ACTIVATION_CONDITION_PLAYER: f32 = 0.3;
const PERCENTAGE_ACTIVATION_CONDITION_PLAYER_OPPONENT: f32 = 0.3;
const PERCENTAGE_ONCE_WHILE_FACEUP: f32 = 0.2;
const PERCENTAGE_ACTIVATION_CONDITION_PHASE: f32 = 0.3;
const PERCENTAGE_DAMAGE_TYPE_BATTLE_FURTHER: f32 = 0.3;
const PERCENTAGE_DAMAGE_TYPE_BATTLE_OPPONENTS: f32 = 0.5;
const PERCENTAGE_DAMAGE_TYPE_BATTLE_MONSTER: f32 = 0.5;
const PERCENTAGE_ACTIVATION_WHILE: f32 = 0.3;
const PERCENTAGE_ACTIVATION_CONDITION_CARD_SELF: f32 = 0.3;
const PERCENTAGE_COST_LP_HALF: f32 = 0.3;
const MAX_COST_LP: u16 = 80;
const PERCENTAGE_COST_DISCARD_ENTIRE: f32 = 0.3;
const MAX_COST_DISCARD: u8 = 6;
const MAX_RES_ADD_TO_HAND: u8 = 3;
const PERCENTAGE_RES_ADD_TO_HAND_ALLOW_UP_TO: f32 = 0.3;
const PERCENTAGE_RES_OPTIONAL: f32 = 0.3;
const PERCENTAGE_RES_SEND_ALLOW_UP_TO: f32 = 0.3;
const MAX_RES_SEND: u8 = 10;
const PERCENTAGE_MORE_LOCATIONS: f32 = 0.2;
const PERCENTAGE_MORE_SEND_LOCATIONS: f32 = 0.2;
const PERCENTAGE_IS_DISCARDED_REQ_OPP: f32 = 0.2;
const PERCENTAGE_IS_DISCARDED_SPECIFIES_GY: f32 = 0.4;
const PERCENTAGE_IS_DISCARDED_SPECIFIES_EFF: f32 = 0.3;
const PERCENTAGE_SPELL_HAS_ARCHETYPE: f32 = 0.5;
const PERCENTAGE_RESOLUTION_HAS_CONJUNCTION: f32 = 0.3;
const PERCENTAGE_TRAP_HAS_ARCHETYPE: f32 = 0.3;
const MAX_TARGETS: u8 = 5;
const PERCENTAGE_TARGET_NEED_TO_MEET_REQ_ON_RES: f32 = 0.5;
const PERCENTAGE_TARGET_DOES_MORE: f32 = 0.4;
const PERCENTAGE_ACTION_TRIGGERS_DIFFERENT_TIME: f32 = 0.2;
const PERCENTAGE_ACTION_SPELL_TRIGGER_HAS_FURTHER_SPECIFICATION: f32 = 0.2;
const MAX_CHAIN_LINK: u8 = 7;
const PERCENTAGE_SPELL_TRIGGER_SPECIFICATION_CHAIN_LINK_HAS_OR_HIGHER: f32 = 0.5;
const PERCENTAGE_MORE_ACTIVATION_LOCATIONS: f32 = 0.3;
const PERCENTAGE_SPELL_TRIGGER_SPECIFICATION_ADDED_NEEDS_CARD: f32 = 0.5;
const PERCENTAGE_PLAYER_ACTION_CONTROL_INVERTED: f32 = 0.5;
const PERCENTAGE_COST_DISCARD_SELF: f32 = 0.3;
const MAX_COST_SEND: u8 = 10;
const PERCENTAGE_COST_SEND_HAS_UP_TO: f32 = 0.3;
const PERCENTAGE_COST_SEND_INCLUDES_EXTRADECK: f32 = 0.5;
const MAX_RES_SPSUMMON: u8 = 5;
const PERCENTAGE_RES_SPSUMMON_ALLOW_UP_TO: f32 = 0.2;
const PERCENTAGE_RES_SPSUMMON_IGNORE_SUMMONING_CONDITIONS: f32 = 0.1;
const MAX_COST_BANISH: u8 = 20;
const PERCENTAGE_COST_BANISH_HAS_UP_TO: f32 = 0.2;
const PERCENTAGE_COST_SEND_IS_TOP_OF_DECK: f32 = 0.3;
const PERCENTAGE_COST_BANISH_IS_TOP_OF_DECK: f32 = 0.5;
const PERCENTAGE_ACTION_DESTROYED_DIFFERENT_TIME: f32 = 0.3;
const PERCENTAGE_ACTION_DESTROYED_HAS_FURTHER_SPECIFICATION: f32 = 0.3;
const PERCENTAGE_ACTION_DESTROYED_SPECIFICATION_NEEDS_TO_HIT_GY: f32 = 0.5;
const PERCENTAGE_RES_NEGATE_NEGATES_ACTIVATION: f32 = 0.3;

const SUMMONING_TYPES: [&str; 9] = [
	"Normal Summon",
	"Ritual Summon",
	"Set",
	"Special Summon",
	"Fusion Summon",
	"Xyz Summon",
	"Synchro Summon",
	"Pendulum Summon",
	"Link Summon",
];
const MAIN_SUMMONING_TYPES: [&str; 5] = [
	"Normal Summon",
	"Ritual Summon",
	"Set",
	"Special Summon",
	"Pendulum Summon",
];
const EXTRA_SUMMONING_TYPES: [&str; 4] = [
	"Fusion Summon",
	"Synchro Summon",
	"Xyz Summon",
	"Link Summon",
];
const EXTRA_MONSTER_TYPES: [&str; 4] = [
	"Fusion Monster",
	"Synchro Monster",
	"Xyz Monster",
	"Link Monster",
];
const MAIN_MONSTER_TYPES: [&str; 2] = ["Effect Monster", "monster"];
const MONSTER_TYPES_WITH_LEVELS: [&str; 3] = ["monster", "Fusion Monster", "Synchro Monster"];
const SUMMONING_MATERIAL_TYPES: [&str; 4] = ["Fusion", "Synchro", "Xyz", "Link"];
const LOCATION: [&str; 8] = [
	"hand",
	"GY",
	"Extra Deck",
	"Deck",
	"field",
	"Main Monster Zone",
	"Extra Monster Zone",
	"Spell/Trap Zone",
];
const IMPERATIVE_ACTIONS: [&str; 3] = ["banishing", "destroying", "excavating"];
const PEOPLE: [&str; 3] = ["you", "your opponent", "the controller of this card"];
const ATTRIBUTES: [&str; 6] = ["DARK", "LIGHT", "EARTH", "WIND", "WATER", "FIRE"];
const TYPES: [&str; 23] = [
	"Aqua",
	"Beast",
	"Beast-Warrior",
	"Cyberse",
	"Dinosaur",
	"Dragon",
	"Fairy",
	"Fiend",
	"Fish",
	"Insect",
	"Machine",
	"Plant",
	"Psychic",
	"Pyro",
	"Reptile",
	"Rock",
	"Sea Serpent",
	"Spellcaster",
	"Thunder",
	"Warrior",
	"Winged Beast",
	"Wyrm",
	"Zombie",
];
const MONSTER_TYPES: [&str; 6] = [
	"Fusion Monster",
	"Synchro Monster",
	"Xyz Monster",
	"Link Monster",
	"Effect Monster",
	"monster",
];
const CARD_TYPES: [&str; 15] = [
	"Fusion Monster",
	"Synchro Monster",
	"Xyz Monster",
	"Link Monster",
	"Effect Monster",
	"Field Spell",
	"Continuous Spell",
	"Quick-Play Spell",
	"Equip Spell",
	"Normal Spell",
	"Spell Card",
	"Continuous Trap",
	"Counter Trap",
	"Normal Trap",
	"Trap Card",
];
const PHASES: [&str; 7] = [
	"Draw Phase",
	"Standby Phase",
	"Main Phase",
	"Main Phase 1",
	"Main Phase 2",
	"Battle Phase",
	"End Phase",
];
const DAMAGE_TYPES: [&str; 3] = ["battle damage", "effect damage", "damage"];
const ADD_LOCATIONS: [&str; 2] = ["Deck", "GY"];
const SEND_LOCATIONS: [&str; 3] = ["Deck", "hand", "field"];
const SEND_LOCATIONS_EXTRA: [&str; 4] = ["Deck", "hand", "field", "Extra Deck"];
const SPELL_TYPES: [&str; 6] = [
	"Field Spell",
	"Continuous Spell",
	"Quick-Play Spell",
	"Equip Spell",
	"Normal Spell",
	"Spell Card",
];
const TRAP_TYPES: [&str; 4] = [
	"Continuous Trap",
	"Counter Trap",
	"Normal Trap",
	"Trap Card",
];
const CONJUNCTIONS: [&str; 5] = [
	" and",
	", and if you do,",
	", also",
	", then",
	", also, after that, ",
];
const ACTIVATION_LOCATIONS: [&str; 3] = ["hand", "Deck", "GY"];
const SPECIAL_SUMMON_LOCATIONS: [&str; 3] = ["hand", "Deck", "GY"];

#[derive(Debug)]
struct Card<'a> {
	name: String,
	card_type: &'a str,
	typ: Option<&'a str>,
	attribute: Option<&'a str>,
	level: Option<u8>,
	atk: Option<u16>,
	def: Option<u16>,
	text: String,
	link_arrows: Option<[bool; 8]>,
}

impl Card<'_> {
	pub fn new() -> Card<'static> {
		let mut rng = rand::thread_rng();
		let card_type = &Self::get_card_type();
		let atk = if card_type.contains("Monster") {
			Some(rng.gen_range(0..MAX_ATK) * 100)
		} else {
			None
		};
		let link_arrows = if *card_type == "Link Monster" {
			let mut arr = [false; 8];
			rng.fill(&mut arr[..]);
			Some(arr)
		} else {
			None
		};
		let def = if *card_type == "Link Monster" {
			link_arrows
				.unwrap()
				.iter()
				.map(|x| *x as u16)
				.reduce(|x, y| x + y)
		} else if card_type.contains("Monster") {
			Some(rng.gen_range(0..=MAX_ATK) * 100)
		} else {
			None
		};
		let attribute = if card_type.contains("Monster") {
			Some(Self::get_attribute())
		} else {
			None
		};
		let level = if card_type.contains("Monster") && *card_type != "Link Monster" {
			if *card_type == "Synchro Monster" {
				Some(rng.gen_range(2..12))
			} else {
				Some(rng.gen_range(1..12))
			}
		} else {
			None
		};
		let typ = if card_type.contains("Monster") {
			Some(Self::get_type())
		} else {
			None
		};
		let mut c = Card {
			name: Self::get_card_name(),
			text: String::from(""),
			typ,
			attribute,
			card_type,
			atk,
			def,
			level,
			link_arrows,
		};
		println!("{:?}", c);
		c.generate_effect();
		c
	}

	pub fn get_type() -> &'static str {
		TYPES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn get_attribute() -> &'static str {
		ATTRIBUTES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn generate_type(text: &mut String) {
		text.push_str(Self::get_type());
	}

	pub fn generate_attribute(text: &mut String) {
		text.push_str(Self::get_attribute());
	}

	pub fn generate_archetype(text: &mut String) {
		text.push_str(&Self::get_archetype());
	}

	pub fn get_card_type() -> &'static str {
		CARD_TYPES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn get_archetype() -> String {
		format!("\"{}\"", "Archetype")
	}

	pub fn get_card_name() -> String {
		format!("\"{}\"", "Card Name")
	}

	pub fn generate_card_type(text: &mut String) {
		text.push_str(Self::get_card_type());
	}

	pub fn generate_monster_attributes(
		text: &mut String,
		level_restriction: Option<u8>,
		needs_level: bool,
	) {
		let mut rng = rand::thread_rng();
		if needs_level && rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_MONSTER_LEVEL {
			text.push_str("level ");
			text.push_str(
				&rng.gen_range(1..=level_restriction.unwrap_or(12))
					.to_string(),
			);
			if level_restriction.is_none() {
				let higher_lower = rng.gen_range(0..3);
				if higher_lower == 0 {
					text.push_str(" or higher ");
				} else if higher_lower == 1 {
					text.push_str(" or lower ");
				} else {
					text.push(' ');
				}
			} else {
				text.push(' ');
			}
		}
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_ATTRIBUTE {
			Self::generate_attribute(text);
			text.push(' ');
		}
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_TYPE {
			Self::generate_type(text);
			text.push(' ');
		}
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_ARCHETYPE {
			Self::generate_archetype(text);
			text.push(' ');
		}
	}

	pub fn get_monster_type(
		summoning_restriction: Option<&str>,
		is_getting_summoned: bool,
		include_extradeck: bool,
	) -> &'static str {
		let _rng = rand::thread_rng();
		if let Some(restriction) = summoning_restriction {
			if is_getting_summoned {
				todo!("Restriction: {:?}", summoning_restriction)
			} else {
				match restriction {
					"Xyz Summon" | "Synchro Summon" => MONSTER_TYPES_WITH_LEVELS
						.choose(&mut rand::thread_rng())
						.unwrap(),
					"Link Summon" | "Fusion Summon" => {
						MONSTER_TYPES.choose(&mut rand::thread_rng()).unwrap()
					}

					_ => todo!("Restriction: {:?}", summoning_restriction),
				}
			}
		} else if include_extradeck {
			MONSTER_TYPES.choose(&mut rand::thread_rng()).unwrap()
		} else {
			MAIN_MONSTER_TYPES.choose(&mut rand::thread_rng()).unwrap()
		}
	}

	pub fn generate_trap_type(text: &mut String) {
		text.push_str(TRAP_TYPES.choose(&mut rand::thread_rng()).unwrap());
	}

	pub fn get_trap_type() -> &'static str {
		TRAP_TYPES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn generate_spell_type(text: &mut String) {
		text.push_str(Self::get_spell_type());
	}

	pub fn get_spell_type() -> &'static str {
		SPELL_TYPES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn generate_monster_type(
		text: &mut String,
		summoning_restriction: Option<&str>,
		is_getting_summoned: bool,
		include_extradeck: bool,
	) {
		text.push_str(Self::get_monster_type(
			summoning_restriction,
			is_getting_summoned,
			include_extradeck,
		));
	}

	pub fn generate_summoning_type(text: &mut String, maybe_card_type: Option<&str>) {
		match maybe_card_type {
			Some(card_type) => {
				if EXTRA_MONSTER_TYPES.contains(&card_type) {
					text.push_str(Self::get_extra_summoning_type());
				} else {
					text.push_str(Self::get_main_summoning_type());
				}
			}
			None => {
				text.push_str(Self::get_summoning_type());
			}
		}
	}

	pub fn get_location() -> &'static str {
		LOCATION.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn generate_location(text: &mut String) {
		text.push_str(Self::get_location());
	}

	pub fn generate_special_summon_location(text: &mut String) {
		text.push_str(
			SPECIAL_SUMMON_LOCATIONS
				.choose(&mut rand::thread_rng())
				.unwrap(),
		);
	}

	pub fn generate_conjunction(text: &mut String) {
		text.push_str(CONJUNCTIONS.choose(&mut rand::thread_rng()).unwrap());
	}

	pub fn generate_locations(text: &mut String) {
		let mut locations: Vec<&str> = Vec::new();
		locations.push(Self::get_location());
		let mut rng = rand::thread_rng();
		while locations.len() < LOCATION.len() && rng.gen::<f32>() < PERCENTAGE_MORE_LOCATIONS {
			locations.push(Self::get_location());
		}
		locations.sort();
		locations.dedup();
		if locations.len() == 1 {
			text.push_str(locations[0]);
		}
		if locations.len() > 1 {
			text.push_str(locations[..(locations.len() - 2)].join(", ").as_str());
			text.push_str("and/or");
			text.push_str(locations.last().unwrap());
		}
	}

	pub fn generate_imperative_action(text: &mut String) {
		text.push_str(IMPERATIVE_ACTIONS.choose(&mut rand::thread_rng()).unwrap());
	}

	pub fn generate_person(text: &mut String) {
		text.push_str(Self::get_person());
	}

	pub fn get_person() -> &'static str {
		PEOPLE.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn get_extra_summoning_type() -> &'static str {
		EXTRA_SUMMONING_TYPES
			.choose(&mut rand::thread_rng())
			.unwrap()
	}

	pub fn get_main_summoning_type() -> &'static str {
		MAIN_SUMMONING_TYPES
			.choose(&mut rand::thread_rng())
			.unwrap()
	}

	pub fn get_summoning_material_type() -> &'static str {
		SUMMONING_MATERIAL_TYPES
			.choose(&mut rand::thread_rng())
			.unwrap()
	}

	pub fn get_summoning_type() -> &'static str {
		SUMMONING_TYPES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn get_summoning_type_by_material(mat_type: &str) -> String {
		format!("{} Summon", mat_type)
	}

	pub fn generate_summoning_material_type(text: &mut String) {
		text.push_str(Self::get_summoning_material_type());
	}

	pub fn generate_summoning_material_types(text: &mut String) {
		let mut types: Vec<&str> = Vec::new();
		types.push(Self::get_summoning_material_type());
		let mut rng = rand::thread_rng();
		while rng.gen::<f32>() < PERCENTAGE_ANOTHER_SUMMONING_MATERIAL_TYPE {
			types.push(Self::get_summoning_material_type());
		}
		types.sort();
		types.dedup();
		text.push_str(&types.join(", "));
		text.push_str(" Material");
	}

	pub fn generate_extra_summoning_types(text: &mut String) {
		let mut types: Vec<&str> = Vec::new();
		types.push(Self::get_extra_summoning_type());
		let mut rng = rand::thread_rng();
		while rng.gen::<f32>() < PERCENTAGE_ANOTHER_SUMMONING_TYPE {
			types.push(Self::get_extra_summoning_type());
		}
		types.sort();
		types.dedup();
		text.push_str(&types.join(", "));
	}

	pub fn generate_fusion_material(text: &mut String) {
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < PERCENTAGE_FUSION_MAT_NAME {
			let amount = rng.gen_range(1..=MAX_SINGLE_FUSION_MAT);
			text.push_str(&amount.to_string());
			if rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_FUSION_MAT {
				text.push('+');
			}
			text.push(' ');
			Self::generate_monster_attributes(text, None, true);
			Self::generate_monster_type(text, None, true, true);
			if amount > 1 {
				text.push('s');
			}
		} else {
			text.push_str(&Self::get_card_name());
		}
	}

	pub fn generate_synchro_material(text: &mut String, is_tuner: bool, max: u8) -> u8 {
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < PERCENTAGE_SYNCHRO_MAT_NAME {
			let m = if max < MAX_SINGLE_SYNCHRO_MAT {
				max
			} else {
				MAX_SINGLE_SYNCHRO_MAT
			};
			let amount = rng.gen_range(1..=m);
			text.push_str(&amount.to_string());
			if rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_SYNCHRO_MAT {
				text.push('+');
			}
			if is_tuner {
				text.push_str(" Tuner");
			} else {
				text.push_str(" non-Tuner monster");
			}
			if amount > 1 {
				text.push('s');
			}
			amount
		} else {
			text.push_str(&Self::get_card_name());
			1
		}
	}

	pub fn generate_xyz_material(text: &mut String, rank: Option<u8>) {
		let mut rng = rand::thread_rng();
		let amount = rng.gen_range(1..=MAX_XYZ_MAT);
		text.push_str(&amount.to_string());
		if rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_XYZ_MAT {
			text.push('+');
		}
		text.push_str(" level ");
		text.push_str(&rank.unwrap().to_string());
		text.push(' ');
		Self::generate_monster_attributes(text, None, false);
		Self::generate_monster_type(text, None, true, true);
		if amount > 1 {
			text.push('s');
		}
	}

	pub fn generate_link_material(text: &mut String, rating: u16) {
		let mut rng = rand::thread_rng();
		let typ = Self::get_monster_type(None, true, true);
		let amount = rng.gen_range(1..=rating);
		text.push_str(&amount.to_string());
		if amount < rating && (typ != "Link Monster" || rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_LINK_MAT)
		{
			text.push('+');
		}
		text.push(' ');
		Self::generate_monster_attributes(text, None, true);
		text.push_str(typ);
		if amount > 1 {
			text.push('s');
		}
	}

	pub fn generate_spell_trigger_specification(text: &mut String) {
		let mut rng = rand::thread_rng();
		match rng.gen_range(0..5) {
			1 => {
				text.push_str("as Chain Link ");
				text.push_str(&(rng.gen_range(2..=MAX_CHAIN_LINK).to_string()));
				if rng.gen::<f32>()
					< PERCENTAGE_SPELL_TRIGGER_SPECIFICATION_CHAIN_LINK_HAS_OR_HIGHER
				{
					text.push_str(" or higher");
				}
			}
			2 => {
				text.push_str("from your ");
				Self::generate_activation_locations(text);
			}
			3 => {
				text.push_str("added from your ");
				Self::generate_add_location(text);
				text.push_str(" to your hand");
				if rng.gen::<f32>() < PERCENTAGE_SPELL_TRIGGER_SPECIFICATION_ADDED_NEEDS_CARD {
					text.push_str(" by a ");
					Self::generate_card_anywhere(text, false);
				}
			}
			_ => todo!("generate_spell_trigger_specification not fully implemented"),
		}
	}

	pub fn generate_materials(&mut self) {
		let mut rng = rand::thread_rng();
		match self.card_type {
			"Fusion Monster" => {
				Self::generate_fusion_material(&mut self.text);
				self.text.push_str(" + ");
				Self::generate_fusion_material(&mut self.text);
				while rng.gen::<f32>() < PERCENTAGE_ANOTHER_FUSION_MATERIAL {
					self.text.push_str(" + ");
					Self::generate_fusion_material(&mut self.text);
				}
			}
			"Synchro Monster" => {
				let count =
					Self::generate_synchro_material(&mut self.text, true, self.level.unwrap());
				self.text.push_str(" + ");
				Self::generate_synchro_material(&mut self.text, false, self.level.unwrap() - count);
			}
			"Xyz Monster" => {
				Self::generate_xyz_material(&mut self.text, self.level);
			}
			"Link Monster" => {
				Self::generate_link_material(&mut self.text, self.def.unwrap());
			}
			_ => panic!("We shouldn't be here"),
		}
		self.text.push('\n');
	}

	pub fn get_damage_type() -> &'static str {
		DAMAGE_TYPES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn generate_damage_type(text: &mut String) {
		let mut rng = rand::thread_rng();
		let damage_type = Self::get_damage_type();
		text.push_str(damage_type);
		if damage_type == "battle damage"
			&& rng.gen::<f32>() < PERCENTAGE_DAMAGE_TYPE_BATTLE_FURTHER
		{
			text.push_str(" from");
			if rng.gen::<f32>() < PERCENTAGE_DAMAGE_TYPE_BATTLE_OPPONENTS {
				text.push_str(" an opponent's");
			} else {
				text.push_str(" your");
			}
			if rng.gen::<f32>() < PERCENTAGE_DAMAGE_TYPE_BATTLE_MONSTER {
				text.push_str(" attacking monster");
			} else {
				text.push_str(" direct attack");
			}
		}
	}
	pub fn generate_monster_card_anywhere(text: &mut String, include_extradeck: bool) {
		Self::generate_monster_attributes(text, None, true);
		let typ = Self::get_monster_type(None, false, include_extradeck);
		text.push_str(typ);
		text.push_str(" that ");
		Self::generate_card_action(text, typ);
	}

	pub fn generate_card_anywhere(text: &mut String, include_extradeck: bool) {
		let mut rng = rand::thread_rng();
		let card_type = rng.gen_range(0..3);
		match card_type {
			0 => {
				Self::generate_monster_card_anywhere(text, include_extradeck);
			}
			1 => {
				if rng.gen::<f32>() < PERCENTAGE_SPELL_HAS_ARCHETYPE {
					Self::generate_archetype(text);
					text.push(' ');
				}
				let typ = Self::get_spell_type();
				text.push_str(typ);
				text.push_str(" that ");
				Self::generate_card_action(text, typ);
			}
			2 => {
				if rng.gen::<f32>() < PERCENTAGE_TRAP_HAS_ARCHETYPE {
					Self::generate_archetype(text);
					text.push(' ');
				}
				let typ = Self::get_trap_type();
				text.push_str(typ);
				text.push_str(" that ");
				Self::generate_card_action(text, typ);
			}
			_ => panic!("We shouldn't be here"),
		}
	}

	pub fn generate_destroyed_specification(text: &mut String, card_type: &str) {
		let mut rng = rand::thread_rng();
		match rng.gen_range(0..5) {
			1 => {
				text.push_str("by ");
				text.push_str(match rng.gen_range(0..4) {
					0 => "card effect",
					1 => "your opponent's card effect",
					2 => "your card effect",
					3 => {
						if MONSTER_TYPES.contains(&card_type) {
							"battle"
						} else {
							"card effect"
						}
					}
					_ => panic!("we shouldn't be here"),
				});
			}
			_ => {
				todo!("generate_destroyed_specification {}", text)
			}
		}
		if rng.gen::<f32>() < PERCENTAGE_ACTION_DESTROYED_SPECIFICATION_NEEDS_TO_HIT_GY {
			text.push_str(" and sent to the GY");
		}
	}

	pub fn generate_card_action(text: &mut String, card_type: &str) {
		let mut rng = rand::thread_rng();
		let case = rng.gen_range(0..5);
		match case {
			0 => {
				text.push_str("is discarded");
				let requires_opponent = rng.gen::<f32>() < PERCENTAGE_IS_DISCARDED_REQ_OPP;
				if requires_opponent {
					text.push_str(" from your hand ");
				}
				if rng.gen::<f32>() < PERCENTAGE_IS_DISCARDED_SPECIFIES_GY {
					text.push_str(" to the GY ");
				}
				if requires_opponent {
					text.push_str(" by an opponent's card effect ");
				} else if rng.gen::<f32>() < PERCENTAGE_IS_DISCARDED_SPECIFIES_EFF {
					text.push_str(" by card effect ");
				}
			}
			1 => {
				text.push_str("is sent from your ");
				if EXTRA_MONSTER_TYPES.contains(&card_type) {
					Self::generate_send_location_extra(text);
				} else {
					Self::generate_send_location(text);
				}
				text.push_str(" to the GY");
			}
			2 => {
				let different_time = rng.gen::<f32>() < PERCENTAGE_ACTION_TRIGGERS_DIFFERENT_TIME;
				if different_time {
					text.push_str("was ");
				} else {
					text.push_str("is ");
				}
				if MONSTER_TYPES.contains(&card_type) {
					Self::generate_summoning_type(text, Some(card_type));
					text.push_str("ed");
					todo!("Monster specifications {}", text);
				} else {
					text.push_str("activated");
					if rng.gen::<f32>() < PERCENTAGE_ACTION_SPELL_TRIGGER_HAS_FURTHER_SPECIFICATION
					{
						text.push(' ');
						Self::generate_spell_trigger_specification(text);
					}
				}
				if different_time {
					text.push_str(" this turn");
				}
			}
			3 => {
				text.push_str("is face-up on the field");
			}
			4 => {
				let different_time = rng.gen::<f32>() < PERCENTAGE_ACTION_DESTROYED_DIFFERENT_TIME;
				if different_time {
					text.push_str("was ");
				} else {
					text.push_str("is ");
				}
				text.push_str("destroyed");
				if rng.gen::<f32>() < PERCENTAGE_ACTION_DESTROYED_HAS_FURTHER_SPECIFICATION {
					text.push(' ');
					Self::generate_destroyed_specification(text, card_type);
				}
				if different_time {
					text.push_str(" this turn");
				}
			}
			_ => todo!("text so far: {}", text),
		}
	}

	pub fn generate_activation_condition_main(
		text: &mut String,
		card_type: &str,
		phase: Option<&str>,
	) {
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < PERCENTAGE_ACTIVATION_CONDITION_PLAYER {
			if rng.gen::<f32>() < PERCENTAGE_ACTIVATION_CONDITION_PLAYER_OPPONENT {
				text.push_str("your opponent ");
				Self::generate_player_action(text, true, phase);
			} else {
				text.push_str("you ");
				Self::generate_player_action(text, false, phase);
			}
		} else {
			if rng.gen::<f32>() < PERCENTAGE_ACTIVATION_CONDITION_CARD_SELF {
				text.push_str("this card ");
			} else {
				text.push_str("a ");
				Self::generate_card_anywhere(text, true);
				text.push(' ');
			}
			Self::generate_card_action(text, card_type);
		}
	}

	pub fn generate_player_action(text: &mut String, is_opponent: bool, phase: Option<&str>) {
		let mut rng = rand::thread_rng();
		match rng.gen_range(0..5) {
			0 => {
				text.push_str("take");
				if is_opponent {
					text.push('s');
				}
				text.push(' ');
				match phase {
					Some("Battle Phase") | None => {
						Self::generate_damage_type(text);
					}
					_ => {
						text.push_str("effect damage");
					}
				}
			}
			1 => {
				text.push_str("control");
				if is_opponent {
					text.push('s');
				}
				text.push(' ');
				if rng.gen::<f32>() < PERCENTAGE_PLAYER_ACTION_CONTROL_INVERTED {
					text.push_str("no ");
				}
				Self::generate_card_anywhere(text, true);
			}
			2 => {
				if is_opponent {
					text.push_str("has ");
				} else {
					text.push_str("have ");
				}
				if rng.gen::<f32>() < PERCENTAGE_PLAYER_ACTION_CONTROL_INVERTED {
					text.push_str("no ");
				}
				let card_type = Self::get_card_type();
				let loc = Self::get_location();
				if loc == "field" {
					text.push_str(" on your ");
				} else {
					text.push_str(" in your ");
				}
				if EXTRA_MONSTER_TYPES.contains(&card_type) && loc == "Deck" {
					text.push_str("Extra ");
				}
				text.push_str(loc);
			}
			3 => {
				text.push_str("draw");
				if is_opponent {
					text.push('s');
				}
				text.push_str(" a card ");
			}
			_ => todo!("text so far: {}", text),
		}
	}

	pub fn generate_phase(text: &mut String) {
		text.push_str(Self::get_phase());
	}

	pub fn get_phase() -> &'static str {
		PHASES.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn can_have_soft_opt(card_type: String) -> bool {
		card_type != "Quick-Play Spell"
			&& card_type != "Normal Spell"
			&& card_type != "Normal Trap"
	}

	pub fn generate_hand_card(text: &mut String) {
		todo!("text so far: {}", text);
	}

	pub fn generate_activation_location(text: &mut String) {
		text.push_str(Self::get_activation_location());
	}

	pub fn get_activation_location() -> &'static str {
		ACTIVATION_LOCATIONS
			.choose(&mut rand::thread_rng())
			.unwrap()
	}

	pub fn generate_add_location(text: &mut String) {
		text.push_str(ADD_LOCATIONS.choose(&mut rand::thread_rng()).unwrap());
	}

	pub fn get_send_location() -> &'static str {
		SEND_LOCATIONS.choose(&mut rand::thread_rng()).unwrap()
	}

	pub fn get_send_location_extra() -> &'static str {
		SEND_LOCATIONS_EXTRA
			.choose(&mut rand::thread_rng())
			.unwrap()
	}

	pub fn generate_send_location(text: &mut String) {
		text.push_str(Self::get_send_location());
	}

	pub fn generate_send_location_extra(text: &mut String) {
		text.push_str(Self::get_send_location_extra());
	}

	pub fn generate_send_locations(text: &mut String) {
		let mut locations: Vec<&str> = Vec::new();
		locations.push(Self::get_send_location());
		let mut rng = rand::thread_rng();
		while locations.len() < SEND_LOCATIONS.len()
			&& rng.gen::<f32>() < PERCENTAGE_MORE_SEND_LOCATIONS
		{
			locations.push(Self::get_send_location());
		}
		locations.sort();
		locations.dedup();
		text.push_str(locations.join(", ").as_str());
		if locations.len() > 1 {
			text.push_str(" and/or ");
			text.push_str(locations.last().unwrap());
		}
	}

	pub fn generate_activation_locations(text: &mut String) {
		let mut locations: Vec<&str> = Vec::new();
		locations.push(Self::get_activation_location());
		let mut rng = rand::thread_rng();
		while locations.len() < ACTIVATION_LOCATIONS.len()
			&& rng.gen::<f32>() < PERCENTAGE_MORE_ACTIVATION_LOCATIONS
		{
			locations.push(Self::get_activation_location());
		}
		locations.sort();
		locations.dedup();
		text.push_str(locations.join(", ").as_str());
		if locations.len() > 1 {
			text.push_str(" and/or ");
			text.push_str(locations.last().unwrap());
		}
	}

	pub fn generate_activation_condition(text: &mut String, card_type: &str) -> bool {
		let mut rng = rand::thread_rng();
		let mut has_soft_opt = false;
		let mut activation_phase = None;
		if Self::can_have_soft_opt(card_type.to_string()) && rng.gen::<f32>() < PERCENTAGE_SOFT_OPT
		{
			text.push_str("Once per turn");
			has_soft_opt = true;
		}
		if !has_soft_opt
			&& Self::can_have_soft_opt(card_type.to_string())
			&& rng.gen::<f32>() < PERCENTAGE_ONCE_WHILE_FACEUP
		{
			text.push_str("Once while face-up on the field");
			has_soft_opt = true;
		}
		if !has_soft_opt || rng.gen::<f32>() < PERCENTAGE_FURTHER_ACTIVATION_CONDITION {
			if rng.gen::<f32>() < PERCENTAGE_ACTIVATION_CONDITION_PHASE {
				if has_soft_opt {
					text.push_str(", during ");
				} else {
					text.push_str("During ");
				}
				let case = rng.gen_range(0..3);
				match case {
					0 => {
						text.push_str("your ");
					}
					1 => {
						text.push_str("your opponent's ");
					}
					2 => text.push_str("the "),
					_ => panic!("We shouldn't be here"),
				}
				let phase = Self::get_phase();
				activation_phase = Some(phase);
				text.push_str(phase);
				if rng.gen::<f32>() < PERCENTAGE_MISS_TIMING {
					text.push_str(", when ");
				} else {
					text.push_str(", if ");
				}
			} else if rng.gen::<f32>() < PERCENTAGE_MISS_TIMING {
				if has_soft_opt {
					text.push_str(", when ");
				} else {
					text.push_str("When ");
				}
			} else if has_soft_opt {
				text.push_str(", if ");
			} else {
				text.push_str("If ");
			}
			Self::generate_activation_condition_main(text, card_type, activation_phase);
			if rng.gen::<f32>() < PERCENTAGE_ACTIVATION_WHILE {
				text.push_str(", while ");
				Self::generate_activation_condition_main(text, card_type, activation_phase);
			}
		}
		if card_type.contains("Monster") && rng.gen::<f32>() < PERCENTAGE_IS_QUICK {
			text.push_str(" (Quick Effect)");
		}
		has_soft_opt
	}

	pub fn generate_cost(text: &mut String) -> Option<String> {
		let mut rng = rand::thread_rng();
		let case = rng.gen_range(0..5);
		match case {
			0 => {
				text.push_str("Pay ");
				if rng.gen::<f32>() < PERCENTAGE_COST_LP_HALF {
					text.push_str("half your");
				} else {
					text.push_str(&(rng.gen_range(1..MAX_COST_LP) * 100).to_string());
				}
				text.push_str(" LP");
				None
			}
			1 => {
				text.push_str("Discard ");
				if rng.gen::<f32>() < PERCENTAGE_COST_DISCARD_SELF {
					text.push_str("this card");
				} else if rng.gen::<f32>() < PERCENTAGE_COST_DISCARD_ENTIRE {
					text.push_str("your entire hand");
				} else {
					let amount = rng.gen_range(1..=MAX_COST_DISCARD);
					text.push_str(&amount.to_string());
					Self::generate_hand_card(text);
				}
				None
			}
			2 => {
				text.push_str("Send ");
				let amount = rng.gen_range(1..=MAX_COST_SEND);
				if rng.gen::<f32>() < PERCENTAGE_COST_SEND_IS_TOP_OF_DECK {
					text.push_str("the top ");
					text.push_str(&amount.to_string());
					text.push_str(" card");
					if amount > 1 {
						text.push('s');
					}
					text.push_str(" of your deck");
				} else {
					if amount > 1 && rng.gen::<f32>() < PERCENTAGE_COST_SEND_HAS_UP_TO {
						text.push_str("up to ");
					}
					text.push_str(&amount.to_string());
					text.push(' ');
					let include_extradeck =
						rng.gen::<f32>() < PERCENTAGE_COST_SEND_INCLUDES_EXTRADECK;
					Self::generate_card_anywhere(text, include_extradeck);
					if amount > 1 {
						text.push_str("s ");
					}
					text.push_str("from your ");
					if include_extradeck {
						Self::generate_send_location_extra(text);
					} else {
						Self::generate_send_locations(text);
					}
				}
				text.push_str(" to the GY");
				None
			}
			3 => {
				text.push_str("Banish ");
				let amount = rng.gen_range(1..=MAX_COST_BANISH);
				if rng.gen::<f32>() < PERCENTAGE_COST_BANISH_IS_TOP_OF_DECK {
					text.push_str("the top ");
					text.push_str(&amount.to_string());
					text.push_str(" card");
					if amount > 1 {
						text.push('s');
					}
					text.push_str(" of your deck");
				} else {
					if amount > 1 && rng.gen::<f32>() < PERCENTAGE_COST_BANISH_HAS_UP_TO {
						text.push_str("up to ");
					}
					text.push_str(&amount.to_string());
					text.push(' ');
					Self::generate_card_anywhere(text, false);
					if amount > 1 {
						text.push_str("s ");
					}
					text.push_str("from your ");
					Self::generate_send_locations(text);
				}
				None
			}
			_ => todo!("text so far: {}", text),
		}
	}

	pub fn generate_target(text: &mut String) -> Option<String> {
		let mut rng = rand::thread_rng();
		text.push_str("Target ");
		let amount = rng.gen_range(1..=MAX_TARGETS);
		text.push_str(&amount.to_string());
		text.push(' ');
		Self::generate_card_anywhere(text, false);
		if amount > 1 {
			text.push(' ');
		}
		let target = if amount > 1 {
			if rng.gen::<f32>() < PERCENTAGE_TARGET_NEED_TO_MEET_REQ_ON_RES {
				"those targets"
			} else {
				"them"
			}
		} else if rng.gen::<f32>() < PERCENTAGE_TARGET_NEED_TO_MEET_REQ_ON_RES {
			"that target"
		} else {
			"it"
		};
		if rng.gen::<f32>() < PERCENTAGE_TARGET_DOES_MORE {
			Self::generate_resolution(text, Some(target.to_string()), false);
		}
		Some(target.to_string())
	}

	pub fn generate_resolution(text: &mut String, referrer: Option<String>, sentence_start: bool) {
		let mut rng = rand::thread_rng();
		let mut start_of_sentence = sentence_start;
		if rng.gen::<f32>() < PERCENTAGE_RES_OPTIONAL {
			text.push_str("You can ");
			start_of_sentence = false;
		}
		let case = rng.gen_range(0..5);
		match case {
			0 => {
				if start_of_sentence {
					text.push_str("Negate ");
				} else {
					text.push_str("negate ");
				}
				if rng.gen::<f32>() < PERCENTAGE_RES_NEGATE_NEGATES_ACTIVATION {
					text.push_str("the activation ");
				} else if referrer.is_some() {
					text.push_str(referrer.as_ref().unwrap());
					text.push_str("'s effects");
				} else {
					todo!("text so far: {}", text);
				}
			}
			1 => {
				if start_of_sentence {
					text.push_str("Add ");
				} else {
					text.push_str("add ");
				}
				let amount = rng.gen_range(1..=MAX_RES_ADD_TO_HAND);
				if amount > 1 && rng.gen::<f32>() < PERCENTAGE_RES_ADD_TO_HAND_ALLOW_UP_TO {
					text.push_str("up to ");
				}
				text.push_str(&amount.to_string());
				text.push(' ');
				Self::generate_card_anywhere(text, false);
				if amount > 1 {
					text.push('s');
				}
				text.push_str(" from your ");
				Self::generate_add_location(text);
				text.push_str(" to your hand");
			}
			2 => {
				if start_of_sentence {
					text.push_str("Send ");
				} else {
					text.push_str("send ");
				}
				let amount = rng.gen_range(1..=MAX_RES_SEND);
				if amount > 1 && rng.gen::<f32>() < PERCENTAGE_RES_SEND_ALLOW_UP_TO {
					text.push_str("up to ");
				}
				text.push_str(&amount.to_string());
				Self::generate_card_anywhere(text, false);
				text.push_str(" from your ");
				Self::generate_send_locations(text);
				text.push_str(" to the GY");
			}
			3 => {
				text.push_str("Special Summon ");
				if let Some(referrer) = &referrer {
					text.push_str(referrer.as_ref());
					text.push(' ');
				} else {
					let amount = rng.gen_range(1..=MAX_RES_SPSUMMON);
					if amount > 1 && rng.gen::<f32>() < PERCENTAGE_RES_SPSUMMON_ALLOW_UP_TO {
						text.push_str("up to ");
					}
					text.push_str(&amount.to_string());
					text.push(' ');
					Self::generate_monster_card_anywhere(text, false);
					if amount > 1 {
						text.push('s');
					}
					text.push_str(" from your ");
					Self::generate_special_summon_location(text);
					if rng.gen::<f32>() < PERCENTAGE_RES_SPSUMMON_IGNORE_SUMMONING_CONDITIONS {
						text.push_str(" ignoring ");
						if amount > 1 {
							text.push_str("their ");
						} else {
							text.push_str("it's ");
						}
						text.push_str("Summoning conditions");
					}
				}
			}
			_ => todo!("text so far: {}", text),
		}
		if rng.gen::<f32>() < PERCENTAGE_RESOLUTION_HAS_CONJUNCTION {
			Self::generate_conjunction(text);
			text.push(' ');
			Self::generate_resolution(text, referrer, false);
		}
	}

	pub fn generate_sentence(&mut self, can_have_more_conditions: bool) -> bool {
		let mut rng = rand::thread_rng();
		if can_have_more_conditions && rng.gen::<f32>() < PERCENTAGE_GENERATE_CONDITION {
			let cond_case = rng.gen_range(0..CONDITION_CASES);
			println!("cond_case: {}", cond_case);
			match cond_case {
				0 => {
					if !self.card_type.contains("Monster") {
						return self.generate_sentence(can_have_more_conditions);
					}
					self.text.push_str("Cannot be ");
					let summoning_type = Self::get_summoning_type();
					self.text.push_str(summoning_type);
					self.text.push_str("ed");
					if !EXTRA_SUMMONING_TYPES.contains(&summoning_type)
						&& rng.gen::<f32>() < PERCENTAGE_GENERATE_SUMMONING_CONDITION_LOCATION
					{
						self.text.push_str(" from ");
						Self::generate_location(&mut self.text);
					}
					if rng.gen::<f32>() < PERCENTAGE_GENERATE_SUMMONING_CONDITION_EXCEPTION {
						self.text.push_str(". Must ");
						if rng.gen::<f32>()
							< PERCENTAGE_GENERATE_SUMMONING_CONDITION_EXCEPTION_FIRST
						{
							self.text.push_str("first ");
						}
						self.text.push_str("be ");
						self.text.push_str(summoning_type);
						self.text.push_str("ed by ");
						Self::generate_imperative_action(&mut self.text);
						self.text.push(' ');
						Self::generate_target(&mut self.text);
					}
				}
				1 => {
					let mat_lim_case = rng.gen_range(0..MAT_LIM_CASES);
					println!("mat_lim_case: {}", mat_lim_case);
					match mat_lim_case {
						0 => {
							let mat_lim_others_case = rng.gen_range(0..MAT_LIM_OTHERS_CASES);
							println!("mat_lim_others_case: {}", mat_lim_others_case);
							match mat_lim_others_case {
								0 => {
									let person = Self::get_person();
									self.text.push_str(person);
									self.text.push_str(" cannot ");
									Self::generate_extra_summoning_types(&mut self.text);
									self.text.push_str(" unless ");
									if person == "you" {
										self.text.push_str("you");
									} else {
										self.text.push_str("they");
									}
									self.text.push_str("use this card as material");
								}
								1 => {
									self.text.push_str("If this card is used as ");
									let summoning_material_type =
										Self::get_summoning_material_type();
									self.text.push_str(summoning_material_type);
									self.text.push_str(" Material, all other ");
									self.text.push_str(summoning_material_type);
									self.text.push_str(" Materials must be ");
									Self::generate_monster_card_anywhere(&mut self.text, true);
									self.text.push('s');
								}
								2 => {
									Self::generate_person(&mut self.text);
									self.text.push_str(" can only use ");
									Self::generate_monster_attributes(&mut self.text, None, true);
									Self::generate_monster_type(&mut self.text, None, false, true);
									self.text.push_str(" as ");
									Self::generate_summoning_material_type(&mut self.text);
									self.text.push_str(" Material");
								}
								_ => panic!("we should not be here"),
							}
						}
						1 => {
							self.text.push_str("Cannot be used as ");
							Self::generate_summoning_material_types(&mut self.text);
						}
						2 => {
							self.text.push_str("Cannot be used as ");
							let summoning_material_type = Self::get_summoning_material_type();
							self.text.push_str(summoning_material_type);
							self.text.push_str(" Material, except for the ");
							let summoning_type =
								&Self::get_summoning_type_by_material(summoning_material_type);
							self.text.push_str(summoning_type);
							self.text.push_str(" of a ");
							Self::generate_monster_attributes(&mut self.text, None, true);
							Self::generate_monster_type(
								&mut self.text,
								Some(summoning_type),
								false,
								true,
							);
						}
						_ => panic!("We should not be here"),
					}
				}
				2 => {
					self.text.push_str("For a ");
					let summoning_type = Self::get_extra_summoning_type();
					self.text.push_str(summoning_type);
					self.text.push_str(", you can ");
					match summoning_type {
						"Fusion Summon" => {
							self.text.push_str("substitute this card for any ");
							self.text
								.push_str(&rng.gen_range(1..=MAX_MAT_SUBSTITUTIONS).to_string());
						}
						_ => self.text.push_str("treat this card as a"),
					}
					self.text.push(' ');
					Self::generate_monster_attributes(&mut self.text, None, true);
					Self::generate_monster_type(&mut self.text, Some(summoning_type), false, true);
				}
				3 => {
					self.text.push_str("If ");
					Self::generate_activation_condition_main(&mut self.text, self.card_type, None);
					self.text.push_str(", you win the Duel");
					// Match winner cards are useless so they are excluded.
				}
				4 => {
					self.text.push_str(&format!(
						"(This card's name is always treated as {}.)\n",
						Self::get_card_name()
					));
					return true;
				}
				5 => {
					self.text.push_str(&format!(
						"(This card is always treated as an {} card.)\n",
						Self::get_archetype()
					));
					return true;
				}
				6 => {
					if self.card_type == "Link Monster" || !self.card_type.contains("Monster") {
						return self.generate_sentence(can_have_more_conditions);
					}
					self.text.push_str("(This card's original ");
					if self.card_type == "Xyz Monster" {
						self.text.push_str("Rank");
					} else {
						self.text.push_str("Level");
					}
					self.text.push_str(" is always treated as ");
					self.text.push_str(&rng.gen_range(0..13).to_string());
					self.text.push_str(".)");
					return true;
				}
				_ => panic!("We should not be here"),
			}
			self.text.push('.');
			return true;
		}
		println!("Generating normal effect");
		let mut has_soft_opt = false;
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ACTIVATION_CONDITION {
			has_soft_opt = Self::generate_activation_condition(&mut self.text, self.card_type);
			self.text.push_str(": ");
		}
		let mut is_targeting = false;
		let mut referrer = None;
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_COST_OR_TARGET {
			if rng.gen::<f32>() < PERCENTAGE_GENERATE_COST {
				referrer = Self::generate_cost(&mut self.text);
			} else {
				is_targeting = true;
				referrer = Self::generate_target(&mut self.text);
			}
			self.text.push_str("; ");
		}
		Self::generate_resolution(&mut self.text, referrer, true);
		if !has_soft_opt && rng.gen::<f32>() < PERCENTAGE_HARD_OPT {
			self.text
				.push_str(". You can only activate this effect of ");
			self.text.push_str(&self.name);
			self.text.push_str(" once per turn");
			// No twice/thrice here...
		}
		self.text.push('.');
		false
	}

	pub fn generate_effect(&mut self) {
		if EXTRA_MONSTER_TYPES.contains(&self.card_type) {
			self.generate_materials();
		}
		let mut can_have_more_conditions = self.generate_sentence(true);
		while rand::random::<f32>() < PERCENTAGE_GENERATE_SENTENCE {
			self.text.push(' ');
			can_have_more_conditions = self.generate_sentence(can_have_more_conditions);
		}
	}
}

pub fn main() {
	let c = Card::new();
	println!("{:?}", c);
}
