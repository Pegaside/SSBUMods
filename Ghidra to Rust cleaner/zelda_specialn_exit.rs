unsafe extern "C" fn zelda_specialn_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    aLStack80 = MA_MSC_SHIELD_SET_STATUS;
    aLStack96 = COLLISION_KIND_REFLECTOR;
    aLStack112 = FIGHTER_ZELDA_REFLECTOR_KIND_REFLECTOR;
    aLStack128 = SHIELD_STATUS_NORMAL;
    aLStack144 = FIGHTER_REFLECTOR_GROUP_EXTEND;
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_num(aLStack80));
    fighter.push_lua_stack(&mut L2CValue::new_num(aLStack96));
    fighter.push_lua_stack(&mut L2CValue::new_num(aLStack112));
    fighter.push_lua_stack(&mut L2CValue::new_num(aLStack128));
    fighter.push_lua_stack(&mut L2CValue::new_num(aLStack144));
    sv_module_access::shield(fighter.lua_state_agent);
    lib::L2CAgent::pop_lua_stack(fighter, 1);
    lib::L2CValue::_L2CValue(aLStack64);
    lib::L2CValue::_L2CValue(aLStack144);
    lib::L2CValue::_L2CValue(aLStack128);
    lib::L2CValue::_L2CValue(aLStack112);
    lib::L2CValue::_L2CValue(aLStack96);
    lib::L2CValue::_L2CValue(aLStack80);
    return_value = 0;
    return return_value.into();
}
