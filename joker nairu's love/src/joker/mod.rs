use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*},
};

// Game ACMD Scripts
// ACMD Game Ground game_specialn_zelda_acmd
unsafe extern "C" fn game_specialn_zelda_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 160, 40, 0, 12, 8.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 160, 40, 0, 12, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 7.0, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 5.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
    }
}

// ACMD Effect Ground effect_specialn_zelda_acmd
unsafe extern "C" fn effect_specialn_zelda_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_flash"), Hash40::new("havel"), 0, 0, 0, -0.0, 0.0, 0.0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_flash"), Hash40::new("haver"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3.5, 13, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
	}
    else {
		if macros::is_excute(agent) {
			macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 3.5, 13, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			}
		}
	frame(agent.lua_state_agent, 8.0);
	if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_l"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
			println!("This effect works!");
		}
	}
	else {
			if macros::is_excute(agent) {
				macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_r"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
				println!("This effect works!");
			}
		}
	frame(agent.lua_state_agent, 9.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
	}
	frame(agent.lua_state_agent, 12.0);
	if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
		if macros::is_excute(agent) {
			macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}
	else {
		if macros::is_excute(agent) {
			macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}
	for _ in 0..6 {
		wait(agent.lua_state_agent, 2.0);
		if macros::is_excute(agent) {
			macros::COL_NORMAL(agent);
		}
	wait(agent.lua_state_agent, 2.0);
	}
}


// ACMD Sound Ground sound_specialn_zelda_acmd
unsafe extern "C" fn sound_specialn_zelda_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_jack_rnd_special_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_jack_special_n01")); // Replaced 86th audio file in nus3audio file with Zelda's
    }
}


// ACMD Game Air game_specialairn_zelda_acmd
unsafe extern "C" fn game_specialairn_zelda_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 160, 40, 0, 12, 8.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 160, 40, 0, 12, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 7.0, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 5.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
    }
}

// ACMD Effect Air effect_specialairn_zelda_acmd
unsafe extern "C" fn effect_specialairn_zelda_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_flash"), Hash40::new("havel"), 0, 0, 0, -0.0, 0.0, 0.0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_flash"), Hash40::new("haver"), 0, 0, 0, 0.0, 0.0, 0.0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3.5, 13, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
	else {
		if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 3.5, 13, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
	}
	frame(agent.lua_state_agent, 8.0);
	if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_l"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
			println!("This effect works!");
		}
	}
	else {
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_nayru_r"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
			println!("This effect works!");
			}
		}
	frame(agent.lua_state_agent, 12.0);
	for _ in 0..6 {
		wait(agent.lua_state_agent, 2.0);
		if macros::is_excute(agent) {
			macros::COL_NORMAL(agent);
		}
	wait(agent.lua_state_agent, 2.0);
	}
}

// ACMD Sound Air sound_specialairn_zelda_acmd
unsafe extern "C" fn sound_specialairn_zelda_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_jack_rnd_special_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_jack_special_n01")); // Replaced 86th audio file in nus3audio file with Zelda's
    }
}


// All Status Scripts
// STATUS Init zelda_specialn_status_init
unsafe extern "C" fn zelda_specialn_status_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// STATUS Pre zelda_specialn_status_pre
unsafe extern "C" fn zelda_specialn_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();

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
        0,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        FIGHTER_STATUS_ATTR_START_TURN.into(),
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N.into(),
        0,
    );

    return 0.into();
}

// STATUS Main zelda_specialn_status_main
unsafe extern "C" fn zelda_specialn_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // let stop_y = WorkModule::get_param_int(fighter.module_accessor,Hash40::new_raw("param_special_n"),10);
	// Explanation for above: first hash goes to param_special_n, and because it starts in "param", the next hash would be a key inside that param thing
	// inside the original character's vl.prc, and that second hash's value is 10. So, entire thing must be just 10. If it starts with param_, remember this.
	let stop_y = 10;

    WorkModule::set_int(
        fighter.module_accessor,
        stop_y,
        *FIGHTER_ZELDA_STATUS_SPECIAL_N_WORK_INT_STOP_Y,
    );

    if !StopModule::is_stop(fighter.module_accessor) {
        zelda_specialn_substatus_main(fighter, false.into());
    }

    fighter.global_table[0x15].assign(&L2CValue::Ptr(zelda_specialn_substatus_main as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(zelda_specialn_status_main_loop as *const () as _))
}

