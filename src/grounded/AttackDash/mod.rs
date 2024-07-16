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

unsafe extern "C" fn reflet_attackdash(agent: &mut L2CAgentBase) {
    WorkModule::is_flag(
        agent.module_accessor,
        *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON,
    );
    if macros::is_excute(agent) {
        ArticleModule::generate_article(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            false,
            -1,
        );
        ArticleModule::change_motion(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            Hash40::new("attack_dash"),
            false,
            -1.0,
        );
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("top"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 0);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
}

unsafe extern "C" fn chrom_attackdash(agent: &mut L2CAgentBase) {
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
                    z: -10.0,
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
                    z: -10.0,
                },
            );
        }
    }
}

unsafe extern "C" fn chrom_effect_attackdash(agent: &mut L2CAgentBase) {
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
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_chrom_sword1"),
            Hash40::new("tex_chrom_sword2"),
            5,
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
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_dash_smoke"),
            Hash40::new("top"),
            -8,
            0,
            0,
            0,
            0,
            0,
            0.9,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 39.0);
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
    }
}

pub fn install() {
    Agent::new("reflet")
        .expression_acmd("expression_attackdash_pairup", reflet_attackdash, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_attackdash", chrom_attackdash, Priority::Low)
        .effect_acmd("effect_attackdash", chrom_effect_attackdash, Priority::Low)
        .install();
}
