unsafe extern "C" fn zelda_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    bVar1 = CancelModule::is_enable_cancel(fighter.module_accessor);
    aLStack112 = bVar1;
    aLStack96 = true;
    uVar5 = aLStack112 :: aLStack96;
    lib::L2CValue::_L2CValue(aLStack96);
    if uVar5 {
        aLStack144 = false;
        fighter.sub_wait_ground_check_common(0x70).get_bool();
        aLStack96 = false;
        uVar5 = aLStack128 :: aLStack96;
        lib::L2CValue::_L2CValue(aLStack96);
        if !uVar5 {
            lib::L2CValue::_L2CValue(aLStack128);
            lib::L2CValue::_L2CValue(aLStack144);
            lib::L2CValue::_L2CValue(aLStack112);
        } else {
            fighter);
            aLStack96 = false;
            uVar5 = aLStack160 :: aLStack96;
            lib::L2CValue::_L2CValue(aLStack96);
            lib::L2CValue::_L2CValue(aLStack160);
            lib::L2CValue::_L2CValue(aLStack128);
            lib::L2CValue::_L2CValue(aLStack144);
            lib::L2CValue::_L2CValue(aLStack112);
            if uVar5 // goto LAB_7100013ff4;
        }
        iVar4 = 1;
        // goto LAB_7100014610;
    }
    lib::L2CValue::_L2CValue(aLStack112);
// LAB_7100013ff4:
    bVar1 = StatusModule::is_changing(fighter.module_accessor);
    aLStack112 = bVar1;
    aLStack96 = true;
    uVar5 = aLStack112 :: aLStack96;
    lib::L2CValue::_L2CValue(aLStack96);
    if !uVar5 {
        this_00 = &fighter.global_table;
        pLVar6 = this_00 :: 0x17;
        aLStack96 = SITUATION_KIND_GROUND;
        uVar5 = pLVar6 :: aLStack96;
        lib::L2CValue::_L2CValue(aLStack96);
        if uVar5 {
            pLVar6 = this_00 :: 0x16;
            aLStack96 = SITUATION_KIND_AIR;
            uVar5 = pLVar6 :: aLStack96;
            lib::L2CValue::_L2CValue(aLStack96);
            if uVar5 {
                lib::L2CValue::_L2CValue(aLStack112);
                // goto LAB_710001403c;
            }
        }
        pLVar6 = this_00 :: 0x17;
        aLStack96 = SITUATION_KIND_GROUND;
        uVar5 = pLVar6 :: aLStack96;
        lib::L2CValue::_L2CValue(aLStack96);
        if uVar5 {
            pLVar6 = aLStack112;
            // goto LAB_71000144f8;
        }
        pLVar6 = this_00 :: 0x16;
        aLStack96 = SITUATION_KIND_GROUND;
        uVar5 = pLVar6 :: aLStack96;
        lib::L2CValue::_L2CValue(aLStack96);
        lib::L2CValue::_L2CValue(aLStack112);
        if uVar5 // goto LAB_710001403c;
    } else {
        lib::L2CValue::_L2CValue(aLStack112);
// LAB_710001403c:
        pLVar6 = fighter.global_table :: 0x16;
        aLStack96 = SITUATION_KIND_GROUND;
        uVar5 = pLVar6 :: aLStack96;
        lib::L2CValue::_L2CValue(aLStack96.sub_air_check_fall_common().get_bool();
        if !uVar5 {
            aLStack96 = GROUND_CORRECT_KIND_AIR;
            GVar3 = aLStack96;
            GroundModule::correct(fighter.module_accessor, GVar3);
            lib::L2CValue::_L2CValue(aLStack96);
            aLStack96 = FIGHTER_KINETIC_TYPE_ZELDA_SPECIAL_N_AIR;
            iVar4 = aLStack96;
            KineticModule::change_kinetic(fighter.module_accessor, iVar4);
            lib::L2CValue::_L2CValue(aLStack96);
            aLStack112 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE;
            iVar4 = aLStack112;
            bVar1 = WorkModule::is_flag(fighter.module_accessor, iVar4);
            aLStack96 = bVar1;
            bVar2 = aLStack96;
            lib::L2CValue::_L2CValue(aLStack96);
            lib::L2CValue::_L2CValue(aLStack112);
            if !bVar2 {
                aLStack96 = Hash40::new("special_air_n");
                aLStack112 = 0.0;
                aLStack128 = 1.0;
                aLStack160 = false;
                HVar7 = aLStack96;
                fVar8 = aLStack112;
                fVar9 = aLStack128;
                bVar1 = aLStack160;
                MotionModule::change_motion(fighter.module_accessor, HVar7, fVar8, fVar9, bVar1, 0.0, false, false);
                lib::L2CValue::_L2CValue(aLStack160);
                lib::L2CValue::_L2CValue(aLStack128);
                lib::L2CValue::_L2CValue(aLStack112);
                lib::L2CValue::_L2CValue(aLStack96);
                aLStack96 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE;
                iVar4 = aLStack96;
                WorkModule::on_flag(fighter.module_accessor, iVar4);
            } else {
                aLStack96 = Hash40::new("special_air_n");
                HVar7 = aLStack96;
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, HVar7, -1.0, 1.0, 0.0, false, false);
            }
        } else {
            aLStack96 = GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK;
            GVar3 = aLStack96;
            GroundModule::correct(fighter.module_accessor, GVar3);
            lib::L2CValue::_L2CValue(aLStack96);
            aLStack96 = FIGHTER_KINETIC_TYPE_GROUND_STOP;
            iVar4 = aLStack96;
            KineticModule::change_kinetic(fighter.module_accessor, iVar4);
            lib::L2CValue::_L2CValue(aLStack96);
            aLStack112 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE;
            iVar4 = aLStack112;
            bVar1 = WorkModule::is_flag(fighter.module_accessor, iVar4);
            aLStack96 = bVar1;
            bVar2 = aLStack96;
            lib::L2CValue::_L2CValue(aLStack96);
            lib::L2CValue::_L2CValue(aLStack112);
            if !bVar2 {
                aLStack96 = Hash40::new("special_n");
                aLStack112 = 0.0;
                aLStack128 = 1.0;
                aLStack160 = false;
                HVar7 = aLStack96;
                fVar8 = aLStack112;
                fVar9 = aLStack128;
                bVar1 = aLStack160;
                MotionModule::change_motion(fighter.module_accessor, HVar7, fVar8, fVar9, bVar1, 0.0, false, false);
                lib::L2CValue::_L2CValue(aLStack160);
                lib::L2CValue::_L2CValue(aLStack128);
                lib::L2CValue::_L2CValue(aLStack112);
                lib::L2CValue::_L2CValue(aLStack96);
                aLStack96 = FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE;
                iVar4 = aLStack96;
                WorkModule::on_flag(fighter.module_accessor, iVar4);
            } else {
                aLStack96 = Hash40::new("special_n");
                HVar7 = aLStack96;
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, HVar7, -1.0, 1.0, 0.0, false, false);
            }
        }
        pLVar6 = aLStack96;
