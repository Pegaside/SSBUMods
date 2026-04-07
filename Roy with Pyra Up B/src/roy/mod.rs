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

const FIGHTER_EFLAME_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_SPEED_Y: i32 = 0xdd9c; // DD9C is unlabeled in dict.txt, test by changing mod.rs of other mod and using raw hash like 0xdd9c instead of name and see if it still works.

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
unsafe extern "C" fn eflame_specialhi_status_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
unsafe extern "C" fn eflame_specialhi_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP {
        eflame_specialhi_substatus_end(fighter); // FUN_71000108a0
    }

    0.into()
}

// SUBSTATUS End eflame_specialhi_substatus_end
unsafe extern "C" fn eflame_specialhi_substatus_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionAnimcmdModule::call_script_single(
        fighter.module_accessor,
        *FIGHTER_ANIMCMD_EFFECT,
        Hash40::new_raw(0x19e219cd48),
        -1
    );

    MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);

    0.into()
}


// ----------
// SpecialHiEnd
// ----------

// STATUS Pre eflame_specialhiend_status_pre
unsafe extern "C" fn eflame_specialhiend_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        GROUND_CORRECT_KIND_GROUND.into(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI,
        0,
    );

    0.into()
}


// STATUS Main eflame_specialhiend_status_main
unsafe extern "C" fn eflame_specialhiend_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );

    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.sub_shift_status_main(L2CValue::Ptr(eflame_specialhiend_status_main_loop as *const () as _,
    ))
}


// STATUS MainLoop eflame_specialhiend_status_main_loop
unsafe extern "C" fn eflame_specialhiend_status_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool()
        {
            return 0.into();
        }
    }

    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    } else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }

    0.into()
}


// STATUS End eflame_specialhiend_status_end
unsafe extern "C" fn eflame_specialhiend_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xe].get_f32() == 1.0 {
        eflame_specialhi_substatus_end(fighter);
    }

    0.into()
}



// ----------
// SpecialHiJump
// ----------

// STATUS Pre eflame_specialhijump_status_pre
unsafe extern "C" fn eflame_specialhijump_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_AIR.into(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI.into(),
        0,
    );

    0.into()
}

// STATUS Main eflame_specialhijump_status_main
unsafe extern "C" fn eflame_specialhijump_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );

    KineticModule::change_kinetic(
        fighter.module_accessor,
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
    );

    let jump_speed_mul = 0.83;

    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, jump_speed_mul);
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(eflame_specialhijump_status_main_loop as *const () as _))
}

// STATUS MainLoop eflame_specialhijump_status_main_loop
unsafe extern "C" fn eflame_specialhijump_status_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(
            FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP.into(),
            false.into(),
        );
        return 0.into();
    }

    if MotionModule::frame(fighter.module_accessor).ceil() == MotionModule::end_frame(fighter.module_accessor) {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

        WorkModule::set_float(
            fighter.module_accessor,
            speed_y, // TODO: confirm the exact expression here; the decompile around `aLStack112 _ aLStack80` was garbled
            FIGHTER_EFLAME_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_SPEED_Y, // DAT_71004ebfcc
        );
    }

    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_ENABLE_GROUND,
        ) {
            WorkModule::off_flag(
                fighter.module_accessor,
                *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_ENABLE_GROUND,
            );

            fighter.change_status(
                FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END.into(),
                false.into(),
            );

            return 0.into();
        }
    }

    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_START_CONTROL,
    ) {
        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_START_CONTROL,
        );

        KineticModule::enable_energy(
            fighter.module_accessor,
            *FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        );

        fighter.clear_lua_stack();
        lua_args!(
            fighter,
            *FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        let jump_speed_x_mul = 0.8;

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, jump_speed_x_mul);
        sv_kinetic_energy::mul_x_speed_max(fighter.lua_state_agent);

        let jump_speed_x_max_mul = 1.2;

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, jump_speed_x_max_mul);
        sv_kinetic_energy::mul_x_accel_mul(fighter.lua_state_agent);
    }

    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_END_CONTROL,
    ) {
        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_END_CONTROL,
        );

        KineticModule::unable_energy(
            fighter.module_accessor,
            *FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        );
    }

    0.into()
}

// STATUS End eflame_specialhijump_status_end
unsafe extern "C" fn eflame_specialhijump_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP
        && fighter.global_table[0xb].get_i32() != *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END
    {
        eflame_specialhi_substatus_end(fighter);
    }

    0.into()
}



// ----------
// SpecialHiLoop
// ----------



// ----------
// Fire Pillar SpecialHi
// ----------

pub fn install() {
    Agent::new("roy")
        .game_acmd("game_ATTACK_NAME_HERE", example_acmd_script, Default) // Game acmd script
        .status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, example_status_script) // Status script
        .install();
}
