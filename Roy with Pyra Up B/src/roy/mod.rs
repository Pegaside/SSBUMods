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

pub const FIGHTER_EFLAME_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_SPEED_Y: i32 = 0xdd9c; // DD9C is unlabeled in dict.txt, test by changing mod.rs of other mod and using raw hash like 0xdd9c instead of name and see if it still works.
pub const FIGHTER_ROY_GENERATE_ARTICLE_PREVOLT: i32 = 0x2;

// Game ACMD Scripts
// ACMD Game Ground Base game_specialhi_roy_acmd
unsafe extern "C" fn game_specialhi_roy_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_hi") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 55, 100, 0, 60, 4.5, 0.0, 4.0, 2.0, Some(0.0), Some(4.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 70, 100, 0, 65, 5.0, 0.0, -4.0, 6.0, Some(0.0), Some(-8.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		println!("HIT!!!");
	}
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 100, 0, 96, 11.0, 15.0, 7.0, 2.0, Some(15.0), Some(7.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 60, 100, 0, 96, 11.0, 15.0, 24.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 11.0, 15.0, 7.0, 2.0, Some(15.0), Some(7.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 11.0, 15.0, 24.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 60, 100, 0, 80, 7.0, 15.0, 39.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

// ACMD Effect Ground Base effect_specialhi_roy_acmd
unsafe extern "C" fn effect_specialhi_roy_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_sword_firetrail"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_speed_line"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_sword_fire"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_sword_fire2"), false, true);
        macros::AFTER_IMAGE_OFF(agent, 3);
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_sword_stab"), Hash40::new("top"), 0, 0, 5.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
	frame(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar_impact"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
	}
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_sword_smoke"), Hash40::new("top"), 0, 0, 5.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_promrevolt_sword_fire"), Hash40::new("sword1"), 2, 0, 0, 0, 0, 0, 0.7, true);
    }
}


// ACMD Effect Ground Interrupt effect_specialhiinterrupt_roy_acmd
unsafe extern "C" fn effect_specialhiinterrupt_roy_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_sword_firetrail"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_speed_line"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_sword_fire"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_promrevolt_sword_fire2"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_firetrail"), true, true);
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

 
// ACMD Game Ground Start game_specialhistart_roy_acmd
unsafe extern "C" fn game_specialhistart_roy_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_hi_jump") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
}

// ACMD Effect Ground Start effect_specialhistart_roy_acmd
unsafe extern "C" fn effect_specialhistart_roy_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_flash"), Hash40::new("sword1"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword1"), Hash40::new("tex_eflame_sword2"), 6, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 18.5, 0, -0.25, false, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
}

// ACMD Game Air Start game_specialairhistart_roy_acmd
unsafe extern "C" fn game_specialairhistart_roy_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_hi_jump") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
}

// ACMD Effect Air Start effect_specialairhistart_roy_acmd
unsafe extern "C" fn effect_specialairhistart_roy_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_flash"), Hash40::new("sword1"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword1"), Hash40::new("tex_eflame_sword2"), 6, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 18.5, 0, -0.25, false, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_fire2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_sword_firetrail_end"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
}


// ACMD Game Air Jump game_specialairhijump_roy_acmd
unsafe extern "C" fn game_specialairhijump_roy_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 85, 100, 64, 16, 2.5, 0.0, 18.0, 5.0, Some(0.0), Some(2.5), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 95, 100, 64, 16, 4.0, 0.0, 15.0, 11.0, Some(0.0), Some(4.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 64, 16, 4.0, 0.0, 14.0, 15.0, Some(0.0), Some(4.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 95, 100, 53, 20, 2.5, 0.0, 21.5, 5.0, Some(0.0), Some(15.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 100, 100, 53, 20, 4.0, 0.0, 20.0, 11.0, Some(0.0), Some(14.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 53, 20, 4.0, 0.0, 20.0, 15.0, Some(0.0), Some(14.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_START_CONTROL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 95, 100, 50, 20, 2.5, 0.0, 23.0, 5.0, Some(0.0), Some(19.0), Some(5.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 100, 100, 50, 20, 4.0, 0.0, 24.0, 11.0, Some(0.0), Some(20.0), Some(11.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 100, 100, 50, 20, 4.0, 0.0, 24.0, 15.0, Some(0.0), Some(20.0), Some(15.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 27.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_hi_jump") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 32.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: -7.0, z: 0.0});
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_END_CONTROL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 280, 130, 39, 0, 6.0, 0.0, 3.0, 9.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_ENABLE_GROUND);
    }
}

// ACMD Effect Air Jump effect_specialairhijump_roy_acmd
unsafe extern "C" fn effect_specialairhijump_roy_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_close"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_beam_m"), true, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_fire2"), false, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_flash"), Hash40::new("sword1"), 12, 0, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_eflame_sword1"), Hash40::new("tex_eflame_sword2"), 5, Hash40::new("sword1"), 0.3, 0, 0, Hash40::new("sword1"), 18.5, 0, -0.25, false, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_promrevolt_sword_firetrail"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_promrevolt_sword_fire"), Hash40::new("sword1"), 0, 0, 2, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("eflame_promrevolt_sword_fire2"), Hash40::new("sword1"), 0, 0, 18, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_promrevolt_speed_line"), Hash40::new("rot"), 0, 0, 1.5, 90, 0, 0, 1.5, false);
    }
}

// ACMD Game Air Fall game_specialairhifall_roy_acmd
unsafe extern "C" fn game_specialairhifall_roy_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_hi_fall") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

