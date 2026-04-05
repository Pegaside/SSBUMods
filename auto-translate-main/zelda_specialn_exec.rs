unsafe extern "C" fn zelda_specialn_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    aLStack64 = FIGHTER_ZELDA_REFLECTOR_KIND_REFLECTOR;
    aLStack96 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START;
    iVar3 = aLStack96;
    bVar1 = WorkModule::is_flag(fighter.module_accessor, iVar3);
    aLStack80 = bVar1;
    bVar2 = aLStack80;
    lib::L2CValue::_L2CValue(aLStack80);
    lib::L2CValue::_L2CValue(aLStack96);
    if bVar2 {
        aLStack96 = MA_MSC_SHIELD_SET_STATUS;
        aLStack112 = COLLISION_KIND_REFLECTOR;
        aLStack128 = SHIELD_STATUS_NORMAL;
        aLStack144 = FIGHTER_REFLECTOR_GROUP_EXTEND;
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack96));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack112));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack64));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack128));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack144));
        sv_module_access::shield(fighter.lua_state_agent);
        lib::L2CAgent::pop_lua_stack(fighter, 1);
        lib::L2CValue::_L2CValue(aLStack80);
        lib::L2CValue::_L2CValue(aLStack144);
        lib::L2CValue::_L2CValue(aLStack128);
        lib::L2CValue::_L2CValue(aLStack112);
        lib::L2CValue::_L2CValue(aLStack96);
        aLStack96 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START;
        iVar3 = aLStack96;
        WorkModule::off_flag(fighter.module_accessor, iVar3);
        lib::L2CValue::_L2CValue(aLStack96);
    }
    aLStack112 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END;
    iVar3 = aLStack112;
    bVar1 = WorkModule::is_flag(fighter.module_accessor, iVar3);
    aLStack96 = bVar1;
    bVar2 = aLStack96;
    lib::L2CValue::_L2CValue(aLStack96);
    lib::L2CValue::_L2CValue(aLStack112);
    if bVar2 {
        aLStack112 = MA_MSC_SHIELD_SET_STATUS;
        aLStack128 = COLLISION_KIND_REFLECTOR;
        aLStack144 = SHIELD_STATUS_NONE;
        aLStack160 = FIGHTER_REFLECTOR_GROUP_EXTEND;
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack112));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack128));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack64));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack144));
        fighter.push_lua_stack(&mut L2CValue::new_num(aLStack160));
        sv_module_access::shield(fighter.lua_state_agent);
        lib::L2CAgent::pop_lua_stack(fighter, 1);
        lib::L2CValue::_L2CValue(aLStack96);
        lib::L2CValue::_L2CValue(aLStack160);
        lib::L2CValue::_L2CValue(aLStack144);
        lib::L2CValue::_L2CValue(aLStack128);
        lib::L2CValue::_L2CValue(aLStack112);
        aLStack112 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END;
        iVar3 = aLStack112;
        WorkModule::off_flag(fighter.module_accessor, iVar3);
        lib::L2CValue::_L2CValue(aLStack112);
    }
    return_value = 0;
    lib::L2CValue::_L2CValue(aLStack64);
    return return_value.into();
}
