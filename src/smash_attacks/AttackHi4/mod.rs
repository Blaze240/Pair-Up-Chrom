use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

unsafe extern "C" fn reflet_attackhi4frame(agent: &mut L2CAgentBase) {
    let slot_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    unsafe {
        if slot_id == 3 {
            if MotionModule::motion_kind(agent.module_accessor) == hash40("attack_hi4_hold") {
                ArticleModule::generate_article(
                    agent.module_accessor,
                    *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
                    false,
                    -1,
                );
                ArticleModule::change_motion(
                    agent.module_accessor,
                    *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
                    Hash40::new("attack_hi4"),
                    false,
                    -1.0,
                );
            }
        }
    }
}

unsafe extern "C" fn reflet_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    WorkModule::is_flag(
        agent.module_accessor,
        *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON,
    );
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("top"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
        );
    }
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 80);
    }
    WorkModule::is_flag(
        agent.module_accessor,
        *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON,
    );
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    } else {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(
                agent.module_accessor,
                Hash40::new("rbkind_nohitl"),
                0,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32,
            );
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 57.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
}

unsafe extern "C" fn chrom_attackhi4(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
    ) as usize;
    let own_boma = sv_battle_object::module_accessor(entry_id as u32);
    if macros::is_excute(agent) {
        if PostureModule::lr(own_boma) < 0.0 {
            LinkModule::set_model_constraint_pos_ort(
                agent.module_accessor,
                *LINK_NO_CONSTRAINT,
                Hash40::new("top"),
                Hash40::new("top"),
                (*CONSTRAINT_FLAG_ORIENTATION
                    | *CONSTRAINT_FLAG_POSITION
                    | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,
                true,
            );
            LinkModule::set_constraint_translate_offset(
                agent.module_accessor,
                &Vector3f {
                    x: -5.0,
                    y: 4.0,
                    z: -7.0,
                },
            );
        } else {
            LinkModule::set_model_constraint_pos_ort(
                agent.module_accessor,
                *LINK_NO_CONSTRAINT,
                Hash40::new("top"),
                Hash40::new("top"),
                (*CONSTRAINT_FLAG_ORIENTATION
                    | *CONSTRAINT_FLAG_POSITION
                    | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,
                true,
            );
            LinkModule::set_constraint_translate_offset(
                agent.module_accessor,
                &Vector3f {
                    x: 5.0,
                    y: 4.0,
                    z: -7.0,
                },
            );
        }
    }
}

unsafe extern "C" fn chrom_effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("finreflet_warp"),
            Hash40::new("top"),
            0.0,
            0.0,
            0.0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_chrom_sword1"),
            Hash40::new("tex_chrom_sword2"),
            9,
            Hash40::new("sword1"),
            0,
            0,
            1.7,
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            12.6,
            true,
            Hash40::new("chrom_sword"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.2,
            0.2,
        );
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }

    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_chrom_sword1"),
            Hash40::new("tex_chrom_sword2"),
            6,
            Hash40::new("sword1"),
            0,
            0,
            1.65,
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            12.4,
            true,
            Hash40::new("chrom_sword"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.2,
            0.2,
        );
    }

    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT(
            agent,
            Hash40::new("finreflet_warp"),
            Hash40::new("top"),
            0.0,
            0.0,
            0.0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
}

pub fn install() {
    Agent::new("reflet")
        .on_line(Main, reflet_attackhi4frame)
        .expression_acmd("expression_attackhi4", reflet_attackhi4, Priority::Low)
        .expression_acmd("expression_attackhi42", reflet_attackhi4, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_attackhi4", chrom_attackhi4, Priority::Low)
        .effect_acmd("effect_attackhi4", chrom_effect_attackhi4, Priority::Low)
        .install();
}