// SUBSTATUS Main FUN_7100013a90 zelda_specialn_substatus_main
unsafe extern "C" fn zelda_specialn_substatus_main(fighter: &mut L2CFighterCommon,param_1: L2CValue) -> L2CValue {
    let should_update = param_1.get_bool();
    let situation = fighter.global_table[0x16].get_i32();

    if !should_update {
        if situation != *SITUATION_KIND_GROUND {
            let stop_y = WorkModule::get_int(
                fighter.module_accessor,
                *FIGHTER_ZELDA_STATUS_SPECIAL_N_WORK_INT_STOP_Y
            );

            if stop_y <= 0 {
                KineticModule::enable_energy(
                    fighter.module_accessor,
                    *FIGHTER_KINETIC_ENERGY_ID_GRAVITY
                );
            } else {
                fighter.clear_lua_stack();
                lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                if speed_y < 0.0 {
                    KineticModule::unable_energy(
                        fighter.module_accessor,
                        *FIGHTER_KINETIC_ENERGY_ID_GRAVITY
                    );
                }
            }
        }
    } else {
        if situation != *SITUATION_KIND_GROUND {
            WorkModule::dec_int(
                fighter.module_accessor,
                *FIGHTER_ZELDA_STATUS_SPECIAL_N_WORK_INT_STOP_Y
            );

            let stick_x = ControlModule::get_stick_x(fighter.module_accessor);

            if stick_x.abs() > 1e-5 {
                // let accel_x = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_n"), 0.032);
				// Same thing like in line 218
				let accel_x = 0.032;

                fighter.clear_lua_stack();
                lua_args!(
                    fighter,
                    *FIGHTER_KINETIC_ENERGY_ID_STOP,
                    stick_x * accel_x,
                    0.0
                );
                sv_kinetic_energy::add_speed(fighter.lua_state_agent);
            }
        }
    }

    return 0.into();
}

// STATUS MainLoop zelda_specialn_status_main_loop
unsafe extern "C" fn zelda_specialn_status_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool()
        {
            return 1.into();
        }
    }

    let prev_situation = fighter.global_table[0x17].get_i32();
    let curr_situation = fighter.global_table[0x16].get_i32();

    let crossed_ground_air_boundary =
        (prev_situation == *SITUATION_KIND_GROUND && curr_situation == *SITUATION_KIND_AIR)
        || (prev_situation != *SITUATION_KIND_GROUND && curr_situation == *SITUATION_KIND_GROUND);

    if StatusModule::is_changing(fighter.module_accessor) || crossed_ground_air_boundary {
        if curr_situation != *SITUATION_KIND_GROUND {
            GroundModule::correct(
                fighter.module_accessor,
                smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
            );
            KineticModule::change_kinetic(
                fighter.module_accessor,
                *FIGHTER_KINETIC_TYPE_ZELDA_SPECIAL_N_AIR,
            );

            if !WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE,
            ) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_air_n"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false,
                );
                WorkModule::on_flag(
                    fighter.module_accessor,
                    *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE,
                );
            } else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("special_air_n"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false,
                );
            }
        } else {
            GroundModule::correct(
                fighter.module_accessor,
                smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK),
            );
            KineticModule::change_kinetic(
                fighter.module_accessor,
                *FIGHTER_KINETIC_TYPE_GROUND_STOP,
            );

            if !WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE,
            ) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_n"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false,
                );
                WorkModule::on_flag(
                    fighter.module_accessor,
                    *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE,
                );
            } else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("special_n"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false,
                );
            }
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let situation = fighter.global_table[0x16].get_i32();

        if situation == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else if situation == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }

    return 0.into();
}

// STATUS Exec zelda_specialn_status_exec
unsafe extern "C" fn zelda_specialn_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let reflector_kind = *FIGHTER_ZELDA_REFLECTOR_KIND_REFLECTOR;

    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START,
    ) {
        fighter.clear_lua_stack();
        lua_args!(
            fighter,
            *MA_MSC_SHIELD_SET_STATUS,
            *COLLISION_KIND_REFLECTOR,
            reflector_kind,
            *SHIELD_STATUS_NORMAL,
            *FIGHTER_REFLECTOR_GROUP_EXTEND
        );
        sv_module_access::shield(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);

        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START,
        );
    }

    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END,
    ) {
        fighter.clear_lua_stack();
        lua_args!(
            fighter,
            *MA_MSC_SHIELD_SET_STATUS,
            *COLLISION_KIND_REFLECTOR,
            reflector_kind,
            *SHIELD_STATUS_NONE,
            *FIGHTER_REFLECTOR_GROUP_EXTEND
        );
        sv_module_access::shield(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);

        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END,
        );
    }

    return 0.into();
}

// STATUS ExecStop zelda_specialn_status_exec_stop
unsafe extern "C" fn zelda_specialn_status_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// STATUS End zelda_specialn_status_end
unsafe extern "C" fn zelda_specialn_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// STATUS Exit zelda_specialn_status_exit
unsafe extern "C" fn zelda_specialn_status_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(
        fighter,
        *MA_MSC_SHIELD_SET_STATUS,
        *COLLISION_KIND_REFLECTOR,
        *FIGHTER_ZELDA_REFLECTOR_KIND_REFLECTOR,
        *SHIELD_STATUS_NORMAL,
        *FIGHTER_REFLECTOR_GROUP_EXTEND
    );
    sv_module_access::shield(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);

    return 0.into();
}

pub fn install() {
    Agent::new("jack")
        .game_acmd("game_specialn", game_specialn_zelda_acmd, Default)
        .game_acmd("game_specialairn", game_specialairn_zelda_acmd, Default)
        .effect_acmd("effect_specialn", effect_specialn_zelda_acmd, Default)
        .effect_acmd("effect_specialairn", effect_specialairn_zelda_acmd, Default)
		.sound_acmd("sound_specialn", sound_specialn_zelda_acmd, Default)
		.sound_acmd("sound_specialairn", sound_specialairn_zelda_acmd, Default)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_status_init)
		.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_status_pre)
		.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_status_main)
		.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_status_exec)
		.status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_status_exec_stop)
		.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_status_end)
		.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_status_exit)
        .install();
}
