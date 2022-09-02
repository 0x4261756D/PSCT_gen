use rand;
use rand::Rng;
use rand::prelude::SliceRandom;

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

const SUMMONING_TYPES: [&str; 9] = ["Normal Summon", "Ritual Summon", "Set", "Special Summon", "Fusion Summon", "Xyz Summon", "Synchro Summon", "Pendulum Summon", "Link Summon"];
const EXTRA_SUMMONING_TYPES: [&str; 4] = ["Fusion Summon", "Synchro Summon", "Xyz Summon", "Link Summon"];
const EXTRA_MONSTER_TYPES: [&str; 4] = ["Fusion Monster", "Synchro Monster", "Xyz Monster", "Link Monster"];
const SUMMONING_MATERIAL_TYPES: [&str; 4] = ["Fusion", "Synchro", "Xyz", "Link"];
const LOCATION: [&str; 5] = ["hand", "GY", "Extra Deck", "Deck", "face-up Extra Deck"];
const IMPERATIVE_ACTIONS: [&str; 3] = ["banishing", "destroying", "excavating"];
const PEOPLE: [&str; 3] = ["you", "your opponent", "the controller of this card"];
const ATTRIBUTES: [&str; 6] = ["DARK", "LIGHT", "EARTH", "WIND", "WATER", "FIRE"];
const TYPES: [&str; 23] = ["Aqua", "Beast", "Beast-Warrior", "Cyberse", "Dinosaur", "Dragon", "Fairy", "Fiend", "Fish", "Insect", "Machine", "Plant", "Psychic", "Pyro", "Reptile", "Rock", "Sea Serpent", "Spellcaster", "Thunder", "Warrior", "Winged Beast", "Wyrm", "Zombie"];
const MONSTER_TYPES: [&str; 6] = ["Fusion Monster", "Synchro Monster", "Xyz Monster", "Link Monster", "Effect Monster", "monster"];
const CARD_TYPES: [&str; 13] = ["Fusion Monster", "Synchro Monster", "Xyz Monster", "Link Monster", "Effect Monster", "Field Spell", "Continuous Spell", "Quick-Play Spell", "Equip Spell", "Normal Spell", "Continuous Trap", "Counter Trap", "Normal Trap"];

#[derive(Debug)]
struct Card<'a>
{
	name: String,
	card_type: &'a str,
	typ: Option<&'a str>,
	attribute: Option<&'a str>,
	level: Option<u16>,
	atk: Option<u16>,
	def: Option<u16>,
	text: String,
	link_arrows: Option<[bool; 8]>,
}

