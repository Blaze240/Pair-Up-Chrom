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

unsafe extern "C" fn reflet_appeals(agent: &mut L2CAgentBase) {
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
            Hash40::new("appeal_s_l"),
            false,
            -1.0,
        );
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
}

unsafe extern "C" fn chrom_appeals(agent: &mut L2CAgentBase) {
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
                    y: 0.0,
                    z: 10.0,
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
                    y: 0.0,
                    z: 10.0,
                },
            );
        }
    }
}

unsafe extern "C" fn chrom_effect_appeals(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("reflet_entry"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            270,
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
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_dash_smoke"),
            Hash40::new("top"),
            -4,
            0,
            -2,
            0,
            30,
            0,
            0.7,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("sword1"),
            0,
            0,
            10,
            0,
            0,
            0,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 49.0);
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
        .game_acmd("game_appealsl_pairup", reflet_appeals, Priority::Low)
        .game_acmd("game_appealsr_pairup", reflet_appeals, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_appealsl", chrom_appeals, Priority::Low)
        .effect_acmd("effect_appealsl", chrom_effect_appeals, Priority::Low)
        .install();
}
