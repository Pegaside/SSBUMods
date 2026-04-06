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

// STATUS Init
unsafe extern "C" fn example_status_script(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

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
