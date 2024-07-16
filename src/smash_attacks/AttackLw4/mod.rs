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

static mut refletPosX: [f32; 8] = [0.0; 8];
static mut refletPosY: [f32; 8] = [0.0; 8];
static mut refletPosZ: [f32; 8] = [0.0; 8];

unsafe extern "C" fn reflet_attacklw4frame(agent: &mut L2CAgentBase) {
    let slot_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    unsafe {
        if slot_id == 2 {
            if MotionModule::motion_kind(agent.module_accessor) == hash40("attack_lw4_hold") {
                ArticleModule::generate_article(
                    agent.module_accessor,
                    *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
                    false,
                    -1,
                );
                ArticleModule::change_motion(
                    agent.module_accessor,
                    *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
                    Hash40::new("attack_lw4"),
                    false,
                    -1.0,
                );
            }
        }
    }
}

unsafe extern "C" fn reflet_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    WorkModule::is_flag(
        agent.module_accessor,
        *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON,
    );
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("sword"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
    }
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 16.0);
    WorkModule::is_flag(
        agent.module_accessor,
        *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON,
    );
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_impact"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    } else {
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
            ControlModule::set_rumble(
                agent.module_accessor,
                Hash40::new("rbkind_impact"),
                0,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32,
            );
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
}

unsafe extern "C" fn chrom_attacklw4(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
    ) as usize;
    let own_boma = sv_battle_object::module_accessor(entry_id as u32);
    if macros::is_excute(agent) {
        if PostureModule::lr(agent.module_accessor) < 0.0 {
            refletPosX[entry_id] = PostureModule::pos_x(own_boma) + 9.0;
            refletPosY[entry_id] = PostureModule::pos_y(own_boma);
            refletPosZ[entry_id] = PostureModule::pos_z(own_boma) - 2.0;

            PostureModule::set_pos(
                agent.module_accessor,
                &Vector3f {
                    x: refletPosX[entry_id],
                    y: refletPosY[entry_id],
                    z: refletPosZ[entry_id],
                },
            );
        } else {
            refletPosX[entry_id] = PostureModule::pos_x(own_boma) - 9.0;
            refletPosY[entry_id] = PostureModule::pos_y(own_boma);
            refletPosZ[entry_id] = PostureModule::pos_z(own_boma) - 2.0;

            PostureModule::set_pos(
                agent.module_accessor,
                &Vector3f {
                    x: refletPosX[entry_id],
                    y: refletPosY[entry_id],
                    z: refletPosZ[entry_id],
                },
            );
        }
    }
}

unsafe extern "C" fn chrom_effect_attacklw4(agent: &mut L2CAgentBase) {
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
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("chrom_volcano_e"),
            Hash40::new("top"),
            0,
            0,
            17,
            0,
            0,
            0,
            0.93,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.95);
        macros::EFFECT(
            agent,
            Hash40::new("chrom_volcano_f"),
            Hash40::new("top"),
            0,
            0,
            17,
            0,
            0,
            0,
            0.93,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.95);
        macros::EFFECT(
            agent,
            Hash40::new("sys_crown"),
            Hash40::new("top"),
            17,
            0,
            0,
            0,
            0,
            0,
            0.78,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("null"),
            Hash40::new("top"),
            17,
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
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_h_smoke_b"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.6,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 51.0);
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
        .on_line(Main, reflet_attacklw4frame)
        .expression_acmd("expression_attacklw4", reflet_attacklw4, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_attacklw4", chrom_attacklw4, Priority::Low)
        .effect_acmd(
            "effect_attacklw4",
            chrom_effect_attacklw4,
            Priority::Default,
        )
        .install();
}
