// For importing modules--do not remove, it is important.
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

// Mod functions, START



// Mod functions, END

// Must list all functions named above
pub fn install() {
    Agent::new("kirby") 	// Replace "kirby" with character you want to modify, using their internal code name.
		// List of functions:
		.game_acmd("game_attacklw3", kirby_game_attacklw3, Default)
        .install(); 		// Don't forget this.
}