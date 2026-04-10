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


// STATUS Init palutena_specials_status_init
unsafe extern "C" fn palutena_specials_status_init(fighter: &mut L2CFighterCommon,) -> L2CValue {
    // FUN_7100009630 original name
    palutena_specials_substatus(fighter, true);

    let mut gravity_speed_y = KineticModule::get_sum_speed_y(
        fighter.module_accessor,
        *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN,
    );

    let special_s_speed_y_mul = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("param_special_s"),        // 0xfea97fe73
        hash40("special_s_speed_y_mul"),  // 0x15931981cc
    );
    gravity_speed_y *= special_s_speed_y_mul;

    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        let special_s_speed_y_add = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("param_special_s"),        // 0xfea97fe73
            hash40("special_s_speed_y_add"),  // 0x15c701d38a
        );
        gravity_speed_y += special_s_speed_y_add;
    }

    fighter.clear_lua_stack();
    lua_args!(
        fighter,
        *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        *ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_speed_y);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    KineticModule::enable_energy(
        fighter.module_accessor,
        *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
    );

    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    }

    fighter.clear_lua_stack();
    lua_args!(
        fighter,
        *FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, 0.0);
    sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);

    KineticModule::unable_energy(
        fighter.module_accessor,
        *FIGHTER_KINETIC_ENERGY_ID_CONTROL,
    );

    fighter.clear_lua_stack();
    lua_args!(
        fighter,
        *FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0,
        0.0
    );
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    KineticModule::unable_energy(
        fighter.module_accessor,
        *FIGHTER_KINETIC_ENERGY_ID_MOTION,
    );

    0.into()
}

// STATUS Main palutena_specials_status_main
unsafe extern "C" fn palutena_specials_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[0x16].get_i32();

    if situation == *SITUATION_KIND_GROUND {
        GroundModule::correct(
            fighter.module_accessor,
            smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK),
        );
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false,
        );
    } else if situation == *SITUATION_KIND_AIR {
        GroundModule::correct(
            fighter.module_accessor,
            smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
        );
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false,
        );
    }

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x37b6ecdcec_u64));

    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_specials_status_main_loop as *const () as _))
}


// STATUS Main Loop palutena_specials_status_main_loop
unsafe extern "C" fn palutena_specials_status_main_loop(fighter: &mut L2CFighterCommon,) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }

        if fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }

    let situation = fighter.global_table[0x16].get_i32();
    let prev_situation = fighter.global_table[0x17].get_i32();

    let air_to_ground =
        situation == *SITUATION_KIND_GROUND && prev_situation == *SITUATION_KIND_AIR;
    let ground_to_air =
        situation == *SITUATION_KIND_AIR && prev_situation == *SITUATION_KIND_GROUND;

    if air_to_ground || ground_to_air {
        // FUN_7100009630 original name
        palutena_specials_substatus(fighter, false);

        if situation == *SITUATION_KIND_GROUND {
            GroundModule::correct(
                fighter.module_accessor,
                smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK),
            );
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false,
            );
        } else if situation == *SITUATION_KIND_AIR {
            GroundModule::correct(
                fighter.module_accessor,
                smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
            );
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false,
            );
        }
    }

    0.into()
}


// STATUS End palutena_specials_status_end
unsafe extern "C" fn palutena_specials_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_LANDING,
        );
    }

    0.into()
}

// STATUS FixCamera palutena_attackair_status_fix_cam
unsafe extern "C" fn palutena_attackair_status_fix_cam(fighter: &mut L2CFighterCommon) -> L2CValue {
    let _ = fighter;
    0.into()
}