// ACMD Effect Air Fall effect_specialairhifall_roy_acmd
unsafe extern "C" fn effect_specialairhifall_roy_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
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
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
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
unsafe extern "C" fn eflame_specialhiend_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

// STATUS Pre eflame_specialhiloop_status_pre
unsafe extern "C" fn eflame_specialhiloop_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        *FS_SUCCEEDS_KEEP_ATTACK,
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

// STATUS Main eflame_specialhiloop_status_main
unsafe extern "C" fn eflame_specialhiloop_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_fall"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );

    let mut speed_y = WorkModule::get_float(
        fighter.module_accessor,
        FIGHTER_EFLAME_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_SPEED_Y,
    );

    if speed_y == 0.0 {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    }

    KineticModule::change_kinetic(
        fighter.module_accessor,
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
    );

    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y.abs());
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y.abs());
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    KineticModule::unable_energy(
        fighter.module_accessor,
        *FIGHTER_KINETIC_ENERGY_ID_CONTROL,
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(eflame_specialhiloop_status_main_loop as *const () as _,))
}

// STATUS MainLoop eflame_specialhiloop_status_main_loop
unsafe extern "C" fn eflame_specialhiloop_status_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(
                FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END.into(),
                false.into(),
            );
        }
    }

    0.into()
}

// STATUS End eflame_specialhiloop_status_end
unsafe extern "C" fn eflame_specialhiloop_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END {
        eflame_specialhi_substatus_end(fighter);
    }

    0.into()
}

/* ACMD PRevolt
unsafe extern "C" fn game_specialhi_prevolt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 60, 100, 0, 96, 11.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 60, 100, 0, 96, 11.0, 0.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 11.0, 0.0, 8.0, 2.0, Some(0.0), Some(8.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 60, 100, 0, 90, 11.0, 0.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 60, 100, 0, 80, 7.0, 0.0, 40.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

// ACMD PRevolt Effect
unsafe extern "C" fn effect_specialhi_prevolt_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT(agent, Hash40::new("eflame_promrevolt_firepillar_impact"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
}



// ----------
// Fire Pillar SpecialHi
// ----------

// STATUS Pre eflamefirepillar_specialhi_status_pre
unsafe extern "C" fn eflamefirepillar_specialhi_status_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        GROUND_CORRECT_KIND_NONE.into(),
        smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );

    GroundModule::set_test_coll_stop(weapon.module_accessor, false);

    0.into()
}

// STATUS Main eflamefirepillar_specialhi_status_main
unsafe extern "C" fn eflamefirepillar_specialhi_status_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("special_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );

    weapon.fastshift(L2CValue::Ptr(eflamefirepillar_specialhi_status_main_loop as *const () as _))
}

// STATUS MainLoop eflamefirepillar_specialhi_status_main_loop
unsafe extern "C" fn eflamefirepillar_specialhi_status_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

// STATUS End eflamefirepillar_specialhi_status_end
unsafe extern "C" fn eflamefirepillar_specialhi_status_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}
*/


pub fn install() {
    Agent::new("roy")
        .game_acmd("game_specialhi", game_specialhi_roy_acmd, Default)
		.game_acmd("game_specialhistart", game_specialhistart_roy_acmd, Default)
		.game_acmd("game_specialairhistart", game_specialairhistart_roy_acmd, Default)
		.game_acmd("game_specialairhijump", game_specialairhijump_roy_acmd, Default)
		.game_acmd("game_specialairhifall", game_specialairhifall_roy_acmd, Default)
		.effect_acmd("effect_specialhi", effect_specialhi_roy_acmd, Default)
		.effect_acmd("effect_specialhiinterrupt", effect_specialhiinterrupt_roy_acmd, Default)
		.effect_acmd("effect_specialhistart", effect_specialhistart_roy_acmd, Default)
		.effect_acmd("effect_specialairhistart", effect_specialairhistart_roy_acmd, Default)
		.effect_acmd("effect_specialairhijump", effect_specialairhijump_roy_acmd, Default)
		.effect_acmd("effect_specialairhifall", effect_specialairhifall_roy_acmd, Default)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, eflame_specialhi_status_pre)
		.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, eflame_specialhi_status_main)
		.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, eflame_specialhi_status_end)
		.status(Pre, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END, eflame_specialhiend_status_pre)
		.status(Main, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END, eflame_specialhiend_status_main)
		.status(End, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END, eflame_specialhiend_status_end)
		.status(Pre, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP, eflame_specialhijump_status_pre)
		.status(Main, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP, eflame_specialhijump_status_main)
		.status(End, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP, eflame_specialhijump_status_end)
		.status(Pre, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP, eflame_specialhiloop_status_pre)
		.status(Main, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP, eflame_specialhiloop_status_main)
		.status(End, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP, eflame_specialhiloop_status_end)
		.install();
	/*Agent::new("roy_prevolt")
		.game_acmd("game_specialhi", game_specialhi_prevolt_acmd, Default)
		.effect_acmd("effect_specialhi", effect_specialhi_prevolt_acmd, Default)
		//.status(Pre, *WEAPON_EFLAME_FIREPILLAR_STATUS_KIND_SPECIAL_HI, eflamefirepillar_specialhi_status_pre)
		//.status(Main, *WEAPON_EFLAME_FIREPILLAR_STATUS_KIND_SPECIAL_HI, eflamefirepillar_specialhi_status_main)
		//.status(End, *WEAPON_EFLAME_FIREPILLAR_STATUS_KIND_SPECIAL_HI, eflamefirepillar_specialhi_status_end)
        .install();*/
}