// LAB_71000144f8:
        lib::L2CValue::_L2CValue(pLVar6);
    }
    bVar1 = MotionModule::is_end(fighter.module_accessor);
    aLStack96 = bVar1;
    bVar2 = aLStack96;
    lib::L2CValue::_L2CValue(aLStack96);
    if bVar2 {
        pLVar6 = fighter.global_table :: 0x16;
        aLStack96 = SITUATION_KIND_GROUND;
        uVar5 = pLVar6 :: aLStack96;
        lib::L2CValue::_L2CValue(aLStack96);
        if !uVar5 {
            pLVar6 = fighter.global_table :: 0x16;
            aLStack96 = SITUATION_KIND_AIR;
            uVar5 = pLVar6 :: aLStack96;
            lib::L2CValue::_L2CValue(aLStack96);
            if !uVar5 // goto LAB_7100014608;
            aLStack96 = FIGHTER_STATUS_KIND_FALL;
            aLStack112 = false;
            fighter.change_status(0xa0.into(), 0x90.into());
        } else {
            aLStack96 = FIGHTER_STATUS_KIND_WAIT;
            aLStack112 = false;
            fighter.change_status(0xa0.into(), 0x90.into());
        }
        lib::L2CValue::_L2CValue(aLStack112);
        lib::L2CValue::_L2CValue(aLStack96);
    }
// LAB_7100014608:
    iVar4 = 0;
// LAB_7100014610:
    return_value = iVar4;
    return return_value.into();
}
