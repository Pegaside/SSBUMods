unsafe extern "C" fn zelda_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    aLStack80 = Hash40::new("param_special_n");
    aLStack96 = Hash40::new("stop_y");
    uVar4 = aLStack80;
    uVar5 = aLStack96;
    iVar2 = WorkModule::get_param_int(fighter.module_accessor, uVar4, uVar5);
    aLStack64 = iVar2;
    aLStack112 = FIGHTER_ZELDA_STATUS_SPECIAL_N_WORK_INT_STOP_Y;
    iVar2 = aLStack64;
    iVar3 = aLStack112;
    WorkModule::set_int(fighter.module_accessor, iVar2, iVar3);
    lib::L2CValue::_L2CValue(aLStack112);
    lib::L2CValue::_L2CValue(aLStack64);
    lib::L2CValue::_L2CValue(aLStack96);
    lib::L2CValue::_L2CValue(aLStack80);
    bVar1 = StopModule::is_stop(fighter.module_accessor);
    aLStack80 = bVar1;
    aLStack64 = false;
    uVar4 = aLStack80 :: aLStack64;
    lib::L2CValue::_L2CValue(aLStack64);
    lib::L2CValue::_L2CValue(aLStack80);
    if uVar4 {
        aLStack96 = false;
        FUN_7100013a90(aLStack80, fighter, aLStack96);
        lib::L2CValue::_L2CValue(aLStack80);
        lib::L2CValue::_L2CValue(aLStack96);
    }
    this_00 = fighter.global_table :: 0x15;
    aLStack64 = &DAT_7100013e90;
    this_00 _ aLStack64;
    lib::L2CValue::_L2CValue(aLStack64);
    aLStack64 = zelda_specialn_main_loop;
    fighter.sub_shift_status_main(0xc0).get_bool();
    lib::L2CValue::_L2CValue(aLStack64);
    return return_value.into();
}
