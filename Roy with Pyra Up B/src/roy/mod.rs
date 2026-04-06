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

// Game ACMD Scripts
// ACMD Game Ground
unsafe extern "C" fn example_acmd_script(agent: &mut L2CAgentBase) {
    
}


// All Status Scripts

// ----------
// SpecialHi
// ----------

// STATUS Pre eflame_specialhi_status_pre
unsafe extern "C" fn eflame_specialhi_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI,
        0
    );

    0.into()
}

// STATUS Main eflame_specialhi_status_main
unsafe extern "C" fn eflame_specialhi_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(
        Hash40::new("special_hi_start").into(),
        Hash40::new("special_air_hi_start").into(),
        false.into()
    );

    fighter.sub_set_special_start_common_kinetic_setting(
        Hash40::new("param_special_hi").into()
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(eflame_specialhi_status_main_loop as *const () as _))
}


// STATUS MainLoop eflame_specialhi_status_main_loop
unsafe extern "C" fn eflame_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(
            Hash40::new("special_hi_start").into(),
            Hash40::new("special_air_hi_start").into(),
            true.into()
        );

        fighter.sub_exec_special_start_common_kinetic_setting(
            Hash40::new("param_special_hi").into()
        );

        0.into()
    } else {
        fighter.change_status(
            FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP.into(),
            false.into()
        );

        0.into()
    }
}


// STATUS End eflame_specialhi_status_end




// ----------
// SpecialHiEnd
// ----------



// ----------
// SpecialHiJump
// ----------



// ----------
// SpecialHiLoop
// ----------



// ----------
// Fire Pillar SpecialHi
// ----------

pub fn install() {
    Agent::new("mario")
        .game_acmd("game_ATTACK_NAME_HERE", example_acmd_script, Default) // Game acmd script
        .status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, example_status_script) // Status script
        .install();
}