// SUBSTATUS FUN_7100009630 palutena_specials_substatus
unsafe fn palutena_specials_substatus(fighter: &mut L2CFighterCommon,is_status_init: bool,) {
    let mut stop_speed_x = KineticModule::get_sum_speed_x(
        fighter.module_accessor,
        *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN,
    );

    if is_status_init {
        let special_s_speed_x_mul = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("param_special_s"),        // 0xfea97fe73
            hash40("special_s_speed_x_mul"),  // 0x15ae79a87c
        );
        stop_speed_x *= special_s_speed_x_mul;
    }

    let mut stop_reset_type = *ENERGY_STOP_RESET_TYPE_GROUND;
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        stop_reset_type = *ENERGY_STOP_RESET_TYPE_AIR;
    }

    fighter.clear_lua_stack();
    lua_args!(
        fighter,
        *FIGHTER_KINETIC_ENERGY_ID_STOP,
        stop_reset_type,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(
        fighter,
        *FIGHTER_KINETIC_ENERGY_ID_STOP,
        stop_speed_x,
        0.0
    );
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    KineticModule::enable_energy(
        fighter.module_accessor,
        *FIGHTER_KINETIC_ENERGY_ID_STOP,
    );

    if !is_status_init {
        fighter.clear_lua_stack();
        lua_args!(
            fighter,
            *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            *ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        KineticModule::enable_energy(
            fighter.module_accessor,
            *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        );

        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        }
    }
}


//----------
// Explosive Flame Check
//----------

// STATUS Pre palutena_explosiveflame_check_status_pre
unsafe extern "C" fn palutena_explosiveflame_check_status_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        GROUND_CORRECT_KIND_AIR.into(),
        smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );

    0.into()
}


// STATUS Main palutena_explosiveflame_check_status_main
unsafe extern "C" fn palutena_explosiveflame_check_status_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new_raw(0x53c8eac13),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );

    if GroundModule::is_touch(
        weapon.module_accessor,
        (
            *GROUND_TOUCH_FLAG_LEFT
            | *GROUND_TOUCH_FLAG_UP
            | *GROUND_TOUCH_FLAG_RIGHT
            | *GROUND_TOUCH_FLAG_UP_LEFT
            | *GROUND_TOUCH_FLAG_UP_RIGHT
        ) as u32,
    ) {
        WorkModule::on_flag(
            weapon.module_accessor,
            *WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS,
        );
    }

    if !WorkModule::is_flag(
        weapon.module_accessor,
        *WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS,
    ) {
        let weapon_ptr = weapon.global_table[0x4].get_ptr() as *mut Weapon;

        if app::WeaponSpecializer_PalutenaExplosiveflame::is_touch_down(weapon_ptr) {
            WorkModule::on_flag(
                weapon.module_accessor,
                *WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS,
            );
        }
    }

    weapon.shift(L2CValue::Ptr(palutena_explosiveflame_check_status_main_loop as *const () as _,));

    0.into()
}

// STATUS End palutena_explosiveflame_check_status_end
unsafe extern "C" fn palutena_explosiveflame_check_status_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}


//----------
// Explosive Flame Explode
//----------

// STATUS Pre palutena_explosiveflame_explode_status_pre
unsafe extern "C" fn palutena_explosiveflame_explode_status_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        GROUND_CORRECT_KIND_AIR.into(),
        smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );

    0.into()
}


// STATUS Main palutena_explosiveflame_explode_status_main
unsafe extern "C" fn palutena_explosiveflame_explode_status_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new_raw(0x754732c8d),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );

    if !StopModule::is_stop(weapon.module_accessor) {
        palutena_explosiveflame_explode_substatus(weapon);
    }

    weapon.global_table[0x14].assign(&L2CValue::Ptr(palutena_explosiveflame_explode_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(palutena_explosiveflame_explode_status_main_loop as *const () as _))
}


// STATUS MainLoop palutena_explosiveflame_explode_status_main_loop
unsafe extern "C" fn palutena_explosiveflame_explode_status_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}


// STATUS End palutena_explosiveflame_explode_status_end
unsafe extern "C" fn palutena_explosiveflame_explode_status_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}


// SUBSTATUS FUN_710001a0b0 palutena_explosiveflame_explode_substatus
unsafe extern "C" fn palutena_explosiveflame_explode_substatus(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(
        weapon.module_accessor,
        *WEAPON_INSTANCE_WORK_ID_INT_LIFE,
    );

    let life = WorkModule::get_int(
        weapon.module_accessor,
        *WEAPON_INSTANCE_WORK_ID_INT_LIFE,
    );

    if life <= 0 {
        weapon.clear_lua_stack();
        lua_args!(
            weapon,
            Hash40::new_raw(0x199c462b5d)
        );
        sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
        weapon.pop_lua_stack(1);
    }

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