impl Card<'_>
{
	pub fn new() -> Card<'static>
	{
		let mut rng = rand::thread_rng();
		let card_type = &Self::get_card_type();
		let atk = if card_type.contains("Monster")
		{
			Some(rng.gen_range(0..MAX_ATK) * 100)
		}
		else
		{
			None
		};
		let link_arrows = if card_type.to_string() == "Link Monster"
		{
			let mut arr = [false; 8];
			rng.fill(&mut arr[..]);
			Some(arr)
		}
		else
		{
			None
		};
		let def = if card_type.to_string() == "Link Monster"
		{
			link_arrows.unwrap().iter().map(|x| *x as u16).reduce(|x, y| x + y)
		}
		else if card_type.contains("Monster")
		{
			Some(rng.gen_range(0..=MAX_ATK) * 100)
		}
		else
		{
			None
		};
		let attribute = if card_type.contains("Monster")
		{
			Some(Self::get_attribute())
		}
		else
		{
			None
		};
		let level = if card_type.contains("Monster") && !(card_type.to_string() == "Link Monster")
		{
			Some(rng.gen_range(1..12))
		}
		else
		{
			None
		};
		let typ = if card_type.contains("Monster")
		{
			Some(Self::get_type())
		}
		else
		{
			None
		};
		let mut c = Card
		{
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
		return c;
	}

	pub fn get_type() -> &'static str
	{
		return TYPES.choose(&mut rand::thread_rng()).unwrap();
	}

	pub fn get_attribute() -> &'static str
	{
		return ATTRIBUTES.choose(&mut rand::thread_rng()).unwrap();
	}
	
	pub fn generate_type(text: &mut String)
	{
		text.push_str(Self::get_type());
	}

	pub fn generate_attribute(text: &mut String)
	{
		text.push_str(Self::get_attribute());
	}

	pub fn generate_archetype(text: &mut String)
	{
		text.push_str(&Self::get_archetype());
	}

	pub fn get_card_type() -> &'static str
	{
		return CARD_TYPES.choose(&mut rand::thread_rng()).unwrap();
	}

	pub fn get_archetype() -> String
	{
		return format!("\"{}\"", "Archetype");
	}

	pub fn get_card_name() -> String
	{
		return format!("\"{}\"", "Card Name");
	}

	pub fn generate_monster_attributes(text: &mut String, monster_restrictions: Option<&str>)
	{
		text.push(' ');
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_MONSTER_LEVEL
		{
			text.push_str("level ");
			text.push_str(&rng.gen_range(1..=12).to_string());
			let higher_lower = rng.gen_range(0..3);
			if higher_lower == 0
			{
				text.push_str(" or higher ");
			}
			else if higher_lower == 1
			{
				text.push_str(" or lower ");
			}
			else
			{
				text.push(' ');
			}
		}
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_ATTRIBUTE
		{
			Self::generate_attribute(text);
			text.push(' ');
		}
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_TYPE
		{
			Self::generate_type(text);
			text.push(' ');
		}
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ATTRIBUTE_ARCHETYPE
		{
			Self::generate_archetype(text);
			text.push(' ');
		}
		Self::generate_monster_type(text, monster_restrictions);
	}

	pub fn generate_monster_type(text: &mut String, monster_restriction: Option<&str>)
	{
		let rng = rand::thread_rng();
		if monster_restriction.is_none()
		{
			text.push_str(MONSTER_TYPES.choose(&mut rand::thread_rng()).unwrap());
		}
		else
		{
			todo!("Text so far: {}\nRestriction: {:?}", text, monster_restriction);
		}
	}

	pub fn generate_summoning_type(text: &mut String)
	{
		text.push_str(Self::get_summoning_type());
	}

	pub fn generate_location(text: &mut String)
	{
		text.push_str(LOCATION.choose(&mut rand::thread_rng()).unwrap());
	}

	pub fn generate_imperative_action(text: &mut String)
	{
		text.push_str(IMPERATIVE_ACTIONS.choose(&mut rand::thread_rng()).unwrap());
	}

	pub fn generate_person(text: &mut String)
	{
		text.push_str(PEOPLE.choose(&mut rand::thread_rng()).unwrap());
	}

	pub fn get_extra_summoning_type() -> &'static str
	{
		return EXTRA_SUMMONING_TYPES.choose(&mut rand::thread_rng()).unwrap();
	}

	pub fn get_summoning_material_type() -> &'static str
	{
		return SUMMONING_MATERIAL_TYPES.choose(&mut rand::thread_rng()).unwrap();
	}

	pub fn get_summoning_type() -> &'static str
	{
		return SUMMONING_TYPES.choose(&mut rand::thread_rng()).unwrap();
	}

	pub fn get_summoning_type_by_material(mat_type: &str) -> String
	{
		return format!("{} Summon", mat_type);
	}

	pub fn generate_summoning_material_type(text: &mut String)
	{
		text.push_str(Self::get_summoning_material_type());
	}

	pub fn generate_summoning_material_types(text: &mut String)
	{
		let mut types: Vec<&str> = Vec::new();
		types.push(Self::get_summoning_material_type());
		let mut rng = rand::thread_rng();
		while rng.gen::<f32>() < PERCENTAGE_ANOTHER_SUMMONING_MATERIAL_TYPE
		{
			types.push(Self::get_summoning_material_type());
		}
		types.sort();
		types.dedup();
		text.push_str(&types.join(", "));
		text.push_str(" Material");
	}

	pub fn generate_extra_summoning_types(text: &mut String)
	{
		let mut types: Vec<&str> = Vec::new();
		types.push(Self::get_extra_summoning_type());
		let mut rng = rand::thread_rng();
		while rng.gen::<f32>() < PERCENTAGE_ANOTHER_SUMMONING_TYPE
		{
			types.push(Self::get_extra_summoning_type());
		}
		types.sort();
		types.dedup();
		text.push_str(&types.join(", "));
	}

	pub fn generate_fusion_material(text: &mut String)
	{
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < PERCENTAGE_FUSION_MAT_NAME
		{
			let amount = rng.gen_range(1..=MAX_SINGLE_FUSION_MAT);
			text.push_str(&amount.to_string());
			if rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_FUSION_MAT
			{
				text.push_str("+");
			}
			Self::generate_monster_attributes(text, None);
			if amount > 1
			{
				text.push('s');
			}
		}
		else
		{
			text.push_str(&Self::get_card_name());
		}
	}

	pub fn generate_synchro_material(text: &mut String, is_tuner: bool)
	{
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < PERCENTAGE_SYNCHRO_MAT_NAME
		{
			let amount = rng.gen_range(1..=MAX_SINGLE_SYNCHRO_MAT);
			text.push_str(&amount.to_string());
			if rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_SYNCHRO_MAT
			{
				text.push_str("+");
			}
			if is_tuner
			{
				text.push_str(" Tuner");
			}
			else
			{
				text.push_str(" non-Tuner monsters");
			}
		}
		else
		{
			text.push_str(&Self::get_card_name());
		}
	}

	pub fn generate_xyz_material(text: &mut String)
	{
		let mut rng = rand::thread_rng();
		let amount = rng.gen_range(1..=MAX_XYZ_MAT);
		text.push_str(&amount.to_string());
		if rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_XYZ_MAT
		{
			text.push_str("+");
		}
		Self::generate_monster_attributes(text, None);
		if amount > 1
		{
			text.push('s');
		}
	}

	pub fn generate_link_material(text: &mut String, rating: u16)
	{
		let mut rng = rand::thread_rng();
		let amount = rng.gen_range(1..=rating);
		text.push_str(&amount.to_string());
		if amount < rating && rng.gen::<f32>() < PERCENTAGE_ALLOW_ADDITIONAL_LINK_MAT
		{
			text.push_str("+");
		}
		Self::generate_monster_attributes(text, None);
		if amount > 1
		{
			text.push('s');
		}
	}

	pub fn generate_materials(&mut self)
	{
		let mut rng = rand::thread_rng();
		match self.card_type
		{
			"Fusion Monster" =>
			{
				Self::generate_fusion_material(&mut self.text);
				while rng.gen::<f32>() < PERCENTAGE_ANOTHER_FUSION_MATERIAL
				{
					self.text.push_str(" + ");
					Self::generate_fusion_material(&mut self.text);
				}
			}
			"Synchro Monster" =>
			{
				Self::generate_synchro_material(&mut self.text, true);
				self.text.push_str(" + ");
				Self::generate_synchro_material(&mut self.text, false);
			}
			"Xyz Monster" =>
			{
				Self::generate_xyz_material(&mut self.text);
			}
			"Link Monster" =>
			{
				Self::generate_link_material(&mut self.text, self.def.unwrap());
			}
			_ => todo!("text so far: {}", self.text)
		}
		self.text.push('\n');
	}


	pub fn generate_activation_condition(text: &mut String)
	{
		todo!("text till here: {}", text);
	}

	pub fn generate_cost(text: &mut String)
	{
		todo!("text so far: {}", text);
	}

	pub fn generate_target(text: &mut String)
	{
		todo!("text so far: {}", text);
	}

	pub fn generate_sentence(&mut self)
	{
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_CONDITION
		{
			let cond_case = rng.gen_range(0..CONDITION_CASES);
			println!("cond_case: {}", cond_case);
			match cond_case
			{
				0 =>
				{
					if !self.card_type.contains("Monster")
					{
						self.generate_sentence();
						return;
					}
					self.text.push_str("Cannot be ");
					let summoning_type = Self::get_summoning_type();
					self.text.push_str(summoning_type);
					self.text.push_str("ed");
					if !EXTRA_SUMMONING_TYPES.contains(&summoning_type) && rng.gen::<f32>() < PERCENTAGE_GENERATE_SUMMONING_CONDITION_LOCATION
					{
						self.text.push_str(" from ");
						Self::generate_location(&mut self.text);
					}
					if rng.gen::<f32>() < PERCENTAGE_GENERATE_SUMMONING_CONDITION_EXCEPTION
					{
						self.text.push_str(". Must ");
						if rng.gen::<f32>() < PERCENTAGE_GENERATE_SUMMONING_CONDITION_EXCEPTION_FIRST
						{
							self.text.push_str("first ");
						}
						self.text.push_str("be ");
						Self::generate_summoning_type(&mut self.text);
						self.text.push_str("ed by ");
						Self::generate_imperative_action(&mut self.text);
						Self::generate_target(&mut self.text);
					}
				}
				1 =>
				{
					let mat_lim_case = rng.gen_range(0..MAT_LIM_CASES);
					println!("mat_lim_case: {}", mat_lim_case);
					match mat_lim_case
					{
						0 =>
						{
							let mat_lim_others_case = rng.gen_range(0..MAT_LIM_OTHERS_CASES);
							println!("mat_lim_others_case: {}", mat_lim_others_case);
							match mat_lim_others_case
							{
								0 =>
								{
									Self::generate_person(&mut self.text);
									self.text.push_str(" cannot ");
									Self::generate_extra_summoning_types(&mut self.text);
									self.text.push_str(" unless they use this card as material")
								}
								1 =>
								{
									self.text.push_str("If this card is used as ");
									let summoning_material_type = Self::get_summoning_material_type();
									self.text.push_str(summoning_material_type);
									self.text.push_str(", all other ");
									self.text.push_str(summoning_material_type);
									self.text.push_str("s must be ");
									Self::generate_monster_attributes(&mut self.text, None);
								}
								2 =>
								{
									Self::generate_person(&mut self.text);
									self.text.push_str(" can only use ");
									Self::generate_monster_attributes(&mut self.text, None);
									self.text.push_str(" as ");
									Self::generate_summoning_material_type(&mut self.text);
								}
								_ => panic!("we should not be here")
							}
						}
						1 =>
						{
							self.text.push_str("Cannot be used as ");
							Self::generate_summoning_material_types(&mut self.text);
						}
						2 =>
						{
							self.text.push_str("Cannot be used as ");
							let summoning_material_type = Self::get_summoning_material_type();
							self.text.push_str(summoning_material_type);
							self.text.push_str(", except for the ");
							let summoning_type = &Self::get_summoning_type_by_material(summoning_material_type);
							self.text.push_str(summoning_type);
							self.text.push_str(" of a");
							Self::generate_monster_attributes(&mut self.text, Some(summoning_type));
						}
						_ => panic!("We should not be here")
					}
				}
				2 =>
				{
					self.text.push_str("For a ");
					let summoning_type = Self::get_extra_summoning_type();
					self.text.push_str(summoning_type);
					self.text.push_str(", you can substitute this card for any ");
					self.text.push_str(&rng.gen_range(1..=MAX_MAT_SUBSTITUTIONS).to_string());
					Self::generate_monster_attributes(&mut self.text, Some(summoning_type));
				}
				3 =>
				{
					Self::generate_activation_condition(&mut self.text);
					self.text.push_str(", you win the Duel");
					// Match winner cards are useless so they are excluded.
				}
				4 =>
				{
					self.text.push_str(&format!("(This card's name is always treated as {}.)", Self::get_card_name()));
					return;
				}
				5 =>
				{
					self.text.push_str(&format!("(This card is always treated as an {} card.)", Self::get_archetype()));
					return;
				}
				6 =>
				{
					if self.card_type == "Link Monster" || !self.card_type.contains("Monster")
					{
						Self::generate_sentence(self);
						return;
					}
					self.text.push_str("(This card's original ");
					if self.card_type == "Xyz Monster"
					{
						self.text.push_str("Rank");
					}
					else
					{
						self.text.push_str("Level");
					}
					self.text.push_str(" is always treated as ");
					self.text.push_str(&rng.gen_range(0..13).to_string());
					self.text.push_str(".)");
					return;
				}
				_ => panic!("We should not be here")
			}
			self.text.push('.');
			return;
		}
		println!("Generating normal effect");
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_ACTIVATION_CONDITION
		{
			Self::generate_activation_condition(&mut self.text);
		}
		let mut is_targeting = false;
		if rng.gen::<f32>() < PERCENTAGE_GENERATE_COST_OR_TARGET
		{
			if rng.gen::<f32>() < PERCENTAGE_GENERATE_COST
			{
				Self::generate_cost(&mut self.text);
			}
			else
			{
				is_targeting = true;
				Self::generate_target(&mut self.text);
			}
		}
		todo!("text till here: {}", self.text);
		self.text.push('.');
	}

	pub fn generate_effect(&mut self)
	{
		if EXTRA_MONSTER_TYPES.contains(&self.card_type)
		{
			Self::generate_materials(self);
		}
		else
		{
			println!("{} is not in extra monster types", self.card_type);
		}
		Self::generate_sentence(self);
		while rand::random::<f32>() < PERCENTAGE_GENERATE_SENTENCE
		{
			self.text.push(' ');
			Self::generate_sentence(self);
		}
	}
}


pub fn main()
{
	let mut c = Card::new();
	println!("{:?}", c);
}
