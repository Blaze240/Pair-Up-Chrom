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

unsafe extern "C" fn reflet_attackhi4frame(agent: &mut L2CAgentBase) {
    unsafe {
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

unsafe extern "C" fn chrom_attackhi4(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
    ) as usize;
    let own_boma = sv_battle_object::module_accessor(entry_id as u32);
    if macros::is_excute(agent) {
        if PostureModule::lr(agent.module_accessor) < 0.0 {
            refletPosX[entry_id] = PostureModule::pos_x(own_boma) + 2.0;
            refletPosY[entry_id] = PostureModule::pos_y(own_boma) + 4.0;
            refletPosZ[entry_id] = PostureModule::pos_z(own_boma) - 4.0;

            PostureModule::set_pos(
                agent.module_accessor,
                &Vector3f {
                    x: refletPosX[entry_id],
                    y: refletPosY[entry_id],
                    z: refletPosZ[entry_id],
                },
            );
        } else {
            refletPosX[entry_id] = PostureModule::pos_x(own_boma) - 2.0;
            refletPosY[entry_id] = PostureModule::pos_y(own_boma) + 4.0;
            refletPosZ[entry_id] = PostureModule::pos_z(own_boma) - 4.0;

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

    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }

    frame(agent.lua_state_agent, 74.0);
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
        .on_line(Main, reflet_attackhi4frame)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_attackhi4", chrom_attackhi4, Priority::Low)
        .effect_acmd("effect_attackhi4", chrom_effect_attackhi4, Priority::Low)
        .install();
}
