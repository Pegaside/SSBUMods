use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

// Game acmd script
unsafe extern "C" fn example_acmd_script(agent: &mut L2CAgentBase) {
    
}


//--------------------
// STATUS SCRIPTS
//--------------------

// STATUS Pre palutena_specials_status_pre
unsafe extern "C" fn palutena_specials_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_smash_dash_input =
        (fighter.global_table[0x22].get_i32() & *FIGHTER_PAD_CMD_CAT3_FLAG_SPECIAL_S_SMASH_DASH) != 0;

    if is_smash_dash_input {
        WorkModule::on_flag(
            fighter.module_accessor,
            *FIGHTER_PALUTENA_STATUS_SPECIAL_S_FLAG_SMASH,
        );
    } else {
        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_PALUTENA_STATUS_SPECIAL_S_FLAG_SMASH,
        );
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PALUTENA_SPECIAL_S_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PALUTENA_SPECIAL_S_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PALUTENA_SPECIAL_S_FLOAT,
        0,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
            | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0,
    );

    0.into()
}


pub fn install() {
    Agent::new("mario")
        .game_acmd("game_ATTACK_NAME_HERE", example_acmd_script, Default) // Game acmd script
        .on_line(Main, fighter_frame) // Char opff
        .status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, example_status_script) // Status script
        .install();
    Agent::new("fighter")
        .on_line(Main, fighter_frame) // Global opff
        .install();
}
