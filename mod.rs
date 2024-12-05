use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash::app::lua_bind::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_battle_object::module_accessor;
use smash::app::{self, sv_information};
use smash::app::*;
use smash::app::sv_animcmd::frame;
use smash::app::utility::get_category;
use smash::app::sv_battle_object::entry_id;
use smash::app::lua_bind::ModelModule::set_mesh_visibility;
use smash_script::macros::*;
use smash::lib::L2CAgent;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CWeaponCommon;
use smash_script::*;

use crate::FIGHTER_CUTIN_MANAGER;

/*
HELPER
*/
pub static mut FIGHTER_NAME: [u64;9] = [0;9];
static mut NAME_CHECK: [bool;9] = [false;9];

pub unsafe fn read_tag(addr: u64) -> String {
    let mut s: Vec<u8> = vec![];

    let mut addr = addr as *const u16;
    loop {
        if *addr == 0_u16 {
            break;
        }
        s.push(*(addr as *const u8));
        addr = addr.offset(1);
    }

    std::str::from_utf8(&s).unwrap().to_owned()
}

pub unsafe fn get_player_number(module_accessor:  &mut app::BattleObjectModuleAccessor) -> usize {
    let player_number;
    if app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    return player_number;
}

pub unsafe fn is_uniq_echo(module_accessor: &mut app::BattleObjectModuleAccessor, fighter_kind: i32) -> bool {
    if fighter_kind == *FIGHTER_KIND_DUCKHUNT && FIGHTER_NAME[get_player_number(module_accessor)] == hash40("GLACEON & FROSMOTH") {
        return true;
    }
    else {
        return false;
    }
}

/*
ACMD
*/
#[acmd_script( agent = "duckhunt", script = "game_specialnblank", category = ACMD_GAME )]
unsafe fn game_specialnblank(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "duckhunt", script = "game_specialairnblank", category = ACMD_GAME )]
unsafe fn game_specialairnblank(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "duckhunt", script = "sound_specialnblank", category = ACMD_SOUND )]
unsafe fn sound_specialnblank(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "duckhunt", script = "sound_specialairnblank", category = ACMD_SOUND )]
unsafe fn sound_specialairnblank(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "duckhunt", script = "effect_specialnblank", category = ACMD_EFFECT )]
unsafe fn effect_specialnblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_specialairnblank", category = ACMD_EFFECT )]
unsafe fn effect_specialairnblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "duckhunt", script = "expression_specialnblank", category = ACMD_EXPRESSION )]
unsafe fn expression_specialnblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "duckhunt", script = "expression_specialairnblank", category = ACMD_EXPRESSION )]
unsafe fn expression_specialairnblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "duckhunt", script = "game_specialsblank", category = ACMD_GAME )]
unsafe fn game_specialsblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.06);
}

#[acmd_script( agent = "duckhunt", script = "game_specialairsblank", category = ACMD_GAME )]
unsafe fn game_specialairsblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.06);
}

#[acmd_script( agent = "duckhunt", script = "sound_specialsblank", category = ACMD_SOUND )]
unsafe fn sound_specialsblank(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "duckhunt", script = "sound_specialairsblank", category = ACMD_SOUND )]
unsafe fn sound_specialairsblank(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "duckhunt", script = "effect_specialsblank", category = ACMD_EFFECT )]
unsafe fn effect_specialsblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("throw"), 0, 6, 12, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_specialairsblank", category = ACMD_EFFECT )]
unsafe fn effect_specialairsblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("throw"), 0, 6, 12, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "duckhunt", script = "expression_specialsblank", category = ACMD_EXPRESSION )]
unsafe fn expression_specialsblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        //ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

#[acmd_script( agent = "duckhunt", script = "expression_specialairsblank", category = ACMD_EXPRESSION )]
unsafe fn expression_specialairsblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        //ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

/*
STATUS
*/
pub unsafe fn special_motion_helper(fighter: &mut L2CFighterCommon,init: bool) {
    let mot_g =  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    let mot = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {Hash40::new_raw(mot_g)} else {Hash40::new_raw(mot_a)};

    if init {
        MotionModule::change_motion(fighter.module_accessor, mot,0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, mot, -1.0, 1.0, 0.0, false, false);
    }
}

#[smashline::status_script(agent = "duckhunt", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let fighter_kind = app::utility::get_kind(&mut *fighter.module_accessor);
    let boma = module_accessor as *mut app::BattleObjectModuleAccessor as u64;
    if NAME_CHECK[get_player_number(&mut *fighter.module_accessor)] == false {
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let name_base = text + 0x52c3758;
        FIGHTER_NAME[get_player_number(&mut *fighter.module_accessor)] = hash40(&read_tag(name_base + 0x260 * get_player_number(&mut *fighter.module_accessor) as u64 + 0x8e));
        NAME_CHECK[get_player_number(&mut *fighter.module_accessor)] = true;
    }
    let should_return_original = !is_uniq_echo(&mut *fighter.module_accessor, fighter_kind) || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN);
    if should_return_original {
        let to_return = original!(fighter);
        if is_uniq_echo(&mut *fighter.module_accessor, fighter_kind) {
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        }
        return to_return;
    }

    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    special_motion_helper(fighter,true);
    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    
    return fighter.sub_shift_status_main(L2CValue::Ptr(specialn_main_loop as *const () as _));
}

unsafe fn specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        special_motion_helper(fighter,false);
    }

    0.into()
} 

#[smashline::status_script(agent = "duckhunt", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let fighter_kind = app::utility::get_kind(&mut *fighter.module_accessor);
    let boma = module_accessor as *mut app::BattleObjectModuleAccessor as u64;
    if NAME_CHECK[get_player_number(&mut *fighter.module_accessor)] == false {
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let name_base = text + 0x52c3758;
        FIGHTER_NAME[get_player_number(&mut *fighter.module_accessor)] = hash40(&read_tag(name_base + 0x260 * get_player_number(&mut *fighter.module_accessor) as u64 + 0x8e));
        NAME_CHECK[get_player_number(&mut *fighter.module_accessor)] = true;
    }
    let should_return_original = !is_uniq_echo(&mut *fighter.module_accessor, fighter_kind) || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY);
    if should_return_original {
        let to_return = original!(fighter);
        if is_uniq_echo(&mut *fighter.module_accessor, fighter_kind) {
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        }
        return to_return;
    }

    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_blank") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    special_motion_helper(fighter,true);

    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    
    return fighter.sub_shift_status_main(L2CValue::Ptr(specialn_main_loop as *const () as _));
}

#[smashline::status_script(agent = "duckhunt_gunman", status = WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let end_frame = MotionModule::end_frame(weapon.module_accessor) as i32;
    WorkModule::set_int(weapon.module_accessor, end_frame, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, end_frame, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
    return original!(weapon);
}

pub fn install() {
    install_acmd_scripts!(
        game_specialnblank,
        game_specialairnblank,
        sound_specialnblank,
        sound_specialairnblank,
        effect_specialnblank,
        effect_specialairnblank,
        expression_specialnblank,
        expression_specialairnblank,
        game_specialsblank,
        game_specialairsblank,
        sound_specialsblank,
        sound_specialairsblank,
        effect_specialsblank,
        effect_specialairsblank,
        expression_specialsblank,
        expression_specialairsblank
    );
    install_status_script!(specialn_main);
    install_status_script!(specials_main);
    install_status_script!(shoot_main);
}