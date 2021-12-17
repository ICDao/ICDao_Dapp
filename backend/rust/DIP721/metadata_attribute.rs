use crate::rand::Rand;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Attribute {
    pub weapons: Vec<String>,
    pub chest: Vec<String>,
    pub head: Vec<String>,
    pub waist: Vec<String>,
    pub foot: Vec<String>,
    pub underwear: Vec<String>,
    pub accessory: Vec<String>,
    pub pants: Vec<String>,
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
    pub name_prefixes: Vec<String>,
    pub name_suffixes: Vec<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AttributeData {
    pub slot: String,
    pub name: String,
    pub prefix: String,
    pub name_prefix: String,
    pub name_suffix: String,
    pub special: bool,
}


impl Attribute {
    pub fn get_weapon(&self, token_id: u64) -> String {
        self.compute(&self.weapons, 1, token_id)
    }

    pub fn get_chest(&self, token_id: u64) -> String {
        self.compute(&self.chest, 222, token_id)
    }

    pub fn get_head(&self, token_id: u64) -> String {
        self.compute(&self.head, 333, token_id)
    }

    pub fn get_waist(&self, token_id: u64) -> String {
        self.compute(&self.waist, 4444, token_id)
    }

    pub fn get_foot(&self, token_id: u64) -> String {
        self.compute(&self.foot, 55555, token_id)
    }

    pub fn get_underwear(&self, token_id: u64) -> String {
        self.compute(&self.underwear, 666666, token_id)
    }

    pub fn get_accessory(&self, token_id: u64) -> String {
        self.compute(&self.accessory, 7777777, token_id)
    }

    pub fn get_pants(&self, token_id: u64) -> String {
        self.compute(&self.pants, 88888888, token_id)
    }

    pub fn get_weapon_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.weapons, 1, token_id, "weapon".to_string())
    }

    pub fn get_chest_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.chest, 222, token_id, "chest".to_string())
    }

    pub fn get_head_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.head, 333, token_id, "head".to_string())
    }

    pub fn get_waist_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.waist, 4444, token_id, "waist".to_string())
    }

    pub fn get_foot_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.foot, 55555, token_id, "foot".to_string())
    }

    pub fn get_underwear_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.underwear, 666666, token_id, "underwear".to_string())
    }

    pub fn get_accessory_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.accessory, 7777777, token_id, "accessory".to_string())
    }

    pub fn get_pants_attr(&self, token_id: u64) -> AttributeData {
        self.compute_attribute(&self.pants, 88888888, token_id, "pants".to_string())
    }

    pub fn get_prefix(&self, rand: u64) -> String {
        return self.prefixes[rand as usize % &self.prefixes.len()].clone();
    }

    pub fn get_name_prefix(&self, rand: u64) -> String {
        return self.name_prefixes[rand as usize % &self.name_prefixes.len()].clone();
    }

    pub fn get_name_suffix(&self, rand: u64) -> String {
        return self.name_suffixes[rand as usize % &self.name_suffixes.len()].clone();
    }

    pub fn compute(&self, items: &Vec<String>, offset: u64, token_id: u64) -> String {
        let rand = Rand::new(token_id + offset).rand();
        let item_index = rand as usize % items.len();

        let mut output = items[item_index].clone();

        let greatness = rand % 21;

        if greatness > 14 {
            output = format!("{} {}", self.get_prefix(rand), output);
        }
        if greatness > 19 {
            if greatness == 19 {
                output = format!(
                    "\"{}\" {} ({})",
                    self.get_name_prefix(rand),
                    self.get_name_suffix(rand),
                    output
                );
            } else {
                output = format!(
                    "\"{}\" {} ({}) 🔥",
                    self.get_name_prefix(rand),
                    output,
                    self.get_name_suffix(rand)
                );
            }
        }
        return output;
    }

    pub fn compute_attribute(
        &self,
        items: &Vec<String>,
        offset: u64,
        token_id: u64,
        kind: String,
    ) -> AttributeData {
        let rand = Rand::new(token_id + offset).rand();
        let item_index = rand as usize % items.len();

        let mut data = AttributeData::default();

        data.slot = kind.clone();
        data.name = items[item_index].clone();

        let greatness = rand % 21;

        if greatness > 14 {
            data.prefix = self.get_prefix(rand);
        }
        if greatness > 19 {
            if greatness == 19 {
                data.name_prefix = self.get_name_prefix(rand);
                data.name_suffix = self.get_name_suffix(rand);
                data.special = false;
            } else {
                data.special = true;
                data.name_prefix = self.get_name_prefix(rand);
                data.name_suffix = self.get_name_suffix(rand);
            }
        }
        return data;
    }


    pub fn get_properties(&self, token_id: u64) -> Vec<(String, String)> {
        return vec![
            ("hand".to_string(), self.get_weapon(token_id)),
            ("shirt".to_string(), self.get_chest(token_id)),
            ("head".to_string(), self.get_head(token_id)),
            ("waist".to_string(), self.get_waist(token_id)),
            ("foot".to_string(), self.get_foot(token_id)),
            ("pants".to_string(), self.get_pants(token_id)),
            ("underwear".to_string(), self.get_underwear(token_id)),
            ("accessory".to_string(), self.get_accessory(token_id)),
        ];
    }
    

    pub fn get_attribute_datas(&self, token_id: u64) -> Vec<AttributeData> {
        return vec![
            self.get_weapon_attr(token_id),
            self.get_chest_attr(token_id),
            self.get_head_attr(token_id),
            self.get_waist_attr(token_id),
            self.get_pants_attr(token_id),
            self.get_underwear_attr(token_id),
            self.get_accessory_attr(token_id),
            self.get_foot_attr(token_id),
        ];
    }

    pub fn generate(&self, token_id: u64) -> String {
        return format!(
            r#"
                <svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMinYMin meet" viewBox="0 0 420 420">
                    <style>
                    .base {{ fill: white; font-family: HelveticaNeue-Bold, Helvetica Neue; font-size: 14px; }} 
                    </style>
                    <rect width="100%" height="100%"/>
                    <text x="10" y="20" class="base">
                    {}
                    </text>
                    <text x="10" y="40" class="base">
                    {}
                    </text>
                    <text x="10" y="60" class="base">
                    {}
                    </text>
                    <text x="10" y="80" class="base">
                    {}
                    </text>
                    <text x="10" y="100" class="base">
                    {}
                    </text>
                    <text x="10" y="120" class="base">
                    {}
                    </text>
                    <text x="10" y="140" class="base">
                    {}
                    </text>
                    <text x="10" y="160" class="base">
                    {}
                    </text>
                </svg>
            "#,
            self.get_weapon(token_id),
            self.get_chest(token_id),
            self.get_head(token_id),
            self.get_waist(token_id),
            self.get_foot(token_id),
            self.get_underwear(token_id),
            self.get_accessory(token_id),
            self.get_pants(token_id),
        );
    }
}