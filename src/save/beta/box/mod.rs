pub mod row_id;
pub mod slot_id;
pub mod matrix;
pub mod position;
pub mod attack_lists;
pub mod caught_ball;
pub mod gender;
pub mod iv_struct;
pub mod pokemon_classes;
pub mod pokemon_gender;
pub mod pokemon_id;
pub mod pokemon_info;
pub mod pp_moves_lists;
pub mod shiny_list;

#[derive(PartialEq, Clone, Debug)]
pub enum StorageType {
    PARTY,
    BOXES,
}