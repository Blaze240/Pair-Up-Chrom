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



unsafe extern "C" fn reflet_attackairlw(agent: &mut L2CAgentBase) {
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
            Hash40::new("attack_air_lw"),
            false,
            -1.0,
        );
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("sword"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 13.0);
    execute(agent.lua_state_agent, 13.0);
    WorkModule::is_flag(
        agent.module_accessor,
        *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON,
    );
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    } else {
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
}

unsafe extern "C" fn chrom_attackairlw(agent: &mut L2CAgentBase) {
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
                    z: -5.0,
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
                    x: -5.0,
                    y: 4.0,
                    z: -5.0,
                },
            );
        }
    }
}

unsafe extern "C" fn chrom_effect_attackairlw(agent: &mut L2CAgentBase) {
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
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
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
            true,
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_chrom_sword1"),
            Hash40::new("tex_chrom_sword2"),
            10,
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
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("chrom_sword_light"),
            Hash40::new("sword1"),
            0,
            0,
            11,
            0,
            0,
            0,
            0.4,
            true,
        );
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(
            agent,
            Hash40::new("sys_attack_speedline"),
            Hash40::new("top"),
            1,
            -6.5,
            -1.5,
            -90,
            0,
            0,
            0.85,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
            1,
        );
        macros::EFFECT(
            agent,
            Hash40::new("chrom_attack_refraction"),
            Hash40::new("sword1"),
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
            true,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("sys_attack_impact"),
            Hash40::new("top"),
            0,
            -8,
            0,
            0,
            0,
            0,
            1.5,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_light"), false, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }

    frame(agent.lua_state_agent, 69.0);
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
        .expression_acmd("expression_attackairlw_pairup", reflet_attackairlw, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_attackairlw", chrom_attackairlw, Priority::Low)
        .effect_acmd(
            "effect_attackairlw",
            chrom_effect_attackairlw,
            Priority::Low,
        )
        .install();
}
